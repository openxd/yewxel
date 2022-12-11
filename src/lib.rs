pub mod xcontainer;
#[cfg(feature="element-x-button")]
pub mod xbutton;
#[cfg(feature="element-x-label")]
pub mod xlabel;
#[cfg(feature="element-x-tooltip")]
pub mod xtooltip;
#[cfg(all(feature="element-x-message", feature="feature-intl"))]
pub mod xmessage;
#[cfg(feature="element-x-menuitem")]
pub mod xmenuitem;
#[cfg(feature="feature-intl")]
mod intl;
#[cfg(feature="element-x-menu")]
pub mod xmenu;
mod utils;

/// Re-exported from `web_sys` crate.
#[cfg(feature = "feature-intl")]
pub use web_sys::Url;

#[derive(PartialEq, Clone, Debug)]
pub enum ComputedSize {
    Small,
    Medium,
    Large
}

impl Default for ComputedSize {
    fn default() -> Self {
        Self::Medium
    }
}

impl ToString for ComputedSize {
    fn to_string(&self) -> String {
        match self {
            Self::Small => String::from("small"),
            Self::Medium => String::from("medium"),
            Self::Large => String::from("large")
        }
    }
}

#[derive(PartialEq, Clone)]
pub enum XComponentSize {
    Smaller,
    Small,
    Medium,
    Large,
    Larger
}

impl Default for XComponentSize {
    fn default() -> Self {
        Self::Medium
    }
}

impl ToString for XComponentSize {
    fn to_string(&self) -> String {
        match self {
            Self::Smaller => String::from("smaller"),
            Self::Small => String::from("small"),
            Self::Medium => String::from("medium"),
            Self::Large => String::from("large"),
            Self::Larger => String::from("larger")
        }
    }
}

#[derive(PartialEq)]
pub enum CSSEasing {
    Linear,
    Ease,
    EaseIn,
    EaseOut,
    EaseInOut,
    CubicBezier(f64, f64, f64, f64)
}

impl ToString for CSSEasing {
    fn to_string(&self) -> String {
        match self {
            Self::Linear => String::from("linear"),
            Self::Ease => String::from("ease"),
            Self::EaseIn => String::from("ease-in"),
            Self::EaseOut => String::from("ease-out"),
            Self::EaseInOut => String::from("ease-in-out"),
            Self::CubicBezier(x1,y1,x2,y2) => format!("cubic-bezier({},{},{},{})",x1, y1, x2, y2)
        }
    }
}

#[cfg(any(feature="element-x-button", feature="element-x-menuitem"))]
pub(crate) fn calculate_computed_size(opt_custom_size: Option<XComponentSize>, default_size: ComputedSize) -> ComputedSize {
    match opt_custom_size {
        Some(custom_size) => match custom_size {
            XComponentSize::Smaller => if default_size == ComputedSize::Large {ComputedSize::Medium} else {ComputedSize::Small},
            XComponentSize::Larger => if default_size == ComputedSize::Small {ComputedSize::Medium} else {ComputedSize::Large},
            XComponentSize::Small => ComputedSize::Small,
            XComponentSize::Medium => ComputedSize::Medium,
            XComponentSize::Large => ComputedSize::Large
        },
        None => default_size
    }
}

