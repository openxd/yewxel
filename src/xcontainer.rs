use std::sync::Once;

use web_sys::window;
use yew::{html, Children, Component, Properties, ContextProvider};

use crate::ComputedSize;

static BASE_CSS: &'static str = include_str!(concat!(env!("OUT_DIR"), "/base.css"));

#[cfg(all(feature = "mode-light", feature = "mode-dark"))]
static LIGHT_CSS: &'static str = include_str!(concat!(env!("OUT_DIR"), "/light.css"));
#[cfg(all(feature = "mode-light", feature = "mode-dark"))]
static DARK_CSS: &'static str = include_str!(concat!(env!("OUT_DIR"), "/dark.css"));

static LOADED: Once = Once::new();

/// User preferred color mode
#[cfg(all(feature = "mode-light", feature = "mode-dark"))]
#[derive(Clone, PartialEq)]
pub enum Mode {
    /// Light color
    Light,
    /// Dark colors
    Dark,
    /// Based on browser settings
    Auto,
}

#[cfg(all(feature = "mode-light", feature = "mode-dark"))]
impl Default for Mode {
    fn default() -> Self {
        Mode::Auto
    }
}

#[derive(PartialEq, Properties, Clone)]
pub struct XContainerProps {
    #[cfg(all(feature = "mode-light", feature = "mode-dark"))]
    #[prop_or_default]
    pub mode: Mode,

    #[prop_or_default]
    pub size: ComputedSize,

    #[prop_or_default]
    pub children: Children,
}

#[derive(Clone, PartialEq)]
pub struct XContainerContext {
    pub size: ComputedSize,
}

pub struct XContainer;

impl Component for XContainer {
    type Message = ();
    type Properties = XContainerProps;

    fn create(_ctx: &yew::Context<Self>) -> Self {
        #[cfg(all(feature = "mode-light", feature = "mode-dark"))]
        let mode = _ctx.props().mode.clone();
        LOADED.call_once(move || {
            let window = window().unwrap();
            let document = window.document().unwrap();
            if document.get_element_by_id("xThemeBase").is_none() {
                let head_tag = document.get_elements_by_tag_name("head").item(0).unwrap();
                let style_tag = document.create_element("style").unwrap();
                style_tag.set_attribute("type", "text/css").unwrap();
                style_tag.set_id("xThemeBase");
                style_tag.set_inner_html(BASE_CSS);
                head_tag.append_child(&style_tag).unwrap();
            }

            #[cfg(all(feature = "mode-light", feature = "mode-dark"))]
            apply_mode_styles(window, document, mode);
        });
        XContainer
    }

    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        html! {
            <div class="x-container">
                <ContextProvider<XContainerContext> context={XContainerContext {size: ctx.props().size.clone()}}>
                    {for ctx.props().children.iter()}
                </ContextProvider<XContainerContext>>
            </div>
        }
    }
}

#[cfg(all(feature = "mode-light", feature = "mode-dark"))]
pub fn apply_mode_styles(window: web_sys::Window, document: web_sys::Document, mode: Mode) {
    let styles = match mode {
        Mode::Dark => DARK_CSS,
        Mode::Light => LIGHT_CSS,
        Mode::Auto => {
            let mut dark = false;
            if let Ok(query_opt) = window.match_media("(prefers-color-scheme: dark)") {
                if let Some(query) = query_opt {
                    if query.matches() {
                        dark = true;
                    }
                }
            }

            if dark {
                DARK_CSS
            } else {
                LIGHT_CSS
            }
        }
    };

    if let Some(style_tag) = document.get_element_by_id("xThemeMode") {
        style_tag.set_inner_html(styles);
    } else {
        let head_tag = document.get_elements_by_tag_name("head").item(0).unwrap();
        let style_tag = document.create_element("style").unwrap();
        style_tag.set_attribute("type", "text/css").unwrap();
        style_tag.set_id("xThemeMode");
        style_tag.set_inner_html(styles);
        head_tag.append_child(&style_tag).unwrap();
    }
}
