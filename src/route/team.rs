use super::*;

#[component]
pub fn Team() -> Element {
    let theme: theme::Theme = use_context();

    rsx!(
        cmp::Page {
            div {
                display: "flex",
                flex_direction: "column",
                justify_content: "start",
                align_items: "start",
                cmp::navbar::Build {}
                div {
                    display: "flex",
                    flex_direction: "column",
                    justify_content: "start",
                    align_items: "start",
                    cmp::Fieldset {
                        label: rsx!("Team"),
                        cmp::typography::P { "" }
                        div {
                            display: "flex",
                            flex_direction: "column",
                        }
                    }
                    div {
                        CouncilMemberRoleCard {
                            name: rsx!("PascalCase"),
                            role: rsx!("Head Engineer"),
                            image: rsx!(
                                cmp::Shape {
                                    w: "64px",
                                    h: "64px",
                                    model: cmp::ShapeModel::ConcentricCircleLayers,
                                    color: theme.color.foreground.to_owned()
                                }
                            )
                        }
                        CouncilMemberRoleCard {
                            name: rsx!("PascalCase"),
                            role: rsx!("Head Engineer"),
                            image: rsx!(
                                cmp::Shape {
                                    w: "64px",
                                    h: "64px",
                                    model: cmp::ShapeModel::ConcentricCircleLayers,
                                    color: theme.color.foreground.to_owned()
                                }
                            )
                        }
                        CouncilMemberRoleCard {
                            name: rsx!("PascalCase"),
                            role: rsx!("Head Engineer"),
                            image: rsx!(
                                cmp::Shape {
                                    w: "64px",
                                    h: "64px",
                                    model: cmp::ShapeModel::ConcentricCircleLayers,
                                    color: theme.color.foreground.to_owned()
                                }
                            )
                        }
                    }
                }
            }
        }
    )
}

#[component]
fn CouncilMemberRoleCard(
    image: Element,
    name: Element,
    role: Element,

    #[props(extends = GlobalAttributes)]
    more: Vec<Attribute>
) -> Element {
    let theme: theme::Theme = use_context();

    rsx!(
        div {
            display: "flex",
            flex_direction: "column",
            justify_content: "start",
            align_items: "start",
            min_width: "400px",
            max_width: "400px",
            ..more,
            div {
                display: "flex",
                flex_direction: "column",
                justify_content: "start",
                align_items: "start",
                border_width: "2px",
                border_style: "solid",
                border_color: format!("{}", theme.color.foreground),
                border_top_left_radius: "2px",
                border_top_right_radius: "2px",
                padding: "16px",
                min_width: "100%",
                div {
                    display: "flex",
                    flex_direction: "row",
                    justify_content: "center",
                    align_items: "center",
                    min_width: "100%",
                    max_width: "100%",
                    min_height: "100%",
                    max_height: "100%",
                    { image }
                }
            }
            div {
                display: "flex",
                flex_direction: "column",
                justify_content: "start",
                align_items: "start",
                padding: "16px",
                gap: "16px",
                min_width: "100%",
                border_width: "2px",
                border_style: "solid",
                border_color: format!("{}", theme.color.foreground),
                background: format!("{}", theme.color.foreground),
                h3 {
                    font_family: format!("{}", theme.font.display),
                    font_weight: "normal",
                    color: format!("{}", theme.color.background),
                    { name }
                }
                div {
                    display: "flex",
                    flex_direction: "row",
                    flex_wrap: "wrap",
                    justify_content: "start",
                    align_items: "start",
                    gap: "8px",
                    h3 {
                        display: "flex",
                        flex_direction: "row",
                        justify_content: "start",
                        align_items: "start",
                        font_family: format!("{}", theme.font.body),
                        font_weight: "normal",
                        color: format!("{}", theme.color.background),
                        { role }
                    }
                }
            }
            div {
                display: "flex",
                flex_direction: "column",
                justify_content: "start",
                align_items: "start",
                padding: "4px",
                min_width: "100%",
                border_bottom_left_radius: "2px",
                border_bottom_right_radius: "2px",
                background: format!("{}", theme.color.highlight)
            }
        }
    )
}

#[component]
fn CoreContributorRoleCard(
    image: Element,
    name: Element,
    role: Element,

    #[props(extends = GlobalAttributes)]
    more: Vec<Attribute>
) -> Element {
    let theme: theme::Theme = use_context();

    rsx!(
        div {
            display: "flex",
            flex_direction: "row",
            justify_content: "start",
            align_items: "start",
            ..more,
            div {
                display: "flex",
                flex_direction: "column",
                justify_content: "start",
                align_items: "start",
                border_width: "2px",
                border_style: "solid",
                border_color: format!("{}", theme.color.foreground),
                border_top_left_radius: "2px",
                border_bottom_left_radius: "2px",
                padding: "16px",
                min_height: "100%",
                div {
                    display: "flex",
                    flex_direction: "row",
                    justify_content: "center",
                    align_items: "center",
                    min_width: "100%",
                    max_width: "100%",
                    min_height: "100%",
                    max_height: "100%",
                    { image }
                }
            }
            div {
                display: "flex",
                flex_direction: "column",
                justify_content: "start",
                align_items: "start",
                padding: "16px",
                min_width: "100%",
                border_width: "2px",
                border_style: "solid",
                border_color: format!("{}", theme.color.foreground),
                background: format!("{}", theme.color.foreground),
                h2 {
                    font_family: format!("{}", theme.font.display),
                    font_weight: "normal",
                    color: format!("{}", theme.color.background),
                    { name }
                }
                h4 {
                    font_family: format!("{}", theme.font.body),
                    font_weight: "normal",
                    color: format!("{}", theme.color.background),
                    { role }
                }
            }
            div {
                display: "flex",
                flex_direction: "column",
                justify_content: "start",
                align_items: "start",
                padding: "4px",
                min_height: "100%",
                border_top_right_radius: "2px",
                border_bottom_right_radius: "2px",
                background: format!("{}", theme.color.highlight)
            }
        }
    )
}