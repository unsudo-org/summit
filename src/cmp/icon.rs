use super::*;

#[derive(Props, Clone, PartialEq)]
pub struct IconProps {
    pub url: Asset,
    pub color: Hex,
    pub w: String,

    #[props(extends = GlobalAttributes)]
    pub attr: Vec<Attribute>
}

#[component]
pub fn Icon(props: IconProps) -> Element {
    rsx!(
        div {
            display: "flex",
            flex_direction: "column",
            justify_content: "center",
            align_items: "center",
            min_width: props.w,
            aspect_ratio: "1 / 1",
            background_image: format!("url({})", props.url),
            background_position: "center",
            background_size: "contain",
            background_repeat: "no-repeat",
            color: format!("{}", props.color),
            ..props.attr
        }
    )
}