use super::*;

#[component]
pub fn LogoCompact() -> Element {
    let conf: conf::Conf = use_context();
    rsx!(
        h1 {
            display: "flex",
            flex_direction: "row",
            justify_content: "center",
            align_items: "center",
            font_family: conf.font.alien_skyline,
            font_weight: "normal",
            color: conf.color.timberwolf.to_string(),
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
                    conf.color.medium_slate_blue,
                    conf.color.rose_pompadour,
                    conf.color.mindaro
                ),
                "un"
            }
        }
    )
}

#[component]
pub fn Logo() -> Element {
    let conf: conf::Conf = use_context();
    rsx!(
        h1 {
            display: "flex",
            flex_direction: "row",
            justify_content: "center",
            align_items: "center",
            font_family: conf.font.alien_skyline,
            font_weight: "normal",
            color: conf.color.timberwolf,
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
                    conf.color.timberwolf,
                    conf.color.timberwolf,
                    conf.color.timberwolf
                ),
                "un"
            }
            span {
                "SUDO"
            }
        }
    )
}