//! `XTooltip` component will allow you to use tooltips on buttons. We are recommending to use the
//! `XTooltip` with a `XButton` element. Otherwise you have to define the css positin by own.
//!
//! Simple:-
//! ```
//! <XButton>
//!     <XLabel>My Button</XLabel>
//!     <XTooltip>
//!         <XLabel>{"My Tooltip"}</XTooltip>
//!     </XTooltip>
//! </XButton>
//!
//! Alignment:-
//! ```
//!
//! Alignment:-
//! ```
//! <XButton>
//!     <XLabel>My Button</XLabel>
//!     <XTooltip align={XTooltipAlign::Right}>
//!         <XLabel>{"My Tooltip"}</XTooltip>
//!     </XTooltip>
//! </XButton>
//! ```
//!
//! > Enable `"element-x-tooltip"` feature to use this component.

// TODO: Implement animation with Element.animate method once
// https://github.com/rustwasm/wasm-bindgen/pull/3142 merged
// TODO: Update position after resize the window
use std::fmt::Write;
use yew::{html, Children, Component, NodeRef, Properties};

use crate::{CSSEasing, Transition};

#[cfg(feature = "element-x-button")]
const WINDOW_WHITESPACE: f64 = 8.0;
#[cfg(feature = "element-x-button")]
const SPACE_BETWEEN: f64 = 8.0;

/// Type of the XTooltip component
#[derive(PartialEq)]
pub enum XTooltipType {
    Hint,
    Error,
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
            XTooltipType::Error => String::from("error"),
        }
    }
}

/// Side to display the XTooltip
#[derive(PartialEq, Clone)]
pub enum XTooltipAlign {
    Left,
    Right,
    Top,
    Bottom,
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
            XTooltipAlign::Bottom => String::from("bottom"),
        }
    }
}

/// Properties to XTooltip component.
///
/// > NOTE: If you are using the XTooltip without `"element-x-button"` feature, it is mandatory
/// > to define position in `style` prop.
#[derive(PartialEq, Properties)]
pub struct XTooltipProps {
    /// Whether the tooltip is opened or not.
    ///
    /// > NOTE: Please implement on_open and on_close callbacks
    /// > when handling the open state manually. Otherwise the tooltip will not open with x-button
    /// > mouseenter event.
    pub open: Option<bool>,
    /// Whether the tooltip is disabled or not
    #[prop_or_default]
    pub disabled: bool,
    /// Type of the tooltip
    #[prop_or_default]
    pub r#type: XTooltipType,
    /// In which side the popup should be displayed.
    ///
    /// > NOTE: This behaviour will be ignored if it
    /// > overflowing from the screen. Default value is "bottom"
    #[cfg(feature = "element-x-button")]
    #[prop_or_default]
    pub align: XTooltipAlign,
    /// Inner contents to display in tooltip
    #[prop_or_default]
    pub children: Children,
    /// When opening with x-button mouseenter event
    #[cfg(feature = "element-x-button")]
    #[prop_or_default]
    pub on_open: Option<yew::Callback<()>>,
    /// When closing with x-button mouseleave event
    #[cfg(feature = "element-x-button")]
    #[prop_or_default]
    pub on_close: Option<yew::Callback<()>>,
    /// Styles to apply for root element of tooltip
    #[prop_or_default]
    pub style: Option<String>,
    /// Classes to apply for root element of tooltip
    #[prop_or_default]
    pub class: Option<String>,
    /// Animation to use when opening the tooltip
    #[prop_or(Transition::new("property", 0.0, CSSEasing::CubicBezier(0.4, 0.0, 0.2, 1.0)))]
    pub open_transition: Transition,
    /// Animation to use when closing the tooltip
    #[prop_or(Transition::new("property", 0.0, CSSEasing::CubicBezier(0.4, 0.0, 0.2, 1.0)))]
    pub close_transition: Transition,
}

#[doc(hidden)]
pub enum XTooltipMessage {
    #[cfg(feature = "element-x-button")]
    XButtonContextUpdated(crate::xbutton::XButtonContext),
    #[cfg(feature = "element-x-button")]
    XButtonCloseAnimationEnd,
}

/// XTooltip Element
pub struct XTooltip {
    #[cfg(feature = "element-x-button")]
    position: Option<web_sys::DomRect>,
    #[cfg(feature = "element-x-button")]
    open: bool,
    #[cfg(feature = "element-x-button")]
    node_ref: NodeRef,
    #[cfg(feature = "element-x-button")]
    _x_button_context_listener: Option<yew::context::ContextHandle<crate::xbutton::XButtonContext>>,
}

impl Component for XTooltip {
    type Message = XTooltipMessage;
    type Properties = XTooltipProps;

    #[cfg(not(feature = "element-x-button"))]
    fn create(_ctx: &yew::Context<Self>) -> Self {
        XTooltip {}
    }

