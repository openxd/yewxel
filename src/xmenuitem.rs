use js_sys::{Math::max, Object};
use prokio::{spawn_local, time::sleep};
use serde_wasm_bindgen::to_value;
use std::{
    collections::HashMap,
    fmt::Write,
    time::{Duration, Instant},
};
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{window, Element, FocusEvent, HtmlElement, MouseEvent, PointerEvent};
use yew::{html, Callback, Children, Component, ContextHandle, NodeRef, Properties};

use crate::{
    calculate_computed_size, utils::new_animation, xcontainer::XContainerContext, XComponentSize,
};

#[derive(PartialEq, Properties)]
pub struct XMenuItemProps {
    #[prop_or_default]
    pub size: Option<XComponentSize>,
    #[prop_or_default]
    pub togglable: bool,
    /// Whether this item is disabled or not
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub toggled: bool,
    /// Value associated with this menu item (usually the command name)
    #[prop_or_default]
    pub value: Option<String>,
    #[prop_or_default]
    pub children: Children,
    /// This callback is firing on the click event when component is togglable
    #[prop_or_default]
    pub ontoggle: Option<Callback<MouseEvent>>,
    /// This callback is firing on the click event even when the component is togglable
    #[prop_or_default]
    pub onclick: Option<Callback<MouseEvent>>,
    // Callback when any trigger effects (such ripples or blinking) are finished
    #[prop_or_default]
    pub on_trigger_end: Callback<()>,
    #[prop_or_default]
    pub trigger_effect: XMenuItemTriggerEffect,
}

#[derive(PartialEq)]
enum XMenuItemRippleAnimationStatus {
    Created,
    Started,
    Finished,
}

struct XMenuItemRipple {
    node_ref: NodeRef,
    size: f64,
    top: f64,
    left: f64,
    in_animation: XMenuItemRippleAnimationStatus,
    out_animation: XMenuItemRippleAnimationStatus,
}

pub struct XMenuItem {
    focused: bool,
    _container_ctx_listner: ContextHandle<XContainerContext>,
    container_ctx: XContainerContext,
    ripples_ref: NodeRef,
    root_ref: NodeRef,
    pointer_down: Option<(Instant, PointerEvent)>,
    ripples: Vec<XMenuItemRipple>,
    prev_ripples_count: u8,
    pressed: bool,
}

#[derive(PartialEq)]
pub enum XMenuItemTriggerEffect {
    Ripple,
    Blink,
    None,
}

impl Default for XMenuItemTriggerEffect {
    fn default() -> Self {
        XMenuItemTriggerEffect::Blink
    }
}

pub enum XMenuItemChild {}

pub enum XMenuItemMessage {
    Focus(FocusEvent),
    Blur(FocusEvent),
    ContainerUpdated(XContainerContext),
    PointerDown(PointerEvent),
    PointerUp(PointerEvent),
    ResetPressed,
    RippleCreated(u8),
    RippleInAnimationFinished(u8),
    RippleOutAnimationFinished(u8),
}

impl XMenuItem {
    fn start_ripple_out_animation(&mut self, link: yew::html::Scope<Self>, i: usize) {
        if let Some(ripple) = self.ripples.get_mut(i) {
            let element = ripple.node_ref.clone().cast::<Element>().unwrap();
            let opacity = if let Some(computed_style) =
                window().unwrap().get_computed_style(&element).unwrap()
            {
                computed_style
                    .get_property_value("opacity")
                    .unwrap_or(String::from("1"))
            } else {
                String::from("1")
            };

            ripple.out_animation = XMenuItemRippleAnimationStatus::Started;
            let mut keyframes = HashMap::new();
            keyframes.insert("opacity", [&opacity, "0"]);
            let animation = new_animation(
                &element,
                &Object::try_from(&to_value(&keyframes).unwrap()).unwrap(),
                300.0,
                &crate::CSSEasing::CubicBezier(0.4, 0.0, 0.2, 1.0),
            );

            spawn_local(async move {
                JsFuture::from(animation.finished().unwrap()).await.unwrap();
                link.send_message(XMenuItemMessage::RippleOutAnimationFinished(i as u8));
            });
        }
    }
}

