use js_sys::Object;
use wasm_bindgen::JsValue;
use web_sys::{Animation, AnimationEffect, Element, KeyframeEffect, OptionalEffectTiming};

use crate::CSSEasing;

pub(crate) fn console_log(message: String) {
    web_sys::console::log_1(&wasm_bindgen::JsValue::from_str(&message));
}

pub(crate) fn new_animation(
    target: &Element,
    keyframes: &Object,
    duration: f64,
    easing: &CSSEasing,
) -> Animation {
    let keyframe =
        KeyframeEffect::new_with_opt_element_and_keyframes(Some(&target), Some(keyframes)).unwrap();
    let animation_effect = AnimationEffect::from(keyframe);

    let mut timing = OptionalEffectTiming::new();
    timing.duration(&JsValue::from_f64(duration));
    timing.easing(easing.to_string().as_str());
    animation_effect.update_timing_with_timing(&timing).unwrap();

    let animation = Animation::new_with_effect(Some(&animation_effect)).unwrap();
    animation
}