    #[cfg(feature = "element-x-button")]
    fn create(ctx: &yew::Context<Self>) -> Self {
        if let Some((_message, context_listener)) = ctx
            .link()
            .context(ctx.link().callback(XTooltipMessage::XButtonContextUpdated))
        {
            XTooltip {
                _x_button_context_listener: Some(context_listener),
                open: false,
                position: None,
                node_ref: NodeRef::default(),
            }
        } else {
            XTooltip {
                _x_button_context_listener: None,
                open: false,
                position: None,
                node_ref: NodeRef::default(),
            }
        }
    }

    #[cfg(feature = "element-x-button")]
    fn update(&mut self, ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
        use std::collections::HashMap;
        use wasm_bindgen_futures::JsFuture;
        use yew::TargetCast;

        use js_sys::Object;
        use web_sys::{window, Element};

        use crate::xbutton::XButtonEvent;

        let align = ctx.props().align.clone();
        let style = ctx.props().style.clone().unwrap_or(String::from(""));

        match msg {
            XTooltipMessage::XButtonContextUpdated(message) => {
                if let Some(event) = &message.event {
                    match event {
                        XButtonEvent::MouseEnter(e) => {
                            let current_element =
                                self.node_ref.cast::<web_sys::HtmlElement>().unwrap();
                            if !self.open {
                                let props = ctx.props();
                                if props.on_open.is_some() {
                                    props.on_open.clone().unwrap().emit(());
                                }

                                if props.close_transition.property == "opacity" {
                                    let mut keyframes = HashMap::new();
                                    keyframes.insert("opacity", ["0", "1"]);
                                    crate::utils::new_animation(
                                        &current_element,
                                        &Object::try_from(
                                            &serde_wasm_bindgen::to_value(&keyframes).unwrap(),
                                        )
                                        .unwrap(),
                                        props.close_transition.duration,
                                        &props.close_transition.easing,
                                    );
                                }
                            }
                            self.open = true;
                            if self.position.is_none() {
                                if let Some(element) = e.target_dyn_into::<web_sys::HtmlElement>() {
                                    current_element
                                        .set_attribute(
                                            "style",
                                            &format!("{}; display:block; position: fixed; opacity: 0; top:0; left:0", &style),
                                        )
                                        .unwrap();
                                    let current_rect = current_element.get_bounding_client_rect();
                                    let current_width = current_rect.width();
                                    let current_height = current_rect.height();
                                    current_element.set_attribute("style", &style).unwrap();

                                    let win = window().unwrap();

                                    let target_rect = element.get_bounding_client_rect();

                                    let window_height =
                                        win.inner_height().unwrap().as_f64().unwrap();
                                    let window_width = win.inner_width().unwrap().as_f64().unwrap();

                                    let position = calculate_position(
                                        target_rect,
                                        current_width,
                                        current_height,
                                        window_width,
                                        window_height,
                                        align,
                                        None,
                                        0,
                                    );
                                    self.position = Some(position);
                                }
                            }
                            true
                        }
                        XButtonEvent::MouseLeave(_e) => {
                            if self.open {
                                let props = ctx.props();
                                if props.on_close.is_some() {
                                    props.on_close.clone().unwrap().emit(());
                                }

                                if props.close_transition.property == "opacity" {
                                    let current_element = self.node_ref.cast::<Element>().unwrap();
                                    let mut keyframes = HashMap::new();
                                    keyframes.insert("opacity", ["1", "0"]);
                                    let animate = crate::utils::new_animation(
                                        &current_element,
                                        &Object::try_from(
                                            &serde_wasm_bindgen::to_value(&keyframes).unwrap(),
                                        )
                                        .unwrap(),
                                        props.close_transition.duration,
                                        &props.close_transition.easing,
                                    );

                                    ctx.link().send_future(async move {
                                        JsFuture::from(animate.finished().unwrap()).await.unwrap();
                                        XTooltipMessage::XButtonCloseAnimationEnd
                                    });
                                }
                            }
                            true
                        }
                        _ => false,
                    }
                } else {
                    false
                }
            }
            XTooltipMessage::XButtonCloseAnimationEnd => {
                self.open = false;
                true
            }
        }
    }

    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        let props = ctx.props();
        let mut classes = String::from("x-tooltip");

        if let Some(user_class) = props.class.clone() {
            classes.push_str(user_class.trim());
        }

        if props.disabled {
            classes.push_str(" disabled");
        }

        write!(&mut classes, " type-{}", props.r#type.to_string()).unwrap();
        #[cfg(feature = "element-x-button")]
        write!(&mut classes, " align-{}", props.align.to_string()).unwrap();

        let mut style = String::new();

        if let Some(user_style) = props.style.clone() {
            style.push_str(&user_style);
            if !user_style.trim().ends_with(";") {
                style.push_str(";");
            }
        }

        #[cfg(feature = "element-x-button")]
        if (props.open.is_some() && props.open.unwrap() || props.open.is_none() && self.open)
            && self.position.is_some()
        {
            let position = self.position.clone().unwrap();
            write!(
                &mut style,
                "top:{}px;left:{}px;display:block",
                position.y().round() as u32,
                position.x().round() as u32
            )
            .unwrap();
        }
        #[cfg(not(feature = "element-x-button"))]
        if props.open.is_some() && props.open.unwrap() {
            style.push_str("display: block;")
        }

        #[cfg(not(feature = "element-x-button"))]
        let node_ref = NodeRef::default();
        #[cfg(feature = "element-x-button")]
        let node_ref = self.node_ref.clone();

        html! {
            <div ref={node_ref} class={classes} style={style}>
                <div class="x-tooltip-contents">
                    {for props.children.iter()}
                </div>
            </div>
        }
    }
}

