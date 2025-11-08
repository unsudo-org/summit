use super::*;

#[component]
pub fn Button(children: Option<Element>) -> Element {
    let mut hover: Signal<bool> = use_signal(|| false);

    rsx!(
        button {
            onmouseenter: move |_| *hover.write() = true,
            onmouseleave: move |_| *hover.write() = false,
            all: "unset",
            display: "flex",
            flex_direction: "row",
            justify_content: "center",
            align_items: "center",
            font_family: font::BR_COBANE,
            font_weight: "normal",
            color: if *hover.read() {
                color::RAISIN_BLACK
            } else {
                color::SILVER
            },
            cursor: format!("url('{}'), auto", cursor::FINGER),
            border_width: "1px",
            border_style: "solid",
            border_image: format!("linear-gradient(to top right, {}) 1", color::SILVER),
            border_radius: "2px",
            padding: "8px",
            background: if *hover.read() {
                color::SILVER
            } else {
                "transparent"
            },
            { children }
        }
    )
}