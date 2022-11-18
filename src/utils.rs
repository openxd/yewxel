pub(crate) fn console_log(message: String) {
    web_sys::console::log_1(&wasm_bindgen::JsValue::from_str(&message));
}

// Animations
#[cfg(all(feature="element-x-tooltip", feature="element-x-button"))]
pub(crate) fn new_animation(
    target: &web_sys::Element,
    keyframes: &js_sys::Object,
    duration: f64,
    easing: &crate::CSSEasing,
) -> web_sys::Animation {
    use web_sys::{AnimationEffect, KeyframeEffect, OptionalEffectTiming};
    let keyframe =
        KeyframeEffect::new_with_opt_element_and_keyframes(Some(&target), Some(keyframes)).unwrap();
    let animation_effect = AnimationEffect::from(keyframe);

    let mut timing = OptionalEffectTiming::new();
    timing.duration(&wasm_bindgen::JsValue::from_f64(duration));
    timing.easing(easing.to_string().as_str());
    animation_effect.update_timing_with_timing(&timing).unwrap();

    let animation = web_sys::Animation::new_with_effect(Some(&animation_effect)).unwrap();
    animation
}
