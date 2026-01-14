use super::*;

#[component]
pub fn Logo(
    color_0: Option<&'static str>,
    color_1: Option<&'static str>,
    color_2: Option<&'static str>,
    font_size: Option<&'static str>
) -> Element {
    let theme: theme::Theme = use_context();

    rsx!(
        h1 {
            display: "flex",
            flex_direction: "row",
            justify_content: "center",
            align_items: "center",
            font_size: if let Some(font_size) = font_size {
                font_size.to_owned()
            } else {
                "32px".to_owned()
            },
            font_family: "alien skyline",
            font_weight: "normal",
            color: if let Some(color) = color_0 {
                color.to_owned()
            } else {
                theme.color.foreground.to_string()
            },
            span {
                component::typography::Gradient {
                    bg: format!(
                        "linear-gradient(to bottom right, {}, {})",
                        if let Some(color) = color_1 {
                            color.to_owned()
                        } else {
                            theme.color.success.to_string()
                        },
                        if let Some(color) = color_2 {
                            color.to_owned()
                        } else {
                            theme.color.highlight.to_string()
                        }
                    ),
                    "un"
                }
            }
            span { "SUDO" }
        }
    )
}