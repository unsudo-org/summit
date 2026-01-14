use super::*;

#[component]
pub fn SectionHighlight(color: Option<String>, children: Option<Element>) -> Element {
    let theme: theme::Theme = use_context();

    rsx!(
        section::Section {
            div {
                display: "flex",
                flex_direction: "column",
                justify_content: "start",
                align_items: "start",
                min_width: "100%",
                max_width: "100%",
                border_width: "2px",
                border_style: "solid",
                border_color: if let Some(color) = color {
                    color
                } else {
                    theme.color.foreground.to_string()
                },
                border_radius: "2px",
                padding: "32px",
                { children }
            }
        }
    )
}