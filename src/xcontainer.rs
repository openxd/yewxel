use std::sync::Once;

use prokio::spawn_local;
use web_sys::window;
use yew::{html, Children, Component, ContextProvider, Properties};

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

    /// Urls to the FTL files
    #[cfg(feature = "feature-intl")]
    #[prop_or_default]
    pub ftls: Vec<web_sys::Url>,

    /// Identifier of the language
    #[cfg(feature = "feature-intl")]
    #[prop_or(String::from("en-US"))]
    pub locale: String,

    #[prop_or_default]
    pub children: Children,
}

#[cfg(feature = "feature-intl")]
pub enum IntlState {
    Initialized,
    Loaded(fluent::FluentBundle<fluent::FluentResource>),
}

#[derive(PartialEq)]
pub enum XContainerMessage {
    Initialized,
    #[cfg(feature = "feature-intl")]
    LocaleLoaded(web_sys::Url, String),
}

#[derive(Clone, PartialEq)]
pub struct XContainerContext {
    pub size: ComputedSize,
}

pub struct XContainer {
    #[cfg(feature = "feature-intl")]
    pub intl_state: std::rc::Rc<std::cell::RefCell<crate::intl::Intl>>,
    #[cfg(feature = "feature-intl")]
    old_props: XContainerProps,
}

impl Component for XContainer {
    type Message = XContainerMessage;
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

        XContainer {
            #[cfg(feature = "feature-intl")]
            intl_state: {
                let intl = std::rc::Rc::new(std::cell::RefCell::new(crate::intl::Intl::new(
                    _ctx.props().locale.clone(),
                )));
                let link = _ctx.link().clone();

                if _ctx.props().ftls.len() > 0 {
                    let ftls = _ctx.props().ftls.clone();
                    spawn_local(async move {
                        for ftl in ftls {
                            let contents =
                                crate::utils::load_text_content(ftl.clone()).await.unwrap();
                            link.send_message(XContainerMessage::LocaleLoaded(ftl, contents));
                        }
                    });
                }

                intl
            },
            #[cfg(feature = "feature-intl")]
            old_props: _ctx.props().clone(),
        }
    }

    #[cfg(feature = "feature-intl")]
    fn update(&mut self, _ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
        match msg {
            XContainerMessage::LocaleLoaded(url, content) => {
                self.intl_state.borrow_mut().load(url, content).unwrap();
                true
            }
            _ => false,
        }
    }

    #[cfg(feature = "feature-intl")]
    fn changed(&mut self, ctx: &yew::Context<Self>) -> bool {
        if !ctx.props().ftls.eq(&self.old_props.ftls) || ctx.props().locale != self.old_props.locale
        {
            if ctx.props().locale != self.old_props.locale {
                self.intl_state
                    .borrow_mut()
                    .change(ctx.props().locale.clone());
            }

            let ftls = ctx.props().ftls.clone();
            let link = ctx.link().clone();
            if ftls.len() > 0 {
                spawn_local(async move {
                    for ftl in ftls {
                        let contents = crate::utils::load_text_content(ftl.clone()).await.unwrap();
                        link.send_message(XContainerMessage::LocaleLoaded(ftl, contents));
                    }
                });
            }
        }

        self.old_props = ctx.props().clone();
        true
    }

    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        let content: yew::Html = html! {
            <div class="x-container">
                <ContextProvider<XContainerContext> context={XContainerContext {size: ctx.props().size.clone()}}>
                    {for ctx.props().children.iter()}
                </ContextProvider<XContainerContext>>
            </div>
        };

        #[cfg(feature = "feature-intl")]
        html! {
            <ContextProvider<std::rc::Rc<std::cell::RefCell<crate::intl::Intl>>> context={self.intl_state.clone()}>
                {content}
            </ContextProvider<std::rc::Rc<std::cell::RefCell<crate::intl::Intl>>>>
        }
        #[cfg(not(feature = "feature-intl"))]
        content
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