#[cfg(feature = "element-x-button")]
fn calculate_position(
    target_rect: web_sys::DomRect,
    tool_width: f64,
    tool_height: f64,
    win_width: f64,
    win_height: f64,
    align: XTooltipAlign,
    from: Option<XTooltipAlign>,
    try_count: u8,
) -> web_sys::DomRect {
    use web_sys::DomRect;
    if try_count > 3 {
        return target_rect;
    }

    let target_bottom = target_rect.bottom();
    let target_top = target_rect.top();
    let target_left = target_rect.left();
    let target_right = target_rect.right();
    let target_width = target_rect.width();
    let target_height = target_rect.height();

    let other_axis = match align {
        XTooltipAlign::Bottom | XTooltipAlign::Top => {
            let mut x = target_left - (tool_width - target_width) / 2.0;
            if x < WINDOW_WHITESPACE {
                x = WINDOW_WHITESPACE;
            }
            if target_right + (tool_width - target_width) / 2.0 + SPACE_BETWEEN + WINDOW_WHITESPACE
                > win_width
            {
                x = win_width - tool_width - SPACE_BETWEEN - WINDOW_WHITESPACE;
            }

            x
        }
        XTooltipAlign::Left | XTooltipAlign::Right => {
            let mut y = target_top - (tool_height - target_height) / 2.0;

            if y < WINDOW_WHITESPACE {
                y = WINDOW_WHITESPACE;
            }

            if target_bottom
                + (tool_height - target_height) / 2.0
                + SPACE_BETWEEN
                + WINDOW_WHITESPACE
                > win_height
            {
                y = win_height - tool_height - SPACE_BETWEEN - WINDOW_WHITESPACE;
            }

            y
        }
    };

    match align {
        XTooltipAlign::Bottom => {
            if target_bottom + tool_height + SPACE_BETWEEN + WINDOW_WHITESPACE > win_height {
                return calculate_position(
                    target_rect,
                    tool_width,
                    tool_height,
                    win_width,
                    win_height,
                    if let Some(XTooltipAlign::Top) = from {
                        XTooltipAlign::Right
                    } else {
                        XTooltipAlign::Top
                    },
                    Some(align),
                    try_count + 1,
                );
            }
            let top = target_bottom + SPACE_BETWEEN;

            DomRect::new_with_x_and_y_and_width_and_height(other_axis, top, tool_width, tool_height)
                .unwrap()
        }
        XTooltipAlign::Top => {
            if target_top - tool_height - SPACE_BETWEEN - WINDOW_WHITESPACE < 0.0 {
                return calculate_position(
                    target_rect,
                    tool_width,
                    tool_height,
                    win_width,
                    win_height,
                    if let Some(XTooltipAlign::Bottom) = from {
                        XTooltipAlign::Right
                    } else {
                        XTooltipAlign::Bottom
                    },
                    Some(align),
                    try_count + 1,
                );
            }

            let top = target_top - SPACE_BETWEEN - tool_height;
            DomRect::new_with_x_and_y_and_width_and_height(other_axis, top, tool_width, tool_height)
                .unwrap()
        }
        XTooltipAlign::Left => {
            if target_left - tool_width - SPACE_BETWEEN - WINDOW_WHITESPACE < 0.0 {
                return calculate_position(
                    target_rect,
                    tool_width,
                    tool_height,
                    win_width,
                    win_height,
                    if let Some(XTooltipAlign::Right) = from {
                        XTooltipAlign::Bottom
                    } else {
                        XTooltipAlign::Right
                    },
                    Some(align),
                    try_count + 1,
                );
            }

            let right = target_left - SPACE_BETWEEN - tool_width;
            DomRect::new_with_x_and_y_and_width_and_height(
                right,
                other_axis,
                tool_width,
                tool_height,
            )
            .unwrap()
        }
        XTooltipAlign::Right => {
            if target_right + tool_width + SPACE_BETWEEN + WINDOW_WHITESPACE > win_width {
                return calculate_position(
                    target_rect,
                    tool_width,
                    tool_height,
                    win_width,
                    win_height,
                    if let Some(XTooltipAlign::Left) = from {
                        XTooltipAlign::Bottom
                    } else {
                        XTooltipAlign::Left
                    },
                    Some(align),
                    try_count + 1,
                );
            }

            let right = target_right + SPACE_BETWEEN;
            DomRect::new_with_x_and_y_and_width_and_height(
                right,
                other_axis,
                tool_width,
                tool_height,
            )
            .unwrap()
        }
    }
}
