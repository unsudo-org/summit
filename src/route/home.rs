use dioxus::html::g::{font_family, font_weight};

use super::*;

#[component]
pub fn Home() -> Element {
    let conf: conf::Conf = use_context();

    rsx!(
        cmp::Page {
            scroll_snap: cmp::PageScrollSnap::Proximity,
            bg: conf.color.raisin_black.to_owned(),
            cmp::PageItem {
                div {
                    display: "flex",
                    flex_direction: "column",
                    justify_content: "space-between",
                    align_items: "start",
                    min_width: "100vw",
                    max_width: "100vw",
                    min_height: "100vh",
                    max_height: "100vh",
                    div {
                        display: "flex",
                        flex_direction: "row",
                        
                        cmp::NavbarBuild {}
                    }
                    div {
                        min_height: "32px"
                    }
                    div {
                        display: "flex",
                        flex_direction: "row",
                        justify_content: "center",
                        align_items: "center",
                        min_width: "100%",
                        max_width: "100%",
                        Banner {}
                    }
                    div {
                        flex: "1"
                    }
                    div {
                        display: "flex",
                        flex_direction: "row",
                        justify_content: "center",
                        align_items: "center",
                        min_width: "100vw",
                        max_width: "100vw",
                        padding: "16px",
                        background: conf.color.medium_slate_blue.to_string(),
                        Routing {}
                    }
                    div {
                        flex: "1"
                    }
                    div {
                        display: "flex",
                        flex_direction: "row",
                        justify_content: "center",
                        align_items: "center",
                        min_width: "100%",
                        max_width: "100%",
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
                    div {
                        min_height: "8px"
                    }
                }
            }
            cmp::PageItem {
                div {
                    display: "flex",
                    flex_direction: "column",
                    justify_content: "space-between",
                    align_items: "start",
                    min_width: "100vw",
                    max_width: "100vw",
                    min_height: "100vh",
                    max_height: "100vh",
                    div {
                        display: "flex",
                        flex_direction: "row",
                        justify_content: "space-between",
                        align_items: "center",
                        min_width: "100%",
                        max_width: "100%",
                        min_height: "100%",
                        max_height: "100%",
                        padding: "16px",

                        div {
                            display: "flex",
                            flex_direction: "column",
                            justify_content: "start",
                            align_items: "start",
                            h2 {
                                display: "flex",
                                flex_direction: "row",
                                font_family: conf.font.brulia_test,
                                font_weight: "normal",
                                color: conf.color.timberwolf.to_string(),
                                gap: "4px",
                                span {
                                    cmp::Counter {
                                        from: 0.0,
                                        to: 9.0,
                                        ms: 3000.0
                                    }
                                }
                            }
                            h2 {
                                font_family: conf.font.borneox,
                                font_weight: "normal",
                                color: conf.color.timberwolf.to_string(),
                                "Billion People"
                            }
                        }

                    }
                    cmp::HazardStripe {
                        min_w: "100vw",
                        max_w: "100vw",
                        min_h: "32px",
                        max_h: "32px",
                        color_0: conf.color.rose_pompadour.to_owned(),
                        color_1: conf.color.rose_pompadour.to_owned(),
                        color_2: conf.color.raisin_black.to_owned(),
                        color_3: conf.color.raisin_black.to_owned(),
                        size_0: 0,
                        size_1: 20,
                        size_2: 0,
                        size_3: 32,
                        animation_speed_seconds: 64
                    }
                    div {
                        display: "flex",
                        flex_direction: "row",
                        justify_content: "space-between",
                        align_items: "center",
                        min_width: "100%",
                        max_width: "100%",
                        padding: "16px",
                        div {
                            display: "flex",
                            flex: "1",
                            h4 {
                                display: "flex",
                                min_width: "500px",
                                max_width: "500px",
                                font_family: conf.font.brulia_test,
                                font_weight: "normal",
                                color: conf.color.timberwolf.to_string(),
                                "Web3 is in trouble, our digital town squares are being attacked. We need to do better... We are weakened by red tape and corruption."
                            }
                        }
                        cmp::Shape {
                            w: "258px",
                            h: "258px",
                            fill: conf.color.timberwolf.to_owned(),
                            model: cmp::ShapeModel::AngularStar
                        }
                        div {
                            flex: "1"
                        }
                    }
                    cmp::HazardStripe {
                        min_w: "100vw",
                        max_w: "100vw",
                        min_h: "32px",
                        max_h: "32px",
                        color_0: conf.color.rose_pompadour.to_owned(),
                        color_1: conf.color.rose_pompadour.to_owned(),
                        color_2: conf.color.raisin_black.to_owned(),
                        color_3: conf.color.raisin_black.to_owned(),
                        size_0: 0,
                        size_1: 20,
                        size_2: 0,
                        size_3: 32,
                        animation_speed_seconds: 64
                    }
                    div {

                    }
                }
            }
        }
    )
}

#[derive(Debug)]
#[derive(Clone)]
#[derive(Copy)]
#[derive(PartialEq)]
#[derive(Eq)]
#[derive(Default)]
enum BannerState {
    #[default]
    Spawn,
    Idle,
    Explore,
    LearnMore
}

#[component]
fn Banner() -> Element {
    let conf: conf::Conf = use_context();
    let mut state: Signal<_> = use_signal(BannerState::default);

    use_effect(move || {
        *state.write() = BannerState::Idle;
    });

    rsx!(
        div {
            display: "flex",
            flex_direction: "column",
            justify_content: "start",
            align_items: "start",
            gap: "8px",
            div {
                onmouseleave: move |_| *state.write() = BannerState::Idle,
                z_index: "1",
                display: "flex",
                flex_direction: "row",
                justify_content: "space-between",
                align_items: "start",
                padding: "16px",
                overflow_x: "hidden",
                overflow_y: "hidden",
                min_width: "100%",
                border_radius: "2px",
                background: conf.color.timberwolf.to_string(),
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
                        font_family: conf.font.borneox,
                        font_weight: "normal",
                        color: conf.color.raisin_black.to_string(),
                        min_height: "100%",
                        max_width: "256px",
                        text_align: "left",
                        word_wrap: "break-word",
                        transition: "transform 1s, opacity 1s, display 1s 4s",
                        transform: if let BannerState::Idle = *state.read() {
                            "translate(0, 0)"
                        } else {
                            "translate(-200%, 0)"
                        },
                        opacity: if let BannerState::Idle = *state.read() {
                            "1"
                        } else {
                            "0"
                        },
                        "A Layer 1.5 For Polkadot"
                    }
                }
                div {
                    min_width: "64px"
                }
                div {
                    display: "flex",
                    flex_direction: "row",
                    justify_content: "end",
                    align_items: "end",
                    transition: "all 1s",
                    transform: match *state.read() {
                        BannerState::Idle => "translate(0, 0)",
                        _ => "translate(500px, 0)"
                    },
                    div {
                        class: "float",
                        display: "flex",
                        flex_direction: "column",
                        justify_content: "start",
                        align_items: "start",
                        max_width: "256px",
                        h3 {
                            font_family: conf.font.brulia_test,
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
                            div {
                                onclick: move |_| *state.write() = BannerState::Explore,
                                cmp::Button {
                                    "explore"
                                }
                            }
                            cmp::Button {
                                "learn more"
                            }
                        }
                    }
                    div {
                        display: "flex",
                        flex_direction: "row",
                        justify_content: "start",
                        align_items: "start",
                        cmp::Shape {
                            w: "200px",
                            h: "auto",
                            fill: conf.color.raisin_black.to_owned(),
                            model: cmp::ShapeModel::FourLobedRoundedSquare
                        }
                    }
                }
                div {
                    position: "absolute",
                    display: "flex",
                    flex_direction: "column",
                    justify_content: "start",
                    align_items: "start",
                    min_width: "100%",
                    min_height: "100%",
                    pointer_events: if let BannerState::Explore = *state.read() {
                        "auto"
                    } else {
                        "none"
                    },
                    transition: "transform 1s",
                    transform: if let BannerState::Explore = *state.read() {
                        "translate(0, 0)"
                    } else {
                        "translate(0, -100vh)"
                    },
                    div {
                        class: "float",
                        display: "flex",
                        flex_direction: "column",
                        justify_content: "start",
                        align_items: "start",
                        max_width: "256px",
                        h3 {
                            font_family: conf.font.brulia_test,
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
                            div {
                                onclick: move |_| *state.write() = BannerState::Idle,
                                cmp::Button {
                                    "not ready yet"
                                }
                            }
                            cmp::Button {
                                "learn more"
                            }
                        }
                    }
                }
            }
            div {
                display: "flex",
                flex_direction: "row",
                justify_content: "start",
                align_items: "center",
                gap: "8px",
                transition: "transform 1s, opacity 1s",
                transform: if let BannerState::Idle = *state.read() {
                    "translate(0, 0)"
                } else {
                    "translate(0, -250px)"
                },
                opacity: if let BannerState::Idle = *state.read() {
                    "1"
                } else {
                    "0"
                },
                Feature {
                    "Transparency"
                }
                Feature {
                    "Resilience"
                }
                Feature {
                    "24/7 Uptime"
                }
                Feature {
                    "Cross Border"
                }
            }
        }
    )
}




