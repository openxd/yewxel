use yew::{html, Component, Properties};

use crate::XComponentSize;

#[derive(PartialEq, Properties)]
pub struct XMenuItemProps {
    #[prop_or(true)]
    pub disabled: bool,
    #[prop_or_default]
    pub size: Option<XComponentSize>,
    #[prop_or_default]
    pub toggled: bool,
}

pub struct XMenuItem;

impl Component for XMenuItem {
    type Message = ();
    type Properties = XMenuItemProps;

    fn create(ctx: &yew::Context<Self>) -> Self {
        XMenuItem
    }

    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        html! {
          <div class="x-menuitem">
            <div class="ripples"></div>
            <svg class="checkmark" viewBox="0 0 100 100" preserveAspectRatio="none">
              <path></path>
            </svg>
            <slot></slot>
            <svg class="arrow" viewBox="0 0 100 100" hidden={true}>
              <path></path>
            </svg>
          </div>
        }
    }
}
