use super::*;

#[component]
pub fn Banner(
    heading: Element,
    call_to_action_heading: Element,
    call_to_action_button_group: Element,
    call_to_action_image: Element,
    highlights: Vec<Element>
) -> Element {
    let conf: theme::Conf = use_context();

    rsx! {
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
                align_items: "end",
                padding: "16px",
                min_width: "100%",
                background: conf.color.timberwolf.to_string(),
                border_radius: "2px",
                h1 {
                    display: "flex",
                    flex_direction: "column",
                    justify_content: "start",
                    align_items: "start",
                    font_family: conf.font.borneox,
                    font_weight: "normal",
                    color: conf.color.raisin_black.to_string(),
                    max_width: "256px",
                    text_align: "left",
                    word_wrap: "break-word",
                    { heading }
                }
                CallToActionGroup {
                    heading: call_to_action_heading,
                    button_group: call_to_action_button_group,
                    image: call_to_action_image
                }
            }
            div {
                display: "flex",
                flex_direction: "row",
                justify_content: "start",
                align_items: "start",
                gap: "8px",
                for highlight in highlights.into_iter() {
                    Highlight { { highlight } }
                }   
            }
        }
    }
}

#[component]
fn CallToActionGroup(heading: Element, button_group: Element, image: Element) -> Element {
    let conf: theme::Conf = use_context();

    rsx! {
        div {
            display: "flex",
            flex_direction: "row",
            justify_content: "start",
            align_items: "end",
            div {
                class: "float",
                display: "flex",
                flex_direction: "column",
                justify_content: "start",
                align_items: "start",
                max_width: "256px",
                gap: "16px",
                h3 {
                    font_family: conf.font.brulia_test,
                    font_weight: "normal",
                    color: conf.color.raisin_black.to_string(),
                    { heading }
                }
                div {
                    display: "flex",
                    flex_direction: "row",
                    justify_content: "start",
                    align_items: "start",
                    gap: "8px",
                    { button_group }
                }
            }
            div {
                display: "flex",
                flex_direction: "row",
                justify_content: "start",
                align_items: "start",
                { image }
            }
        }
    }
}

#[component]
fn Highlight(children: Element) -> Element {
    let conf: theme::Conf = use_context();

    rsx! {
        div {
            display: "flex",
            flex_direction: "row",
            justify_content: "start",
            align_items: "start",
            border_width: "2px",
            border_style: "solid",
            border_color: conf.color.timberwolf.to_string(),
            border_radius: "2px",
            padding: "16px",
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
    }
}