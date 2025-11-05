use super::*;

#[component]
pub fn Navbar(
    left: Option<Element>,
    right: Option<Element>
) -> Element {
    rsx!(
        div {
            style: format!(
                r#"
                    display: flex;
                    flex-direction: row;
                    justify-content: space-between;
                    align-items: center;
                    min-width: 100%;
                    max-width: 100%;
                "#
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