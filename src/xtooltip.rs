use web_sys::{HtmlElement, DomRect};
use yew::{Properties, Children, Component, html, TargetCast};
use std::fmt::Write;

use crate::console_log;

#[derive(PartialEq)]
pub enum XTooltipType {
    Hint,
    Error
}

impl Default for XTooltipType {
    fn default() -> Self {
        XTooltipType::Hint
    }
}

impl ToString for XTooltipType {
    fn to_string(&self) -> String {
        match self {
            XTooltipType::Hint => String::from("hint"),
            XTooltipType::Error => String::from("error")
        }
    }
}

#[derive(PartialEq)]
pub enum XTooltipAlign {
    Left,
    Right,
    Top,
    Bottom
}

impl Default for XTooltipAlign {
    fn default() -> Self {
        XTooltipAlign::Bottom
    }
}

impl ToString for XTooltipAlign {
    fn to_string(&self) -> String {
        match self {
            XTooltipAlign::Left => String::from("left"),
            XTooltipAlign::Right => String::from("right"),
            XTooltipAlign::Top => String::from("top"),
            XTooltipAlign::Bottom => String::from("bottom")
        }
    }
}

#[derive(PartialEq, Properties)]
pub struct XTooltipProps {
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub r#type: XTooltipType,
    #[prop_or_default]
    pub align: XTooltipAlign,
    #[prop_or_default]
    pub children: Children,
}

pub enum XTooltipMessage {
    #[cfg(feature="element-x-button")]
    XButtonContextUpdated(crate::xbutton::XButtonContext),
}

pub struct XTooltip {
    position: Option<DomRect>,
    open: bool,
    #[cfg(feature="element-x-button")]
    x_button_context_listener: Option<yew::context::ContextHandle<crate::xbutton::XButtonContext>>
}

impl Component for XTooltip {
    type Message = XTooltipMessage;
    type Properties = XTooltipProps;

    #[cfg(not(feature="element-x-button"))]
    fn create(_ctx: &yew::Context<Self>) -> Self {
        XTooltip {open: false, position: None}
    }

    #[cfg(feature="element-x-button")]
    fn create(ctx: &yew::Context<Self>) -> Self {
        if let Some((_message, context_listener)) = ctx.link().context(
            ctx.link()
                .callback(XTooltipMessage::XButtonContextUpdated)) {
            XTooltip { x_button_context_listener: Some(context_listener), open: false, position: None}
        } else {
            XTooltip { x_button_context_listener: None, open: false, position: None}
        }
    }

    #[cfg(feature="element-x-button")]
    fn update(&mut self, _ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
        use crate::xbutton::XButtonEvent;
        match msg {
            XTooltipMessage::XButtonContextUpdated(message) => {
                if let Some(event) = &message.event {
                    match event {
                        XButtonEvent::MouseEnter(e) => {
                            self.open = true;
                            if self.position.is_none() {
                                if let Some(element) = e.target_dyn_into::<HtmlElement>() {
                                    self.position = Some(element.get_bounding_client_rect());
                                }
                            }
                            true
                        },
                        XButtonEvent::MouseLeave(_e) => {
                            self.open = false;
                            true
                        },
                        _=>{false}
                    }
                } else {
                    false
                }
            }
        }
    }

    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        let props = ctx.props();
        let mut classes = String::from("x-tooltip");

        if props.disabled {
            classes.push_str(" disabled");
        }

        console_log(format!("Opened: {:?}", self.open));

        write!(&mut classes, " type-{}", props.r#type.to_string()).unwrap();
        write!(&mut classes, " align-{}", props.align.to_string()).unwrap();

        html!{
            <div class={classes}>
                <div class="x-tooltip-contents">
                    {for props.children.iter()}
                </div>
            </div>
        }
    }
}