// MARK: R

#[derive(Default)]
enum RouteState {
    #[default]
    Spawn,
    Idle
}

#[component]
fn Routing() -> Element {
    let conf: conf::Conf = use_context();
    let mut state: Signal<_> = use_signal(RouteState::default);

    use_effect(move || {
        *state.write() = RouteState::Idle;
    });

    rsx!(
        div {
            display: "flex",
            flex_direction: "row",
            justify_content: "center",
            align_items: "center",
            min_width: "100vw",
            max_width: "100vw",
            padding: "16px",
            background: conf.color.medium_slate_blue.to_string(),
            div {
                class: "float",
                display: "flex",
                flex_direction: "row",
                justify_content: "center",
                align_items: "center",
                min_width: "100%",
                max_width: "100%",
                min_height: "100%",
                max_height: "100%",
                gap: "8px",
                transition: "transform 1s",
                transform: if let RouteState::Idle = *state.read() {
                    "translate(0, 0)"
                } else {
                    "translate(-100vw, 0)"
                },
                cmp::Spacer {
                    min_w: "2px",
                    max_w: "2px",
                    min_h: "128px",
                    max_h: "128px",
                    fill: conf.color.raisin_black.to_owned()
                }
                GuideCard {
                    label: "Build",
                    description: "Write your synth using rust compile to WASM.",
                    label_button: "build"
                }
                cmp::Spacer {
                    min_w: "2px",
                    max_w: "2px",
                    min_h: "128px",
                    max_h: "128px",
                    fill: conf.color.raisin_black.to_owned()
                }
                GuideCard {
                    label: "Deployment",
                    description: "Start deploying",
                    label_button: "deploy"
                }
                cmp::Spacer {
                    min_w: "2px",
                    max_w: "2px",
                    min_h: "128px",
                    max_h: "128px",
                    fill: conf.color.raisin_black.to_owned()
                }
                GuideCard {
                    label: "Maintainance",
                    description: "Start deploying",
                    label_button: "maintain"
                }
                cmp::Spacer {
                    min_w: "2px",
                    max_w: "2px",
                    min_h: "128px",
                    max_h: "128px",
                    fill: conf.color.raisin_black.to_owned()
                }
                GuideCard {
                    label: "Funding",
                    description: "Start deploying",
                    label_button: "fund"
                }
            }
        }
    )
}

