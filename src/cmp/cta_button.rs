use super::*;

#[component]
pub fn CtaButton(
    class: Option<String>,
    style: Option<String>,
    children: Option<Element>
) -> Element {
    let conf: conf::Conf = use_context();
    let mut hover: Signal<bool> = use_signal(|| false);

    rsx!(
        button {
            onmouseenter: move |_| *hover.write() = true,
            onmouseleave: move |_| *hover.write() = false,
            all: "unset",
            display: "flex",
            flex_direction: "column",
            justify_content: "center",
            align_items: "center",
            font_family: conf.font.br_cobane,
            font_weight: "normal",
            color: if *hover.read() {
                conf.color.raisin_black.to_string()
            } else {
                conf.color.silver.to_string()
            },
            cursor: format!("url({}), auto", conf.cursor.finger),
            border_width: "1px",
            border_style: "solid",
            border_image: if *hover.read() {
                format!("linear-gradient(to bottom right, {}) 1", conf.color.silver)
            } else {
                format!("linear-gradient(to bottom right, {}, {}, {}) 1", conf.color.medium_slate_blue, conf.color.rose_pompadour, conf.color.mindaro)
            },
            border_radius: "2px",
            padding: "8px",
            background: if *hover.read() {
                format!("linear-gradient(to right bottom, {}, {}. {})", conf.color.medium_slate_blue, conf.color.rose_pompadour, conf.color.mindaro)
            } else {
                "transparent".to_owned()
            },
            { children }
        }
    )
}