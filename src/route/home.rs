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
                    div {
                        style: format!(
                            r#"
                                min-width: 100%;
                                min-height: 1px;
                                background: linear-gradient(
                                    to right,
                                    {},
                                    {},
                                    {}
                                );
                            "#,
                            color::MEDIUM_SLATE_BLUE,
                            color::ROSE_POMPADOUR,
                            color::MINDARO
                        )
                    }
                    div {
                        style: format!(
                            r#"
                                padding: 4px;
                            "#
                        )
                    }
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
                        flex-direction: row;
                        justify-content: center;
                        align-items: center;
                        gap: 16px;
                    "#
                ),
                div {
                    style: format!(
                        r#"
                            display: flex;
                            flex-direction: column;
                            justify-content: center;
                            align-items: start;
                            min-height: 300px;
                            padding-left: 16px;
                            padding-right: 16px;
                        "#
                    ),
                    div {
                        style: format!(
                            r#"
                                display: flex;
                                flex-direction: column;
                                justify-content: center;
                                align-items: start;
                                padding-bottom: 16px;
                                gap: 8px;
                            "#
                        ),
                        Heading {
                            "A layer 1.5 built on top of Polkadot JAM"
                        }
                        SubHeading {
                            "Its time to move to a safer, faster, and more flexible ecosystem."
                        }
                    }
                    div {
                        style: format!(
                            r#"
                                display: flex;
                                flex-direction: row;
                                gap: 8px
                            "#
                        ),
                        cmp::CtaButton {
                            "Create Account"
                        }
                        cmp::Button {
                            "Learn More"
                        }
                    }
                }
                cmp::Shape {
                    w: 200.0,
                    h: 200.0,
                    model: cmp::ShapeModel::random(),
                    color: color::SILVER
                }
            }
            div {
                display: "flex",
                flex_direction: "row",
                gap: "16px",
                card::Card {
                    heading: rsx!(
                        "Technical Complexity"
                    ),
                    sub_heading: rsx!(
                        "Build scalable interoperable decentralized software."
                    )
                }
                card::Card {
                    heading: rsx!(
                        "Infrastructure Overhead"
                    ),
                    sub_heading: rsx!(
                        "Run and maintain compute and build autonomous software."
                    )
                }
                card::Card {
                    heading: rsx!(
                        "Cost and Resource Uncertainty"
                    ),
                    sub_heading: rsx!(
                        "Infrastructure and deployment costs can fluctuate unpredictably."
                    )
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

mod card {
    use super::*;

    #[component]
    pub fn Card(
        heading: Option<Element>,
        sub_heading: Option<Element>
    ) -> Element {
        rsx!(
            div {
                display: "flex",
                flex_direction: "column",
                justify_content: "start",
                align_items: "start",
                max_width: "500px",
                div {
                    display: "flex",
                    flex_direction: "row",
                    justify_content: "start",
                    align_items: "center",
                    padding_bottom: "8px",
                    gap: "8px",
                    div {
                        cmp::Shape {
                            w: 16.0,
                            h: 16.0,
                            color: color::TIMBERWOLF,
                            model: cmp::ShapeModel::random()
                        }
                    }
                    h4 {
                        display: "flex",
                        flex_direction: "row",
                        justify_content: "start",
                        align_items: "center",
                        font_family: font::BR_COBANE,
                        font_weight: "bold",
                        color: color::TIMBERWOLF,
                        padding_bottom: "4px",
                        { heading }
                    }
                }
                p {
                    display: "flex",
                    flex_direction: "row",
                    justify_content: "start",
                    align_items: "center",
                    font_family: font::BR_COBANE,
                    font_weight: "normal",
                    color: color::SILVER,
                    { sub_heading }
                }
            }
        )
    }
}