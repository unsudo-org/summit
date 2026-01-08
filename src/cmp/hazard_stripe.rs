use super::*;

#[component]
pub fn HazardStripe(
    min_w: String,
    max_w: String,
    min_h: String,
    max_h: String,
    foreground_color: Hex,
    background_color: Hex,
    duration: time::Duration
) -> Element {
    rsx!(
        div {
            min_width: min_w,
            max_width: max_w,
            min_height: min_h,
            max_height: max_h,
            background: format!(
                "repeating-linear-gradient(45deg, {} 0px, {} 8px, {} 8px, {} 16px)",
                foreground_color,
                foreground_color,
                background_color,
                background_color
            ),
            animation: format!("hazard-motion {}s linear infinite", duration.as_secs_f32())
        }
    )
}