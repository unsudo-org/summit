use super::*;

#[component]
pub fn Build() -> Element {
    let theme: theme::Theme = use_context();

    rsx!(
        Navbar {
            left: rsx!(
                div {
                    display: "flex",
                    flex_direction: "row",
                    justify_content: "start",
                    align_items: "center",
                    gap: "32px",
                    To {
                        to: "/",
                        logo::Logo {}
                    }
                    div {
                        display: "flex",
                        flex_direction: "row",
                        justify_content: "start",
                        align_items: "center",
                        gap: "16px",
                        Link {
                            all: "unset",
                            to: "/vision",
                            NavbarGotoButton { "Vision" }
                        }
                        Link {
                            all: "unset",
                            to: "/team",
                            NavbarGotoButton { "Team" }
                        }
                        Link {
                            all: "unset",
                            to: "/tokenomics",
                            NavbarGotoButton { "Tokenomics" }
                        }
                        Link {
                            all: "unset",
                            to: "/",
                            NavbarGotoButton { "Learn More" }
                        }
                    }
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
            padding_top: "8px",
            padding_left: "16px",
            padding_right: "16px",
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
    let theme: theme::Theme = use_context();

    rsx!(
        button {
            all: "unset",
            display: "flex",
            flex_direction: "row",
            justify_content: "center",
            align_items: "center",
            font_family: theme.font.body,
            font_weight: "normal",
            color: theme.color.foreground.to_string(),
            cursor: format!("url('{}'), auto", theme.cursor.finger),
            div {
                { children }
            }
        }
    )
}