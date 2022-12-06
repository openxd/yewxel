use std::fmt::Write;
use web_sys::{FocusEvent, KeyboardEvent, MouseEvent};
use yew::{html, Callback, Children, Component, ContextHandle, Properties};

use crate::{calculate_computed_size, xcontainer::XContainerContext, XComponentSize};

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
}

pub struct XMenuItem {
    focused: bool,
    _container_ctx_listner: ContextHandle<XContainerContext>,
    container_ctx: XContainerContext,
}

pub enum XMenuItemChild {}

pub enum XMenuItemMessage {
    Focus(FocusEvent),
    Blur(FocusEvent),
    ContainerUpdated(XContainerContext),
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
        }
    }

    fn update(&mut self, _ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
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
        }
        true
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

        let onfocus = {
            let ctx = ctx.link().clone();
            Callback::from(move |e: FocusEvent| {
                ctx.send_message(XMenuItemMessage::Focus(e));
            })
        };

        let onblur = {
            let ctx = ctx.link().clone();
            Callback::from(move |e: FocusEvent| {
                ctx.send_message(XMenuItemMessage::Blur(e));
            })
        };

        html! {
          <div
            aria-disabled={if props.disabled {"disabled"} else {""}}
            onblur={onblur}
            onfocus={onfocus}
            tabindex={if props.disabled {"-1"} else {"1"}}
            role="menuitem"
            class={classes.clone()}>
            <div class="ripples"></div>
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
