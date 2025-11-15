use super::*;

#[component]
pub fn Button(children: Option<Element>) -> Element {
    let conf: conf::Conf = use_context();
    let mut hover: Signal<bool> = use_signal(|| false);

    rsx!(
        button {
            onmouseenter: move |_| *hover.write() = true,
            onmouseleave: move |_| *hover.write() = false,
            all: "unset",
            display: "flex",
            flex_direction: "row",
            justify_content: "start",
            align_items: "start",
            font_family: conf.font.br_cobane,
            font_weight: "normal",
            color: if *hover.read() {
                conf.color.timberwolf.to_string()
            } else {
                conf.color.raisin_black.to_string()
            },
            border_width: "2px",
            border_style: "solid",
            border_image: format!("linear-gradient(to bottom right, {}) 1", conf.color.raisin_black),
            border_radius: "2px",
            background: if *hover.read() {
                conf.color.raisin_black.to_string()
            } else {
                "transparent".to_owned()
            },
            cursor: format!("url('{}'), auto", conf.cursor.finger),
            padding: "8px",
            transition: "linear 0.1s",
            { children }
        }
    )
}

#[component]
pub fn ButtonDashed(children: Option<Element>) -> Element {
    let conf: conf::Conf = use_context();
    let mut hover: Signal<bool> = use_signal(|| false);

    rsx!(
        button {
            onmouseenter: move |_| *hover.write() = true,
            onmouseleave: move |_| *hover.write() = false,
            all: "unset",
            display: "flex",
            flex_direction: "row",
            justify_content: "start",
            align_items: "start",
            font_family: conf.font.br_cobane,
            font_weight: "normal",
            color: if *hover.read() {
                conf.color.timberwolf.to_string()
            } else {
                conf.color.raisin_black.to_string()
            },
            border_width: "1px",
            border_style: "dashed",
            border_color: conf.color.raisin_black.to_string(),
            border_radius: "2px",
            background: if *hover.read() {
                conf.color.raisin_black.to_string()
            } else {
                "transparent".to_owned()
            },
            cursor: format!("url('{}'), auto", conf.cursor.finger),
            padding: "8px",
            transition: "linear 0.1s",
            { children }
        }
    )
}