impl Component for XMenuItem {
    type Message = XMenuItemMessage;
    type Properties = XMenuItemProps;

    fn create(ctx: &yew::Context<Self>) -> Self {
        let (container_ctx, _container_ctx_listner) = ctx
            .link()
            .context(ctx.link().callback(XMenuItemMessage::ContainerUpdated))
            .expect("Container not found");

        XMenuItem {
            focused: false,
            _container_ctx_listner,
            container_ctx,
            root_ref: NodeRef::default(),
            ripples_ref: NodeRef::default(),
            pointer_down: None,
            ripples: Vec::new(),
            prev_ripples_count: 0,
            pressed: false,
        }
    }

    fn update(&mut self, ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
        match msg {
            XMenuItemMessage::ContainerUpdated(container_ctx) => {
                self.container_ctx = container_ctx;
            }
            XMenuItemMessage::Focus(_) => {
                self.focused = true;
            }
            XMenuItemMessage::Blur(_) => {
                self.focused = false;
            }
            XMenuItemMessage::ResetPressed => {
                self.pressed = false;
            }
            XMenuItemMessage::PointerDown(e) => {
                let mouse_event = e.clone().dyn_into::<MouseEvent>().unwrap();
                if mouse_event.buttons() > 1 {
                    return false;
                }

                // TODO: Check if a x-menuitem exist in closing menu

                let closest_item = e.target().unwrap().dyn_into::<Element>().unwrap().closest(".x-menuitem").unwrap();
                let root_element = self.root_ref.cast::<Element>().unwrap();
                if let Some(closest_item) = closest_item {
                    if closest_item != root_element {
                        return false;
                    }
                } else {
                    return false;
                }

                let root_element = self.root_ref.cast::<HtmlElement>().unwrap();
                root_element.set_pointer_capture(e.pointer_id()).unwrap();
                self.pointer_down = Some((Instant::now(), e.clone()));
                self.pressed = true;

                // TODO: If do not have a XMenu as a child
                match ctx.props().trigger_effect {
                    XMenuItemTriggerEffect::Ripple => {
                        let ripples_element = self.ripples_ref.cast::<HtmlElement>().unwrap();
                        let bounding_box = ripples_element.get_bounding_client_rect();
                        let size = max(bounding_box.width(), bounding_box.height()) * 1.5;
                        let top = e.client_y() as f64 - bounding_box.y() - size / 2.0;
                        let left = e.client_x() as f64 - bounding_box.x() - size / 2.0;
                        self.ripples.push(XMenuItemRipple {
                            node_ref: NodeRef::default(),
                            size,
                            top,
                            left,
                            in_animation: XMenuItemRippleAnimationStatus::Created,
                            out_animation: XMenuItemRippleAnimationStatus::Created,
                        });
                    }
                    _ => {}
                }
            }
            XMenuItemMessage::PointerUp(_) => {
                if let Some(pointer_down) = self.pointer_down.clone() {
                    self.pointer_down = None;

                    let found_ripple = self.ripples.iter().position(|r| {
                        r.in_animation == XMenuItemRippleAnimationStatus::Finished
                            && r.out_animation == XMenuItemRippleAnimationStatus::Created
                    });
                    if let Some(in_ripple) = found_ripple {
                        self.start_ripple_out_animation(ctx.link().clone(), in_ripple);
                    }

                    let link = ctx.link().clone();
                    spawn_local(async move {
                        let pressed_time = Instant::now() - pointer_down.0;
                        let min_pressed_time = if pointer_down.1.pointer_type() == "touch" {
                            600
                        } else {
                            150
                        } as u128;

                        if pressed_time.as_millis() < min_pressed_time {
                            sleep(Duration::from_millis(
                                (min_pressed_time - pressed_time.as_millis()) as u64,
                            ))
                            .await;
                        }

                        link.send_message(XMenuItemMessage::ResetPressed)
                    })
                }
            }
            XMenuItemMessage::RippleCreated(i) => {
                let ripple = self.ripples.get_mut(i as usize);
                if let Some(ripple) = ripple {
                    let ripple_element = ripple.node_ref.cast::<Element>().unwrap();
                    let mut keyframes = HashMap::new();
                    keyframes.insert("transform", ["scale3d(0, 0, 0)", "none"]);
                    let animation = new_animation(
                        &ripple_element,
                        &Object::try_from(&to_value(&keyframes).unwrap()).unwrap(),
                        300.0,
                        &crate::CSSEasing::CubicBezier(0.4, 0.0, 0.2, 1.0),
                    );
                    ripple.in_animation = XMenuItemRippleAnimationStatus::Started;
                    let link = ctx.link().clone();
                    spawn_local(async move {
                        JsFuture::from(animation.finished().unwrap()).await.unwrap();
                        link.send_message(XMenuItemMessage::RippleInAnimationFinished(i));
                    });
                }
            }
            XMenuItemMessage::RippleInAnimationFinished(i) => {
                if let Some(ripple) = self.ripples.get_mut(i as usize) {
                    ripple.in_animation = XMenuItemRippleAnimationStatus::Finished;
                    if let None = self.pointer_down {
                        self.start_ripple_out_animation(ctx.link().clone(), i as usize);
                    }
                }
            },
            XMenuItemMessage::RippleOutAnimationFinished(i) => {
                self.ripples.remove(i as usize);

                if self.ripples.len() == 0 {
                    ctx.props().on_trigger_end.emit(());
                }
            }
        }
        true
    }

