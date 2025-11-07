use super::*;

#[component]
pub fn Home() -> Element {
    rsx!(
        cmp::Page {
            scroll_snap: cmp::PageScrollSnap::Proximity,
            style: format!(
                r#"
                    background: {};
                "#,
                color::RAISIN_BLACK
            ),
            cmp::PageItem {
                style: format!(
                    r#"
                        justify-content: space-between;
                        padding-left: 16px;
                        padding-right: 16px;
                        padding-top: 16px;
                    "#
                ),
                div {
                    style: format!(
                        r#"
                            display: flex;
                            flex-direction: column;
                            justify-content: center;
                            align-items: center;
                            min-width: 100%;
                            max-width: 100%;
                        "#
                    ),
                    cmp::NavbarBuild {}
                }
                div {
                    style: format!(
                        r#"
                            display: flex;
                            flex-direction: column;
                            justify-content: center;
                            align-items: center;
                            min-width: 100%;
                            max-width: 100%;
                            flex: 1;
                        "#
                    ),
                    hero::Hero {}
                }
            }
            cmp::PageItem {
                "Hello World"
            }
        }
    )
}

mod hero {
    use super::*;

    #[component]
    pub fn Hero() -> Element {
        rsx!(
            div {
                style: format!(
                    r#"
                        display: flex;
                        flex-direction: column;
                        justify-content: center;
                        align-items: center;
                        min-width: 100%;
                        max-width: 100%;
                        min-height: 300px;
                        border-width: 1px;
                        border-style: solid;
                        border-color: {};
                        border-radius: 2px;
                        position: relative;
                    "#,
                    color::SILVER
                ),
                cmp::CtaButton {
                    "Create Account"
                }
            }
        )
    }

    #[component]
    pub fn Heading(
        children: Option<Element>
    ) -> Element {
        rsx!(
            h1 {
                style: format!(
                    r#"
                        display: flex;
                        flex-direction: row;
                        justify-content: center;
                        align-items: center;
                        font-family: {};
                        font-weight: normal;
                        color: {};
                    "#,
                    font::BR_COBANE,
                    color::TIMBERWOLF
                ),
                { children }
            }
        )
    }

    #[component]
    pub fn SubHeading(
        children: Option<Element>
    ) -> Element {
        rsx!(
            h2 {
                style: format!(
                    r#"
                        display: flex;
                        flex-direction: row;
                        justify-content: center;
                        align-items: center;
                        font-family: {};
                        font-weight: normal;
                        color: {};
                    "#,
                    font::BR_COBANE,
                    color::SILVER
                ),
                { children }
            }
        )
    }

}

#[component]
fn HeroSectionWithAsset(
    content: Option<Element>,
    image: Option<Element>
) -> Element {
    rsx!(
        div {
            style: format!(
                r#"
                    display: flex;
                    flex-direction: row;
                    justify-content: space-between;
                    align-items: center;
                    padding: 8px;
                "#
            ),
            div {
                style: format!(
                    r#"
                        padding: 8px;
                    "#
                ),
                { image }
            }
            div {
                style: format!(
                    r#"
                        display: flex;
                        flex-direction: column;
                        justify-content: center;
                        align-items: start;
                        padding: 8px;
                    "#
                ),
                hero::Heading { "No gasfees" }
                hero::SubHeading { "Pick your monetization model and pass down full flexibility." }
            }
        }
    )
}

