use yew::{Component, Properties, html, Callback, NodeRef};

use crate::{Transition, CSSEasing};

pub struct XMenu;

#[derive(PartialEq)]
pub enum XMenuAlign {
    Start,
    End
}

#[derive(PartialEq)]
pub enum XMenuOpen {
    OverElement(NodeRef, NodeRef),
    OverLabel(NodeRef),
    NextToElement(NodeRef),
    AtPoint(f64, f64)
}

#[derive(PartialEq, Properties)]
pub struct XMenuProps {
    #[prop_or(Transition::new("transform", 100.0, CSSEasing::CubicBezier(0.4, 0.0, 0.2, 1.0)))]
    pub open_transition: Transition,
    #[prop_or(Transition::new("opacity", 200.0, CSSEasing::CubicBezier(0.4, 0.0, 0.2, 1.0)))]
    pub close_transition: Transition,
    #[prop_or(XMenuAlign::Start)]
    pub align: XMenuAlign,
    #[prop_or_default]
    pub open: Option<XMenuOpen>,
    #[prop_or_default]
    pub on_open: Option<Callback<XMenuOpen>>,
    #[prop_or_default]
    pub on_open_finished: Option<Callback<XMenuOpen>>,
    #[prop_or_default]
    pub on_close: Option<Callback<()>>,
    #[prop_or_default]
    pub on_close_finished: Option<Callback<()>>
}

pub enum XMenuChild {
    #[cfg(feature="element-x-menuitem")]
    Item(crate::xmenuitem::XMenuItem)
}

impl Component for XMenu {
    type Properties = XMenuProps;
    type Message = ();

    fn create(ctx: &yew::Context<Self>) -> Self {
        XMenu
    }

    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        html! {
            <div class="x-menu">
            </div>
        }
    }
}
