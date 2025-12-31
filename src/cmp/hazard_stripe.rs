// hazard-motion.css

use super::*;

#[derive(Props, Clone, PartialEq)]
pub struct HazardStripeProps {
    pub min_w: Option<String>,
    pub max_w: Option<String>,
    pub min_h: Option<String>,
    pub max_h: Option<String>,
    pub color_0: Hex,
    pub color_1: Hex,
    pub color_2: Hex,
    pub color_3: Hex,
    pub size_0: Hex,
    pub size_1: Hex,
    pub size_2: Hex,
    pub size_3: Hex,
    pub duration: time::Duration,

    #[props(extends = GlobalAttributes)]
    pub attr: Vec<Attribute>
}

#[component]
pub fn HazardStripe(props: HazardStripeProps) -> Element {
    rsx!(
        div {
            min_width: props.min_w,
            max_width: props.max_w,
            min_height: props.min_h,
            max_height: props.max_h,
            background: format!(
                "repeating-linear-gradient(45deg, {} {}px, {} {}px, {} {}px, {} {}px)",
                props.color_0,
                props.size_0,
                props.color_1,
                props.size_1,
                props.color_2,
                props.size_2,
                props.color_3,
                props.size_3
            ),
            animation: format!("hazard-motion {}s linear infinite", props.duration.as_millis()),
            ..props.attr
        }
    )
}