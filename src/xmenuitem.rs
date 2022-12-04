use web_sys::{MouseEvent, KeyboardEvent, FocusEvent};
use yew::{html, Component, Properties, use_context, Children, Callback};
use std::fmt::Write;

use crate::{calculate_computed_size, XComponentSize, xcontainer::XContainerContext};

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
    focused: bool
}

pub enum XMenuItemChild {
}

pub enum XMenuItemMessage {
    Focus(FocusEvent),
    Blur(Blu)
}

impl Component for XMenuItem {
    type Message = ();
    type Properties = XMenuItemProps;

    fn create(ctx: &yew::Context<Self>) -> Self {
        XMenuItem {
            focused: false
        }
    }

    fn update(&mut self, ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
        
    }

    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        let props = ctx.props();
        let mut classes = String::from("x-menuitem");

        let context =
            use_context::<XContainerContext>().expect("XContainer should be the root element");

        let computed_size = calculate_computed_size(props.size.clone(), context.size.clone());
        write!(classes, " computedsize-{}", computed_size.to_string()).unwrap();

        if props.togglable {
            classes.write_str(" togglable").unwrap();
        }

        if props.toggled {
            classes.write_str(" toggled").unwrap();
        }

        html! {
          <div role="menuitem" class={classes.clone()}>
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
