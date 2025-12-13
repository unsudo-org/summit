use dioxus::html::u::flex_direction;

use super::*;

#[component]
pub fn Home() -> Element {
    let conf: conf::Conf = use_context();

    rsx!(
        cmp::Page {
            scroll_snap: cmp::PageScrollSnap::Proximity,
            bg_color: conf.color.raisin_black.to_owned(),
            surface: rsx!(),
            cmp::PageItem {
                div {
                    display: "flex",
                    flex_direction: "column",
                    justify_content: "start",
                    align_items: "start",
                    min_width: "100vw",
                    max_width: "100vw",
                    min_height: "100vh",
                    max_height: "100vh",
                    div {
                        cmp::NavbarBuild {}
                    }
                    div {
                        display: "flex",
                        flex_direction: "row",
                        justify_content: "center",
                        align_items: "center",
                        min_width: "100%",
                        max_width: "100%",
                        flex: "1",
                        HeroSection {}
                    }
                    div {
                        display: "flex",
                        flex_direction: "row",
                        justify_content: "center",
                        align_items: "center",
                        min_width: "100%",
                        max_width: "100%",
                        flex: "1",
                        div {
                            class: "soft_flicker",
                            display: "flex",
                            flex_direction: "column",
                            justify_content: "center",
                            align_items: "center",
                            font_size: "32px",
                            font_family: conf.font.br_cobane,
                            font_weight: "normal",
                            color: conf.color.timberwolf.to_string(),
                            "↡"
                        }
                    }
                }
            }
            cmp::PageItem {
                div {
                    display: "flex",
                    flex_direction: "column",
                    justify_content: "start",
                    align_items: "start",
                    min_width: "100vw",
                    max_width: "100vw",
                    min_height: "100vh",
                    max_height: "100vh",
                    padding: "32px",

                    div {
                        display: "flex",
                        flex_direction: "row",
                        justify_content: "start",
                        align_items: "center",
                        div {
                            display: "flex",
                            background: conf.color.timberwolf.to_string(),
                            padding: "8px",
                            font_size: "128px",
                            font_family: "alien android",
                            font_weight: "normal",
                            color: conf.color.raisin_black.to_string(),
                            border_radius: "4px",
                            min_width: "100%",
                            "Web3 for Web2"
                        }
                    }
                    div {
                        display: "flex",
                        flex_direction: "row",
                        justify_content: "center",
                        align_items: "center",
                        flex: "1",
                        min_width: "100%",
                        div {
                            display: "flex",
                            flex_direction: "row",
                            justify_content: "center",
                            align_items: "center",
                            font_size: "128px",
                            font_family: conf.font.br_cobane,
                            font_weight: "normal",
                            color: conf.color.timberwolf.to_string(),

                        }
                    }
                        cmp::Shape {
                            w: "256px",
                            h: "256px",
                            fill: conf.color.timberwolf.to_owned(),
                            model: cmp::ShapeModel::RoundedTab
                        }
                    div {
                        display: "flex",
                        flex_direction: "row",
                        justify_content: "start",
                        align_items: "start",
                        gap: "64px",

                        div {
                            min_height: "16px"
                        }
                        div {
                            font_size: "16px",
                            font_family: conf.font.br_cobane,
                            font_weight: "normal",
                            color: conf.color.timberwolf.to_string(),
                            text_align: "justify",
                            max_width: "400px",
                            "While Web3 promises decentralization, 17% of it is driven purely by speculation—focused on hype and short-term gains rather than real utility. This imbalance slows meaningful adoption and distracts from building products people actually use."
                        }
                        div {
                            font_size: "16px",
                            font_family: conf.font.br_cobane,
                            font_weight: "normal",
                            color: conf.color.timberwolf.to_string(),
                            text_align: "justify",
                            max_width: "400px",
                            "Web3 aims to create real ownership and value, yet 17% of the space is dominated by speculation. This focus on hype over utility limits trust, usability, and long-term impact."
                        }
                    }
                }
            }
            cmp::PageItem {
                div {
                    display: "flex",
                    flex_direction: "column",
                    justify_content: "start",
                    align_items: "start",
                    min_width: "100vw",
                    max_width: "100vw",
                    min_height: "100vh",
                    max_height: "100vh",
                    padding: "32px",
                    div {
                        display: "flex",
                        flex_direction: "row",
                        justify_content: "start",
                        align_items: "start",
                        gap: "20px",
                        Card {
                            label: rsx!(
                                "Why Us"
                            ),
                            div {
                                display: "flex",
                                flex_direction: "column",
                                justify_content: "start",
                                align_items: "start",
                                gap: "16px",
                                BulletPoint { "Onchain Infrastructure" }
                                BulletPoint { "Robust" }
                                BulletPoint { "Modularity" }
                            }
                        }
                        Card {
                            label: rsx!(
                                "Why It Matters"
                            ),
                            div {
                                display: "flex",
                                flex_direction: "column",
                                justify_content: "start",
                                align_items: "start",
                                gap: "16px",
                                BulletPoint { "Onchain Infrastructure" }
                                BulletPoint { "Robust" }
                                BulletPoint { "Modularity" }
                            }
                        }
                        Card {
                            label: rsx!(
                                "What can you do?"
                            ),
                            div {
                                display: "flex",
                                flex_direction: "column",
                                justify_content: "start",
                                align_items: "start",
                                gap: "16px",
                                BulletPoint { "Onchain Infrastructure" }
                                BulletPoint { "Robust" }
                                BulletPoint { "Modularity" }
                            }
                        }
                    }
                }
            }
        }
    )
}

