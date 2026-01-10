use super::*;

#[component]
pub fn Logo(
    color_0: Option<String>,
    color_1: Option<String>,
    color_2: Option<String>,
    font_size: Option<String>
) -> Element {
    let theme: theme::Theme = use_context();

    rsx!(
        h1 {
            display: "flex",
            flex_direction: "row",
            justify_content: "center",
            align_items: "center",
            font_size: font_size.unwrap_or("32px".to_string()),
            font_family: "alien skyline",
            font_weight: "normal",
            color: color_0.unwrap_or(theme.color.foreground.to_string()),
            span {
                cmp::typography::Gradient {
                    bg: format!(
                        "linear-gradient(to bottom right, {}, {})",
                        color_1.unwrap_or(theme.color.success.to_string()),
                        color_2.unwrap_or(theme.color.highlight.to_string())
                    ),
                    "un"
                }
            }
            span { "SUDO" }
        }
    )
}