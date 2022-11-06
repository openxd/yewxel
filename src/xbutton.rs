use std::fmt::Write;
use yew::{html, Children, Component, Properties};

use crate::calculate_computed_style;
use crate::ComputedSize;
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
    pub children: Children,
}

/// `XButton` has the same purpose as the standard HTML `button` element, but can be easily
/// composited with other YewXel elements.
pub struct XButton;

impl Component for XButton {
    type Message = ();
    type Properties = XButtonProps;

    fn create(_ctx: &yew::Context<Self>) -> Self {
        XButton
    }

    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        let props = ctx.props();
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

        write!(classes, " skin-{}", props.skin.to_string()).unwrap();

        if let Some(size) = props.size.clone() {
            write!(classes, " size-{}", size.to_string()).unwrap();
        }

        // TODO: Get the default size from context
        let computed_size = calculate_computed_style(props.size.clone(), ComputedSize::Medium);
        write!(classes, " computedsize-{}", computed_size.to_string()).unwrap();

        html! {
            <div class={classes}>
              <div class="x-button-ripples"></div>
              {for ctx.props().children.iter()}
              <svg class="x-button-arrow" part="arrow" viewBox="0 0 100 100" preserveAspectRatio="none">
                <path class="x-button-arrow-path"></path>
              </svg>
            </div>
        }
    }
}