#[component]
fn HeroSection() -> Element {
    let conf: conf::Conf = use_context();

    rsx!(
        div {
            display: "flex",
            flex_direction: "column",
            justify_content: "start",
            align_items: "start",
            HeroSectionBanner {}
            div {
                min_height: "8px"
            }
            div {
                display: "flex",
                flex_direction: "row",
                gap: "8px",
                min_width: "100%",
                HeroSectionBannerFeature {
                    heading: "TRANSPARENCY"
                }
                HeroSectionBannerFeature {
                    heading: "RESILIENCE"
                }
                HeroSectionBannerFeature {
                    heading: "24/7 UPTIME"
                }
                HeroSectionBannerFeature {
                    heading: "CROSS BORDER"
                }
            }
            div {
                display: "flex",
                flex_direction: "row",
                font_size: "1em",
                font_family: "",
                font_weight: "normal",
                color: conf.color.timberwolf.to_string(),
                "bla bla bla"
            }
        }
    )
}

#[component]
fn HeroSectionBanner() -> Element {
    let conf: conf::Conf = use_context();

    rsx!(
        div {
            display: "flex",
            flex_direction: "row",
            justify_content: "space-between",
            align_items: "start",
            border_radius: "2px",
            background: conf.color.timberwolf.to_string(),
            padding_top: "16px",
            padding_bottom: "0px",
            padding_left: "16px",
            padding_right: "16px",
            min_width: "100%",
            div {
                display: "flex",
                flex_direction: "column",
                justify_content: "end",
                align_items: "start",
                HeroSectionBannerHeading {
                    "AN L1.5 FOR POLKADOT"
                }
            }
            div {
                min_width: "200px"
            }
            div {
                display: "flex",
                flex_direction: "row",
                justify_content: "end",
                align_items: "end",
                HeroSectionBannerSubHeading {}
                cmp::Shape {
                    w: "200px",
                    h: "auto",
                    fill: Some(conf.color.raisin_black.to_owned()),
                    model: cmp::ShapeModel::FourLobedRoundedSquare
                }
            }
        }
    )
}

