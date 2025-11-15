use dioxus::html::u::{max_width, min_width, text_align};

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
                    Hero {}
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
                            font_family: "alien android",
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
fn Hero() -> Element {
    let conf: conf::Conf = use_context();

    rsx!(
        div {
            display: "flex",
            flex_direction: "column",
            justify_content: "start",
            align_items: "start",
            HeroBanner {
                heading: "AN L1.5 FOR POLKADOT"
            }
        }
    )
}

#[component]
fn HeroBanner(heading: String) -> Element {
    let conf: conf::Conf = use_context();

    rsx!(
        div {
            display: "flex",
            flex_direction: "row",
            justify_content: "start",
            align_items: "start",
            border_radius: "2px",
            background: conf.color.timberwolf.to_string(),
            padding: "16px",
            div {
                display: "flex",
                flex_direction: "column",
                justify_content: "end",
                align_items: "start",
                h1 {
                    display: "flex",
                    flex_direction: "column",
                    justify_content: "start",
                    align_items: "start",
                    font_size: "4em",
                    font_family: "br cobane",
                    font_weight: "normal",
                    color: conf.color.raisin_black.to_string(),
                    max_width: "350px",
                    text_align: "left",
                    word_wrap: "break-word",
                    { heading }
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
                HeroSubHeading {}
                cmp::Shape {
                    w: "200px",
                    h: "auto",
                    color_: Some(conf.color.raisin_black.to_owned()),
                    model: cmp::ShapeModel::LayeredStructure
                }
            }
        }
    )
}

#[component]
fn HeroSubHeading() -> Element {
    let conf: conf::Conf = use_context();

    rsx!(
        div {
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
fn HeroFeatureCard(heading: String, sub_heading: String) -> Element {
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
                font_family: "Borneox",
                font_weight: "bold",
                color: conf.color.timberwolf.to_string(),
                { heading }
            }
            
        }
    )
}

