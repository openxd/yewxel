use std::fmt::Write;
use web_sys::MouseEvent;
use yew::html::ChildrenRenderer;
use yew::{
    function_component, use_context, use_reducer, use_state, Callback, Reducible,
    UseReducerHandle,
};
use yew::{html, ContextProvider, Properties};

use crate::calculate_computed_size;
use crate::xcontainer::XContainerContext;
use crate::XComponentSize;

#[derive(PartialEq, Clone)]
pub enum XButtonSkin {
    Flat,
    Recessed,
    Nav,
    Dock,
    Circular,
    Default,
}

impl Default for XButtonSkin {
    fn default() -> Self {
        XButtonSkin::Default
    }
}

impl ToString for XButtonSkin {
    fn to_string(&self) -> String {
        match self {
            Self::Flat => String::from("flat"),
            Self::Recessed => String::from("recessed"),
            Self::Nav => String::from("nav"),
            Self::Dock => String::from("dock"),
            Self::Circular => String::from("circular"),
            Self::Default => String::from("default"),
        }
    }
}

#[derive(PartialEq, Clone, derive_more::From)]
pub enum XButtonChild {
    #[cfg(feature = "element-x-tooltip")]
    Tooltip(yew::virtual_dom::VChild<crate::xtooltip::XTooltip>),
    #[cfg(feature = "element-x-label")]
    Label(yew::virtual_dom::VChild<crate::xlabel::XLabel>),
    Other(yew::Html),
}

impl Into<yew::Html> for XButtonChild {
    fn into(self) -> yew::Html {
        match self {
            #[cfg(feature = "element-x-tooltip")]
            Self::Tooltip(child) => child.into(),
            #[cfg(feature = "element-x-label")]
            Self::Label(child) => child.into(),
            Self::Other(child) => child.into(),
        }
    }
}

/// All props associated with the XButton component
#[derive(PartialEq, Properties)]
pub struct XButtonProps {
    /// A unique value associated with this button.
    #[prop_or_default]
    pub value: Option<String>,
    /// Whether this button is toggled.
    #[prop_or_default]
    pub toggled: bool,
    /// Whether this button can be toggled on/off by the user (e.g. by clicking the button).
    #[prop_or_default]
    pub togglable: bool,
    /// Whether the this button has "mixed" state.
    #[prop_or_default]
    pub mixed: bool,
    /// Whether this button is disabled.
    #[prop_or_default]
    pub disabled: bool,
    /// Whether the button should take less horizontal space.
    #[prop_or_default]
    pub condensed: bool,

    #[prop_or_default]
    pub skin: XButtonSkin,

    #[prop_or_default]
    pub size: Option<XComponentSize>,

    #[prop_or_default]
    pub children: ChildrenRenderer<XButtonChild>,

    #[prop_or_default]
    pub onclick: Option<Callback<MouseEvent>>,

    #[prop_or_default]
    pub ontoggle: Option<Callback<MouseEvent>>,
}

#[derive(PartialEq)]
pub enum XButtonEvent {
    Click(MouseEvent),
    MouseEnter(MouseEvent),
    MouseLeave(MouseEvent),
}

#[derive(PartialEq)]
pub struct XButtonMessage {
    pub event: Option<XButtonEvent>,
}

impl Reducible for XButtonMessage {
    type Action = Option<XButtonEvent>;

    fn reduce(self: std::rc::Rc<Self>, action: Self::Action) -> std::rc::Rc<Self> {
        XButtonMessage { event: action }.into()
    }
}

pub(crate) type XButtonContext = UseReducerHandle<XButtonMessage>;

/// `XButton` has the same purpose as the standard HTML `button` element, but can be easily
/// composited with other YewXel elements.
#[function_component(XButton)]
pub fn x_button(props: &XButtonProps) -> yew::Html {
    let pressed = use_state(|| false);

    for _child in props.children.iter() {
        // TODO: Check for menu and popovers 
    }

    let mut classes = String::from("x-button");

    if props.toggled {
        classes.push_str(" toggled");
    }

    if props.disabled {
        classes.push_str(" disabled");
    }

    if props.togglable {
        classes.push_str(" togglable");
    }

    if props.disabled {
        classes.push_str(" disabled");
    }

    if props.condensed {
        classes.push_str(" condensed");
    }

    if *pressed.clone() {
        classes.push_str(" pressed");
    }

    write!(classes, " skin-{}", props.skin.to_string()).unwrap();

    if let Some(size) = props.size.clone() {
        write!(classes, " size-{}", size.to_string()).unwrap();
    }

    let context =
        use_context::<XContainerContext>().expect("XContainer should be the root element");

    let computed_size = calculate_computed_size(props.size.clone(), context.size.clone());
    write!(classes, " computedsize-{}", computed_size.to_string()).unwrap();

    let message = use_reducer(|| XButtonMessage { event: None });

    let onclick = {
        let message = message.clone();
        let oc_callback = props.onclick.clone();
        let ot_callback = props.ontoggle.clone();
        let togglable = props.togglable;
        Callback::from(move |e: MouseEvent| {
            if togglable {
                if let Some(callback) = &ot_callback {
                    callback.emit(e);
                }
            } else if let Some(callback) = &oc_callback {
                message.dispatch(Some(XButtonEvent::Click(e.clone())));
                callback.emit(e);
            }
        })
    };

    let onmouseenter = {
        let message = message.clone();
        Callback::from(move |e: MouseEvent| {
            message.dispatch(Some(XButtonEvent::MouseEnter(e)));
        })
    };

    let onmouseleave = {
        let message = message.clone();
        Callback::from(move |e: MouseEvent| {
            message.dispatch(Some(XButtonEvent::MouseLeave(e)));
        })
    };

    let onmousedown = {
        let pressed = pressed.clone();
        Callback::from(move |_e: MouseEvent| {
            pressed.set(true);
        })
    };

    let onmouseup = {
        let pressed = pressed.clone();
        Callback::from(move |_e: MouseEvent| {
            pressed.set(false);
        })
    };

    html! {
        <ContextProvider<XButtonContext> context={message}>
        <div
            onclick={onclick}
            onmouseenter={onmouseenter}
            onmousedown={onmousedown}
            onmouseleave={onmouseleave}
            onmouseup={onmouseup}
            class={classes}>
          <div class="x-button-ripples"></div>
          {for props.children.iter()}
          <svg class="x-button-arrow" part="arrow" viewBox="0 0 100 100" preserveAspectRatio="none">
            <path class="x-button-arrow-path"></path>
          </svg>
        </div>
        </ContextProvider<XButtonContext>>
    }
}