#[component]
fn HeroSectionBannerHeading(children: Element) -> Element {
    let conf: conf::Conf = use_context();

    rsx!(
        h1 {
            display: "flex",
            flex_direction: "column",
            justify_content: "start",
            align_items: "start",
            font_size: "4em",
            font_family: "alien android",
            font_weight: "normal",
            color: conf.color.raisin_black.to_string(),
            max_width: "350px",
            text_align: "left",
            word_wrap: "break-word",
            { children }
        }
    )
}

#[component]
fn HeroSectionBannerSubHeading() -> Element {
    let conf: conf::Conf = use_context();

    rsx!(
        div {
            class: "float",
            display: "flex",
            flex_direction: "column",
            justify_content: "start",
            align_items: "start",
            max_width: "250px",
            h2 {
                font_family: "br cobane",
                font_weight: "normal",
                color: conf.color.raisin_black.to_string(),
                "It's time for a better web3 experience"
            }
            div {
                min_height: "16px"
            }
            div {
                display: "flex",
                flex_direction: "row",
                justify_content: "center",
                align_items: "start",
                gap: "8px",
                cmp::Button { "explore" }
                cmp::Button { "learn more" }
            }
        }
    )
}

#[component]
fn HeroSectionBannerFeature(heading: String) -> Element {
    let conf: conf::Conf = use_context();

    rsx!(
        div {
            display: "flex",
            flex_direction: "column",
            justify_content: "start",
            align_items: "start",
            padding: "16px",
            border_width: "2px",
            border_style: "solid",
            border_color: conf.color.timberwolf.to_string(),
            border_radius: "2px",
            h1 {
                display: "flex",
                flex_direction: "row",
                justify_content: "center",
                align_items: "center",
                font_family: "alien android",
                font_weight: "bold",
                color: conf.color.timberwolf.to_string(),
                { heading }
            }
        }
    )
}

#[component]
fn HeroSectionSoftFlickerDownArrow() -> Element {
    let conf: conf::Conf = use_context();

    rsx!(
        div {
            class: "soft_flicker",
            display: "flex",
            flex_direction: "column",
            justify_content: "center",
            align_items: "center",
            font_size: "32px",
            font_family: conf.font.br_cobane,
            font_weight: "normal",
            color: conf.color.timberwolf.to_string(),
            "↡"
        }
    )
}




#[component]
fn Card(label: Option<Element>, children: Option<Element>) -> Element {
    let conf: conf::Conf = use_context();

    rsx!(
        div {
            display: "flex",
            flex_direction: "column",
            justify_content: "start",
            align_items: "start",
            border_width: "2px",
            border_style: "solid",
            border_color: conf.color.timberwolf.to_string(),
            border_radius: "4px",
            div {
                display: "flex",
                flex_direction: "row",
                justify_content: "center",
                align_items: "center",
                background: conf.color.timberwolf.to_string(),
                min_width: "100%",
                max_width: "100%",
                padding: "8px",
                div {
                    display: "flex",
                    flex_direction: "row",
                    justify_content: "center",
                    align_items: "center",
                    min_width: "100%",
                    font_size: "32px",
                    font_family: conf.font.br_cobane,
                    font_weight: "normal",
                    color: conf.color.raisin_black.to_string(),
                    { label }
                }
            }
            div {
                display: "flex",
                flex_direction: "column",
                justify_content: "start",
                align_items: "start",
                padding: "8px",
                { children }
            }
        }
    )
}





#[component]
fn BulletPoint(
    children: Option<Element>
) -> Element {
    let conf: conf::Conf = use_context();

    rsx!(
        div {
            display: "flex",
            flex_direction: "row",
            justify_content: "start",
            align_items: "center",
            gap: "16px",
            cmp::Shape {
                w: "32px",
                h: "32px",
                model: cmp::ShapeModel::QuatrefoilFlower,
                fill: conf.color.timberwolf.to_owned()
            }
            div {
                display: "flex",
                flex_direction: "row",
                justify_content: "start",
                align_items: "center",
                font_size: "2em",
                font_family: conf.font.br_cobane,
                font_weight: "normal",
                color: conf.color.timberwolf.to_string(),
                { children }
            }
        }
    )
}






