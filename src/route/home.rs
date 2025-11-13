use super::*;

#[component]
pub fn Home() -> Element {
    let conf: conf::Conf = use_context();

    rsx!(
        cmp::Page {
            scroll_snap: cmp::PageScrollSnap::Proximity,
            color_: conf.color.raisin_black.to_owned(),
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
                            background: conf.color.timberwolf.to_string(),
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
                                        font_family: conf.font.br_cobane,
                                        font_weight: "bold",
                                        color: conf.color.raisin_black.to_string(),
                                        "Some catchy heading"
                                    }
                                    p {
                                        font_family: conf.font.br_cobane,
                                        font_weight: "normal",
                                        color: conf.color.raisin_black.to_string(),
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
                                        border_image: conf.color.raisin_black.to_string(),
                                        border_radius: "2px",
                                        padding: "8px",
                                        font_family: conf.font.br_cobane,
                                        font_weight: "normal",
                                        "Explore"
                                    }
                                    button {
                                        all: "unset",
                                        border_width: "1px",
                                        border_style: "dashed",
                                        border_color: conf.color.raisin_black.to_string(),
                                        border_radius: "2px",
                                        padding: "8px",
                                        font_family: conf.font.br_cobane,
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
                                    color_: Some(conf.color.raisin_black.to_owned()),
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
                                border_color: conf.color.timberwolf.to_string(),
                                border_radius: "2px",
                                h1 {
                                    font_family: conf.font.br_cobane,
                                    font_weight: "bold",
                                    color: conf.color.timberwolf.to_string(),
                                    "Blazingly fast"
                                }
                                p {
                                    font_family: conf.font.br_cobane,
                                    font_weight: "normal",
                                    color: conf.color.timberwolf.to_string(),
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
                                border_color: conf.color.timberwolf.to_string(),
                                border_radius: "2px",
                                h1 {
                                    font_family: conf.font.br_cobane,
                                    font_weight: "bold",
                                    color: conf.color.timberwolf.to_string(),
                                    "Blazingly fast"
                                }
                                p {
                                    font_family: conf.font.br_cobane,
                                    font_weight: "normal",
                                    color: conf.color.timberwolf.to_string(),
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