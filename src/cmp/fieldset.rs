use super::*;

#[component]
pub fn Fieldset(
    label: Option<Element>,
    children: Option<Element>,
    #[props(extends = GlobalAttributes)]
    more: Vec<Attribute>
) -> Element {
    let theme: theme::Theme = use_context();

    rsx! {
        fieldset {
            border_width: "2px",
            border_style: "solid",
            border_color: format!("{}", theme.color.foreground),
            border_radius: "2px",
            ..more,
            legend {
                display: "flex",
                flex_direction: "row",
                justify_content: "start",
                align_items: "start",
                padding_left: "8px",
                padding_right: "8px",
                font_family: format!("{}", theme.font.display),
                font_weight: "normal",
                font_size: "8px",
                color: format!("{}", theme.color.foreground),
                margin_left: "8px",
                { label }
            },
            div {
                display: "flex",
                flex_direction: "column",
                justify_content: "start",
                align_items: "start",
                padding_top: "8px",
                padding_bottom: "16px",
                padding_left: "16px",
                padding_right: "16px",
                min_width: "128px",
                { children }
            }
        }
    }
}