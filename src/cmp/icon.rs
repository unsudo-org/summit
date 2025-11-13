use super::*;

#[component]
pub fn Icon(url: Asset, w: String) -> Element {
    let conf: conf::Conf = use_context();

    rsx!(
        div {
            display: "flex",
            flex_direction: "column",
            justify_content: "center",
            align_items: "center",
            min_width: w,
            aspect_ratio: "1 / 1",
            background_image: format!("url({})", url),
            background_position: "center",
            background_size: "contain",
            background_repeat: "no-repeat",
            color: conf.color.timberwolf
        }
    )
}