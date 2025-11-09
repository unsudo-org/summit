use super::*;

#[component]
pub fn LogoCompact() -> Element {
    rsx!(
        h1 {
            display: "flex",
            flex_direction: "row",
            justify_content: "center",
            align_items: "center",
            font_family: font::ALIEN_SKYLINE,
            font_weight: "normal",
            color: color::TIMBERWOLF,
            span {
                style: format!(
                    r#"
                        background: linear-gradient(to bottom right, {}, {}, {});
                        -webkit-background-clip: text;
                        -webkit-text-fill-color: transparent;
                        background-clip: text;
                        color: transparent;
                        display: inline-block;
                    "#,
                    color::MEDIUM_SLATE_BLUE,
                    color::ROSE_POMPADOUR,
                    color::MINDARO
                ),
                "un"
            }
        }
    )
}

#[component]
pub fn Logo() -> Element {
    rsx!(
        h1 {
            display: "flex",
            flex_direction: "row",
            justify_content: "center",
            align_items: "center",
            font_family: font::ALIEN_SKYLINE,
            font_weight: "normal",
            color: color::TIMBERWOLF,
            span {
                style: format!(
                    r#"
                        background: linear-gradient(to bottom right, {}, {}, {});
                        -webkit-background-clip: text;
                        -webkit-text-fill-color: transparent;
                        background-clip: text;
                        color: transparent;
                        display: inline-block;
                    "#,
                    color::TIMBERWOLF,
                    color::TIMBERWOLF,
                    color::TIMBERWOLF
                ),
                "un"
            }
            span {
                "SUDO"
            }
        }
    )
}