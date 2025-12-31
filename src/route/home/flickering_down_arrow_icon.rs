use super::*;

#[component]
pub fn FlickeringDownArrowIcon(
    #[props(extends = GlobalAttributes)]
    attr: Vec<Attribute>
) -> Element {
    let theme: theme::Theme = use_context();

    rsx! {
        div {
            class: "soft_flicker",
            display: "flex",
            flex_direction: "column",
            justify_content: "center",
            align_items: "center",
            font_size: "32px",
            font_family: format!("{}", theme.font.body),
            font_weight: "normal",
            color: format!("{}", theme.color.foreground),
            ..attr,
            "↡"
        }
    }
}