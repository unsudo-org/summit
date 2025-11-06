use super::*;

#[derive(Default)]
#[derive(Clone)]
#[derive(PartialEq)]
pub enum LogoMode {
    #[default]
    Icon,
    Horizontal
}

#[component]
pub fn Logo(
    mode: Option<LogoMode>,
    color: Option<String>
) -> Element {
    rsx!(
        match mode.unwrap_or_default() {
            LogoMode::Icon => {
                rsx!(
                    Icon {
                        url: asset!("/asset/press-kit/logo.svg"),
                        w: "32px"
                    }
                )
            },
            LogoMode::Horizontal => {
                rsx!(
                    div {
                        style: format!(
                            r#"
                                font-size: 2em;
                                font-family: {};
                                font-weight: normal;
                                color: {};
                            "#,
                            font::BR_COBANE,
                            color::SILVER
                        ),
                        "Unsudo"
                    }
                )
            }
        }
    )
}