use super::*;

#[component]
pub fn NavbarBuild() -> Element {
    rsx!(
        Navbar {
            left: rsx!(
                Logo {}
            ),
            right: rsx!(

            )
        }
    )
}

#[component]
pub fn Navbar(
    left: Option<Element>,
    right: Option<Element>,
    class: Option<String>,
    style: Option<String>,
    children: Option<Element>
) -> Element {
    rsx!(
        div {
            class: class,
            style: format!(
                r#"
                    display: flex;
                    flex-direction: row;
                    justify-content: space-between;
                    align-items: center;
                    min-width: 100%;
                    max-width: 100%;
                    padding-left: 10px;
                    padding-right: 10px;
                    {}
                "#,
                style.unwrap_or_default()
            ),
            div {
                style: format!(
                    r#"
                        display: flex;
                        flex-direction: row;
                        justify-content: start;
                        align-items: center;
                    "#
                ),
                { left }
            }
            div {
                style: format!(
                    r#"
                        display: flex;
                        flex-direction: row;
                        justify-content: end;
                        align-items: center;
                    "#
                ),
                { right }
            }
        }
    )
}


#[component]
pub fn NavbarGotoButton(children: Option<Element>) -> Element {
    rsx!(
        button {
            all: "unset",
            display: "flex",
            flex_direction: "row",
            justify_content: "center",
            align_items: "center",
            font_family: font::BR_COBANE,
            font_weight: "normal",
            color: color::TIMBERWOLF,
            cursor: format!("url('{}'), auto", cursor::FINGER),
            div {
                { children }
            }
        }
    )
}