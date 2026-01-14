use super::*;

#[derive(Props, Clone, PartialEq)]
pub struct IconProps {
    pub url: Asset,
    pub color: String,
    pub w: String,
    #[props(extends = GlobalAttributes)]
    pub more: Vec<Attribute>
}

#[component]
pub fn Icon(props: IconProps) -> Element {
    rsx!(
        div {
            display: "flex",
            flex_direction: "column",
            justify_content: "center",
            align_items: "center",
            min_width: format!("{}", props.w),
            aspect_ratio: "1 / 1",
            background_image: format!("url({})", props.url),
            background_position: "center",
            background_size: "contain",
            background_repeat: "no-repeat",
            color: format!("{}", props.color),
            ..props.more
        }
    )
}