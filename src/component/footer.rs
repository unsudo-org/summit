use super::*;

#[component]
pub fn Common() -> Element {
    let theme: theme::Theme = use_context();

    rsx!(
        Footer {
            socials_icon_color: theme.color.foreground.to_string(),
            socials_icon_w: "16px"
        }
    )
}

#[component]
pub fn Footer(
    socials_icon_color: String,
    socials_icon_w: String
) -> Element {
    rsx!(
        div {
            display: "flex",
            flex_direction: "column",
            min_width: "100%",
            gap: "8px",
            div {
                display: "flex",
                justify_content: "center",
                gap: "16px",
                Socials {
                    icon_color: socials_icon_color,
                    icon_w: socials_icon_w
                }
            }
            div {
                display: "flex",
                justify_content: "center",
                min_width: "100%",
                typography::P { "unsudo@atomicmail.io" }
            }
            div {
                display: "flex",
                flex_direction: "row",
                justify_content: "center",
                min_width: "100%",
                typography::P { "© 2026 Unsudo. All rights reserved." }
            }
        }
    )
}

#[component]
fn Socials(
    icon_color: String,
    icon_w: String
) -> Element {
    rsx!(
        div {
            display: "flex",
            flex_direction: "row",
            gap: "8px",
            To {
                to: "https://discord.gg/5yRwGEpUJY",
                icon::Icon {
                    url: asset!("/asset/icon/social/discord.svg"),
                    color: icon_color.to_owned(),
                    w: icon_w.to_owned()
                }
            }
            To {
                to: "https://github.com/settings/organizations",
                icon::Icon {
                    url: asset!("/asset/icon/social/github.svg"),
                    color: icon_color.to_owned(),
                    w: icon_w.to_owned()
                }
            }
            To {
                to: "/",
                icon::Icon {
                    url: asset!("/asset/icon/social/telegram.svg"),
                    color: icon_color.to_owned(),
                    w: icon_w.to_owned()
                }
            }
        }
    )
}