use super::*;

#[component]
pub fn NavbarBuild() -> Element {
    let conf: conf::Conf = use_context();

    rsx!(
        Navbar {
            left: rsx!(
                Logo {}
            ),
            right: rsx!(
                button {
                    all: "unset",
                    border_width: "1px",
                    border_style: "solid",
                    border_color: conf.color.timberwolf.to_string(),
                    border_radius: "2px",
                    padding: "8px",
                    font_family: conf.font.br_cobane,
                    font_weight: "normal",
                    color: conf.color.timberwolf.to_string(),
                    cursor: format!("url({}), auto", conf.cursor.finger),
                    "Connect"
                }
            )
        }
    )
}

#[component]
pub fn Navbar(left: Option<Element>, right: Option<Element>, center: Option<Element>) -> Element {
    rsx!(
        div {
            display: "flex",
            flex_direction: "row",
            justify_content: "space-between",
            align_items: "center",
            min_width: "100vw",
            max_width: "100vw",
            padding_left: "8px",
            padding_right: "8px",
            div {
                display: "flex",
                flex_direction: "row",
                justify_content: "start",
                align_items: "center",
                { left }
            }
            div {
                display: "flex",
                flex_direction: "row",
                justify_content: "center",
                align_items: "center",
                { center }
            }
            div {
                display: "flex",
                flex_direction: "row",
                justify_content: "end",
                align_items: "center",
                { right }
            }
        }
    )
}

#[component]
pub fn NavbarGotoButton(children: Option<Element>) -> Element {
    let conf: conf::Conf = use_context();
    rsx!(
        button {
            all: "unset",
            display: "flex",
            flex_direction: "row",
            justify_content: "center",
            align_items: "center",
            font_family: conf.font.br_cobane,
            font_weight: "normal",
            color: conf.color.timberwolf.to_string(),
            cursor: format!("url('{}'), auto", conf.cursor.finger),
            div {
                { children }
            }
        }
    )
}