    fn rendered(&mut self, ctx: &yew::Context<Self>, _first_render: bool) {
        if self.prev_ripples_count as usize > self.ripples.len() {
            let link = ctx.link().clone();

            if let Some(pointer_down) = self.pointer_down.clone() {
                let root_element = self.root_ref.cast::<HtmlElement>().unwrap();
                root_element
                    .set_pointer_capture(pointer_down.1.pointer_id())
                    .unwrap();
            }

            for i in self.prev_ripples_count..self.ripples.len() as u8 {
                link.send_message(XMenuItemMessage::RippleCreated(i));
            }
            self.prev_ripples_count = self.ripples.len() as u8;
        }
    }

    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        let props = ctx.props();
        let mut classes = String::from("x-menuitem");

        let computed_size =
            calculate_computed_size(props.size.clone(), self.container_ctx.size.clone());
        write!(classes, " computedsize-{}", computed_size.to_string()).unwrap();

        if props.togglable {
            classes.write_str(" togglable").unwrap();
        }

        if props.toggled {
            classes.write_str(" toggled").unwrap();
        }

        if self.pointer_down.is_some() {
            classes.write_str(" pressed").unwrap();
        }

        let onfocus = ctx.link().callback(XMenuItemMessage::Focus);
        let onblur = ctx.link().callback(XMenuItemMessage::Blur);
        let onpointerup = ctx.link().callback(XMenuItemMessage::PointerUp);
        let onpointerdown = ctx.link().callback(XMenuItemMessage::PointerDown);

        html! {
          <div
            aria-disabled={if props.disabled {"disabled"} else {""}}
            {onblur}
            {onfocus}
            {onpointerup}
            {onpointerdown}
            tabindex={if props.disabled {"-1"} else {"1"}}
            role="menuitem"
            ref={self.root_ref.clone()}
            class={classes.clone()}>
            <div ref={self.ripples_ref.clone()} class="ripples">

            </div>
            <svg class="checkmark" viewBox="0 0 100 100" preserveAspectRatio="none">
              <path></path>
            </svg>
            {for props.children.iter()}
            <svg class="arrow" viewBox="0 0 100 100" hidden={true}>
              <path></path>
            </svg>
          </div>
        }
    }
}
