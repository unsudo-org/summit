use super::*;

#[component]
pub fn HazardStripe(
    foreground_color: Hex,
    background_color: Hex,
    duration: time::Duration,
    children: Option<Element>,

    #[props(extends = GlobalAttributes)]
    more: Vec<Attribute>
) -> Element {
    rsx!(
        div {
            background: format!(
                "repeating-linear-gradient(45deg, {} 0px, {} 8px, {} 8px, {} 16px)",
                foreground_color,
                foreground_color,
                background_color,
                background_color
            ),
            animation: format!("hazard-motion {}s linear infinite", duration.as_secs_f32()),
            ..more,
            { children }
        }
    )
}