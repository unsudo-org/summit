use super::*;

#[component]
pub fn NavbarBuild() -> Element {
    rsx!(
        Navbar {
            left: rsx!(
                Logo {
                    mode: LogoMode::Icon,
                    color: "#000000"
                }
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