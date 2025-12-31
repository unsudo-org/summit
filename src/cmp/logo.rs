use super::*;

#[derive(Props, Clone, PartialEq)]
pub struct LogoProps {
    pub colors: (
        Hex,
        Hex,
        Hex,
        Hex
    ),
    pub size: &'static str
}

#[component]
pub fn Logo(props: LogoProps) -> Element {
    let style: String = format!(
        r#"
            background: linear-gradient(to bottom right, {}, {}, {});
            -webkit-background-clip: text;
            -webkit-text-fill-color: transparent;
            background-clip: text;
            color: transparent;
            display: inline-block;
        "#,
        props.colors.1,
        props.colors.2,
        props.colors.3
    );

    rsx!(
        h1 {
            display: "flex",
            flex_direction: "row",
            justify_content: "center",
            align_items: "center",
            font_size: props.size,
            font_family: "alien skyline",
            font_weight: "normal",
            color: format!("{}", props.colors.0),
            span { style, "un" }
            span {
                "SUDO"
            }
        }
    )
}