#[component]
fn GuideCard(
    label: String,
    label_button: String,
    description: String,
) -> Element {
    let conf: conf::Conf = use_context();

    rsx!(
        div {
            display: "flex",
            flex_direction: "column",
            padding: "16px",
            min_width: "300px",
            max_width: "300px",
            min_height: "100%",
            div {
                cmp::Shape {
                    w: "16px",
                    h: "auto",
                    fill: conf.color.raisin_black.to_owned(),
                    model: cmp::ShapeModel::FourLobedFlower
                }
            }
            div {
                min_height: "2px"
            }
            div {
                display: "flex",
                flex_direction: "column",
                justify_content: "start",
                align_items: "start",
                h4 {
                    font_family: conf.font.borneox,
                    font_weight: "normal",
                    color: conf.color.raisin_black.to_string(),
                    { label }
                }
                div {
                    min_height: "2px"
                }
                h4 {
                    font_family: conf.font.brulia_test,
                    font_weight: "normal",
                    color: conf.color.raisin_black.to_string(),
                    text_align: "start",
                    { description }
                }
            }
            div {
                min_height: "16px"
            }
            div {
                cmp::Button {
                    { label_button }
                }
            }
        }
    )
}

#[component]
fn Feature(children: Element) -> Element {
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
            h3 {
                display: "flex",
                flex_direction: "row",
                justify_content: "center",
                align_items: "center",
                font_family: conf.font.borneox,
                font_weight: "bold",
                color: conf.color.timberwolf.to_string(),
                { children }
            }
        }
    )
}