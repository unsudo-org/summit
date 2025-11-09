use super::*;

#[component]
pub fn Home() -> Element {
    rsx!(
        cmp::Page {
            scroll_snap: cmp::PageScrollSnap::Proximity,
            color: color::RAISIN_BLACK,
            cmp::PageItem {
                div {
                    display: "flex",
                    flex_direction: "column",
                    justify_content: "center",
                    align_items: "center",
                    min_width: "100%",
                    max_width: "100%",
                    padding_top: "4px",
                    padding_bottom: "4px",
                    cmp::NavbarBuild {}
                }
                div {
                    display: "flex",
                    flex_direction: "column",
                    justify_content: "start",
                    align_items: "center",
                    min_width: "100%",
                    max_width: "100%",
                    flex: "1",
                    "h"
                }
            }
            cmp::PageItem {
                div {
                    display: "flex",
                    flex_direction: "column",
                    justify_content: "center",
                    align_items: "center",
                    min_width: "100vw",
                    max_width: "100vw",
                    min_height: "100vh",
                    max_height: "100vh",
                    div {
                        display: "flex",
                        flex_direction: "column",
                        justify_content: "start",
                        align_items: "start",
                        gap: "8px",
                        div {
                            display: "flex",
                            flex_direction: "row",
                            justify_content: "start",
                            align_items: "start",
                            background: color::TIMBERWOLF,
                            border_radius: "2px",
                            padding: "16px",
                            gap: "32px",
                            div {
                                display: "flex",
                                flex_direction: "column",
                                justify_content: "start",
                                align_items: "start",
                                div {
                                    all: "unset",
                                    display: "flex",
                                    flex_direction: "column",
                                    justify_content: "start",
                                    align_items: "start",
                                    margin_bottom: "16px",
                                    h1 {
                                        font_family: font::BR_COBANE,
                                        font_weight: "bold",
                                        color: color::RAISIN_BLACK,
                                        "Some catchy heading"
                                    }
                                    p {
                                        font_family: font::BR_COBANE,
                                        font_weight: "normal",
                                        color: color::RAISIN_BLACK,
                                        "Some vey nice catchy phrase or body describing something very important."
                                    }
                                }
                                div {
                                    display: "flex",
                                    flex_direction: "row",
                                    justify_content: "center",
                                    align_items: "center",
                                    gap: "8px",
                                    button {
                                        all: "unset",
                                        border_width: "1px",
                                        border_style: "solid",
                                        border_image: color::RAISIN_BLACK,
                                        border_radius: "2px",
                                        padding: "8px",
                                        font_family: font::BR_COBANE,
                                        font_weight: "normal",
                                        "Explore"
                                    }
                                    button {
                                        all: "unset",
                                        border_width: "1px",
                                        border_style: "dashed",
                                        border_color: color::RAISIN_BLACK,
                                        border_radius: "2px",
                                        padding: "8px",
                                        font_family: font::BR_COBANE,
                                        font_weight: "normal",
                                        "Learn More"
                                    }
                                }
                            }
                            div {
                                display: "flex",
                                flex_direction: "column",
                                justify_content: "center",
                                align_items: "center",
                                cmp::Shape {
                                    w: 200.0,
                                    h: 200.0,
                                    color: color::RAISIN_BLACK,
                                    model: cmp::ShapeModel::random()
                                }
                            }
                        }
                        div {
                            display: "flex",
                            flex_direction: "row",
                            justify_content: "space-between",
                            align_items: "start",
                            min_width: "100%",
                            gap: "8px",
                            div {
                                display: "flex",
                                flex_direction: "column",
                                justify_content: "start",
                                align_items: "start",
                                padding: "16px",
                                flex: "1",
                                border_width: "1px",
                                border_style: "solid",
                                border_color: color::TIMBERWOLF,
                                border_radius: "2px",
                                h1 {
                                    font_family: font::BR_COBANE,
                                    font_weight: "bold",
                                    color: color::TIMBERWOLF,
                                    "Blazingly fast"
                                }
                                p {
                                    font_family: font::BR_COBANE,
                                    font_weight: "normal",
                                    color: color::TIMBERWOLF,
                                    "Bla bla bla bla"
                                }
                            }
                            div {
                                display: "flex",
                                flex_direction: "column",
                                justify_content: "start",
                                align_items: "start",
                                padding: "16px",
                                flex: "1",
                                border_width: "1px",
                                border_style: "solid",
                                border_color: color::TIMBERWOLF,
                                border_radius: "2px",
                                h1 {
                                    font_family: font::BR_COBANE,
                                    font_weight: "bold",
                                    color: color::TIMBERWOLF,
                                    "Blazingly fast"
                                }
                                p {
                                    font_family: font::BR_COBANE,
                                    font_weight: "normal",
                                    color: color::TIMBERWOLF,
                                    "Bla bla bla bla"
                                }
                            }
                        }
                    }
                }
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