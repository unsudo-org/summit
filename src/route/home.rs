use super::*;

#[component]
pub fn Home() -> Element {
    let conf: conf::Conf = use_context();

    rsx!(
        cmp::Page {
            scroll_snap: cmp::PageScrollSnap::Proximity,
            color_: conf.color.raisin_black.to_owned(),
            surface: rsx!(
                div {
                    display: "flex",
                    flex_direction: "column",
                    justify_content: "end",
                    align_items: "start",
                    min_width: "100vw",
                    max_width: "100vw",
                    min_height: "100vh",
                    max_height: "100vh",
                    padding_left: "16px",
                    padding_bottom: "16px",
                    button {
                        all: "unset",
                        font_family: conf.font.br_cobane,
                        font_weight: "normal",
                        color: conf.color.timberwolf.to_string(),
                        border_width: "1px",
                        border_style: "solid",
                        border_color: conf.color.mindaro.to_string(),
                        border_radius: "2px",
                        padding: "8px",
                        "<sign_in_sign_up_form>"
                    }
                }
            ),
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
            }
            cmp::PageItem {
                div {
                    display: "flex",
                    flex_direction: "column",
                    justify_content: "space-around",
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
                        position: "relative",
                        gap: "8px",
                        div {
                            display: "flex",
                            flex_direction: "row",
                            justify_content: "space-between",
                            align_items: "end",
                            min_width: "100%",
                            div {
                                font_size: "0.5em",
                                font_family: conf.font.br_cobane,
                                font_weight: "normal",
                                color: conf.color.timberwolf.to_string(),
                                "Something nice to know"
                            }
                            div {
                                font_size: "0.5em",
                                font_family: conf.font.br_cobane,
                                font_weight: "normal",
                                color: conf.color.timberwolf.to_string(),
                                "2025"
                            }
                        }
                        Hero {
                            heading: rsx!("An L1.5 for Polkadot"),
                            sub_heading: rsx!("It's time for a better web3 experience"),
                            left_feature_card_heading: rsx!("..."),
                            left_feature_card_sub_heading: rsx!("..."),
                            right_feature_card_heading: rsx!("..."),
                            right_feature_card_sub_heading: rsx!("...")
                        }
                        div {
                            display: "flex",
                            flex_direction: "column",
                            justify_content: "start",
                            align_items: "start",
                            font_size: "0.5em",
                            font_family: conf.font.br_cobane,
                            font_weight: "normal",
                            color: conf.color.timberwolf.to_string(),
                            "Something that is not the main focus but nice to know"
                        }
                    }
                    div {
                        class: "pulse_flicker",
                        font_size: "2em",
                        font_family: conf.font.br_cobane,
                        font_weight: "normal",
                        color: conf.color.timberwolf.to_string(),
                        padding_top: "64px",
                        "↡"
                    }
                }
            }
            cmp::PageItem {
                div {
                    display: "flex",
                    flex_direction: "row",
                    justify_content: "start",
                    align_items: "center",
                    overflow_x: "auto",
                    overflow_y: "hidden",
                    div {
                        display: "flex",
                        flex_direction: "column",
                        justify_content: "start",
                        align_items: "start",
                        div {
                            font_size: "0.5em",
                            font_family: "br cobane",
                            font_weight: "normal",
                            color: conf.color.timberwolf.to_string(),
                            "Bla bla bla"
                        }
                        h1 {
                            font_family: "Alien Android",
                            font_weight: "normal",
                            color: conf.color.timberwolf.to_string(),
                            "Pillars of Decentralization"
                        }
                        div {
                            display: "flex",
                            flex_direction: "row",
                            justify_content: "end",
                            min_width: "100%",
                            div {
                                font_size: "0.5em",
                                font_family: "br cobane",
                                font_weight: "normal",
                                color: conf.color.timberwolf.to_string(),
                                "Bla bla bla"
                            }
                        }
                    }
                    div {
                        display: "flex",
                        flex_direction: "column",
                        justify_content: "start",
                        align_items: "start",

                    }
                }
            }
        }
    )
}

#[component]
fn Hero(
    heading: Option<Element>,
    sub_heading: Option<Element>,
    left_feature_card_heading: Option<Element>,
    left_feature_card_sub_heading: Option<Element>,
    right_feature_card_heading: Option<Element>,
    right_feature_card_sub_heading: Option<Element>
) -> Element {
    let conf: conf::Conf = use_context();

    rsx!(
        div {
            display: "flex",
            flex_direction: "column",
            justify_content: "start",
            align_items: "start",
            gap: "8px",
            div {
                display: "flex",
                flex_direction: "row",
                justify_content: "space-between",
                align_items: "start",
                min_width: "100%",
                background: conf.color.timberwolf.to_string(),
                border_radius: "2px",
                padding: "16px",
                gap: "64px",
                max_width: "1440px",
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
                        h1 {
                            font_family: "Alien Android",
                            font_weight: "bold",
                            color: conf.color.raisin_black.to_string(),
                            { heading }
                        }
                        p {
                            font_family: conf.font.br_cobane,
                            font_weight: "normal",
                            color: conf.color.raisin_black.to_string(),
                            { sub_heading }
                        }
                    }
                    div {
                        display: "flex",
                        flex_direction: "row",
                        justify_content: "center",
                        align_items: "center",
                        gap: "8px",
                        padding_top: "16px",
                        padding_bottom: "16px",
                        cmp::Button { "Explore" }
                        cmp::ButtonDashed { "Learn More" }
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
                        color_: Some(conf.color.raisin_black.to_owned()),
                        model: cmp::ShapeModel::LayeredStructure
                    }
                }
            }
            div {
                display: "flex",
                flex_direction: "row",
                justify_content: "space-between",
                align_items: "start",
                gap: "8px",
                HeroFeatureCard {
                    heading: "24/7 Uptime",
                    sub_heading: "Build on infrastructure that never sleeps"
                }
                HeroFeatureCard {
                    heading: "Gassless Economy",
                    sub_heading: "Say goodbye to gas fees"
                }
            }
        }
    )
}

#[component]
fn HeroFeatureCard(heading: String, sub_heading: String) -> Element {
    let conf: conf::Conf = use_context();

    rsx!(
        div {
            display: "flex",
            flex_direction: "column",
            justify_content: "start",
            align_items: "start",
            padding: "16px",
            border_width: "1px",
            border_style: "solid",
            border_color: conf.color.timberwolf.to_string(),
            border_radius: "2px",
            min_width: "400px",
            max_width: "400px",
            h1 {
                font_family: "Alien Android",
                font_weight: "bold",
                color: conf.color.timberwolf.to_string(),
                { heading }
            }
            p {
                font_family: conf.font.br_cobane,
                font_weight: "normal",
                color: conf.color.timberwolf.to_string(),
                { sub_heading }
            }
        }
    )
}

