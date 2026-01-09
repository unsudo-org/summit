use super::*;

#[component]
pub fn Home() -> Element {
    let theme: theme::Theme = use_context();

    rsx!(
        cmp::Page {
            HeroSection {}
            SummarySection {}
            ProblemSection {}
            TeamSection {}
        }
    )
}

#[component]
fn HeroSection() -> Element {
    let theme: theme::Theme = use_context();

    rsx!(
        cmp::PageItem {
            cmp::navbar::Build {}
            div {
                flex: "1"
            }
            div {
                display: "flex",
                flex_direction: "row",
                justify_content: "center",
                align_items: "start",
                min_width: "100%",
                max_width: "100%",
                padding_top: "32px",
                padding_bottom: "32px",
                div {
                    display: "flex",
                    flex_direction: "column",
                    justify_content: "start",
                    align_items: "start",
                    div {
                        display: "flex",
                        flex_direction: "row",
                        justify_content: "start",
                        align_items: "start",
                        gap: "8px",
                        padding_bottom: "32px",
                        for (asset, to) in vec![
                            (asset!("asset/icon/social/discord.svg"), "/"),
                            (asset!("asset/icon/social/github.svg"), "/"),
                            (asset!("asset/icon/social/telegram.svg"), "/")
                        ] {
                            Link {
                                to,
                                div {
                                    display: "flex",
                                    flex_direction: "start",
                                    align_items: "start",
                                    min_width: "16px",
                                    max_width: "16px",
                                    aspect_ratio: "1 / 1",
                                    background_image: format!("url({})", asset),
                                    background_position: "center",
                                    background_size: "contain",
                                    background_repeat: "no-repeat",
                                    color: theme.color.foreground.to_string()
                                }
                            }
                        }
                    }
                    cmp::glyph::Glyph {
                        top_left: rsx!(
                            cmp::glyph::TopLeftAngle {
                                border_width: "2px",
                                border_style: "solid",
                                border_color: format!("{}", theme.color.foreground),
                                border_length: "8px",
                                border_radius: "2px"
                            }
                        ),
                        top_right: rsx!(
                            cmp::glyph::TopRightAngle {
                                border_width: "2px",
                                border_style: "solid",
                                border_color: format!("{}", theme.color.foreground),
                                border_length: "8px",
                                border_radius: "2px"
                            }
                        ),
                        bottom_left: rsx!(
                            cmp::glyph::BottomLeftAngle {
                                border_width: "2px",
                                border_style: "solid",
                                border_color: format!("{}", theme.color.foreground),
                                border_length: "8px",
                                border_radius: "2px"
                            }
                        ),
                        bottom_right: rsx!(
                            cmp::glyph::BottomRightAngle {
                                border_width: "2px",
                                border_style: "solid",
                                border_color: format!("{}", theme.color.foreground),
                                border_length: "8px",
                                border_radius: "2px"
                            }
                        ),
                        cmp::glyph::Content {
                            padding: "16px",
                            Banner {
                                heading: rsx!(
                                    span {
                                        "A Layer 1.5 For "
                                    }
                                    span {
                                        background: theme.color.background.to_string(),
                                        border_radius: "2px",
                                        cmp::typography::Gradient {
                                            bg: format!(
                                                "linear-gradient(to bottom right, {}, {})",
                                                theme.color.failure,
                                                theme.color.failure
                                            ),
                                            "Polkadot"
                                        }
                                    }
                                ),
                                call_to_action_heading: rsx!("Challenge the status quo"),
                                call_to_action_button_group: rsx!(
                                    Link {
                                        all: "unset",
                                        to: "/",
                                        div {
                                            display: "flex",
                                            flex_direction: "column",
                                            justify_content: "start",
                                            align_items: "start",
                                            gap: "8px",
                                            cmp::Button {
                                                is_locked: true,
                                                { "explore" }
                                            }
                                            div {
                                                font_size: "8px",
                                                font_family: theme.font.body,
                                                font_weight: "normal",
                                                "Coming soon"
                                            }
                                        }
                                    }
                                    Link {
                                        all: "unset",
                                        to: "/learn_more",
                                        cmp::Button { { "learn more" } }
                                    }
                                ),
                                call_to_action_image: rsx!(
                                    cmp::Shape {
                                        w: "200px",
                                        h: "auto",
                                        color: theme.color.background.to_owned(),
                                        model: cmp::ShapeModel::FourLobedRoundedSquare
                                    }
                                ),
                                highlights: vec![
                                    rsx!("Transparency"),
                                    rsx!("Resilience"),
                                    rsx!("100% Uptime"),
                                    rsx!("Gassless")
                                ]
                            }
                        }
                    }
                }
            }
            div {
                flex: "1"
            }
            div {
                display: "flex",
                flex_direction: "row",
                justify_content: "center",
                align_items: "start",
                min_width: "100%",
                max_width: "100%",
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
                    "↡"
                }
            }
            div {
                flex: "1"
            }
        }
    )
}

#[component]
fn SummarySection() -> Element {
    let theme: theme::Theme = use_context();

    rsx!(
        div {
            display: "flex",
            flex_direction: "row",
            justify_content: "center",
            align_items: "start",
            min_width: "100%",
            max_width: "100%",
            gap: "32px",
            cmp::Fieldset {
            
                label: rsx!("What?"),
                div {
                    display: "flex",
                    flex_direction: "column",
                    justify_content: "start",
                    align_items: "start",
                    min_width: "300px",
                    max_width: "300px",
                    cmp::typography::P {
                        "A Layer 1.5 built on Polkadot that abstracts complexity without sacrificing sovereignty. We provide a unified execution and coordination layer that lets builders ship faster, users interact seamlessly, and systems scale horizontally—while remaining natively interoperable with the Polkadot ecosystem."
                    }
                }
            }
            cmp::Fieldset {
                label: rsx!("Why?"),
                div {
                    display: "flex",
                    flex_direction: "column",
                    justify_content: "start",
                    align_items: "start",
                    min_width: "300px",
                    max_width: "300px",
                    cmp::typography::P {
                        "You cannot challenge the status quo while depending on it. Real change requires infrastructure that is transparent by default, resilient by design, and impossible to silently control. We build for a future where systems serve users—not the other way around."
                    }
                }
            }
            cmp::Fieldset {
                label: rsx!("Where?"),
                div {
                    display: "flex",
                    flex_direction: "column",
                    justify_content: "start",
                    align_items: "start",
                    min_width: "300px",
                    max_width: "300px",
                    cmp::typography::P {
                        "On-chain, between chains, and beyond them. We operate where systems meet—turning fragmentation into coherence."
                    }
                }
            }
        }
        div {
            display: "flex",
            flex_direction: "row",
            justify_content: "center",
            align_items: "start",
            min_width: "100%",
            max_width: "100%",
            h4 {
                display: "flex",
                font_family: format!("{}", theme.font.display),
                font_weight: "normal",
                color: format!("{}", theme.color.foreground),
                "jjj"
            }
        }
        // comment section on what it is
    )
}

#[component]
fn ProblemSection() -> Element {
    let theme: theme::Theme = use_context();

    rsx!(
        div {
            display: "flex",
            flex_direction: "column",
            justify_content: "start",
            align_items: "start",
            min_width: "100%",
            max_width: "100%",
            padding_top: "64px",
            padding_bottom: "64px",
            div {
                display: "flex",
                flex_direction: "row",
                justify_content: "start",
                align_items: "start",
                min_width: "100%",
                max_width: "100%",
                padding: "32px",
                cmp::glyph::Glyph {
                    top_left: rsx!(
                        cmp::glyph::TopLeftAngle {
                            border_width: "2px",
                            border_style: "solid",
                            border_color: format!("{}", theme.color.foreground),
                            border_radius: "2px",
                            border_length: "8px"
                        }
                    ),
                    top_right: rsx!(
                        cmp::glyph::TopRightAngle {
                            border_width: "2px",
                            border_style: "solid",
                            border_color: format!("{}", theme.color.foreground),
                            border_radius: "2px",
                            border_length: "8px"
                        }
                    ),
                    bottom_left: rsx!(
                        cmp::glyph::BottomLeftAngle {
                            border_width: "2px",
                            border_style: "solid",
                            border_color: format!("{}", theme.color.foreground),
                            border_radius: "2px",
                            border_length: "8px"
                        }
                    ),
                    bottom_right: rsx!(
                        cmp::glyph::BottomRightAngle {
                            border_width: "2px",
                            border_style: "solid",
                            border_color: format!("{}", theme.color.foreground),
                            border_radius: "2px",
                            border_length: "8px"
                        }
                    ),
                    cmp::glyph::Content {
                        padding: "16px",
                        cmp::typography::P {
                            max_width: "256px",
                            text_align: "justify",
                            "We were promised"
                        }
                    }
                }
            }
            HazardStripeContainer {
                div {
                    display: "flex",
                    flex_direction: "row",
                    justify_content: "center",
                    align_items: "start",
                    min_width: "100%",
                    max_width: "100%",
                    div {
                        display: "flex",
                        flex_direction: "row",
                        flex_wrap: "wrap",
                        flex: "1",
                        justify_content: "center",
                        align_items: "center",
                        gap: "8px",
                        HazardStripeContainerContent {
                            cmp::effect::RenderOnVisible {
                                ProblemCard {
                                    label: rsx!("Unbanked"),
                                    value: rsx!(
                                        cmp::Counter {
                                            from: 0.0,
                                            to: 4_000_000.0,
                                            duration: time::Duration::from_millis(9000)
                                        }
                                    )
                                }
                            }
                        }
                        HazardStripeContainerContent {
                            cmp::effect::RenderOnVisible {
                                ProblemCard {
                                    label: rsx!("Unbanked"),
                                    value: rsx!(
                                        cmp::Counter {
                                            from: 0.0,
                                            to: 4_000_000.0,
                                            duration: time::Duration::from_millis(9000)
                                        }
                                    )
                                }
                            }
                        }
                        HazardStripeContainerContent {
                            cmp::effect::RenderOnVisible {
                                ProblemCard {
                                    label: rsx!("Unbanked"),
                                    value: rsx!(
                                        cmp::Counter {
                                            from: 0.0,
                                            to: 4_000_000.0,
                                            duration: time::Duration::from_millis(9000)
                                        }
                                    )
                                }
                            }
                        }
                    }
                    div {
                        min_width: "32px"
                    }
                    HazardStripeContainerContent {
                        cmp::Shape {
                            w: "256px",
                            h: "256px",
                            model: cmp::ShapeModel::GeometricStarburst,
                            color: theme.color.foreground.to_owned()
                        }
                    }
                    div {
                        min_width: "32px"
                    }
                    div {
                        display: "flex",
                        flex_direction: "row",
                        flex_wrap: "wrap",
                        flex: "1",
                        justify_content: "center",
                        align_items: "center",
                        gap: "8px",
                        HazardStripeContainerContent {
                            cmp::effect::RenderOnVisible {
                                ProblemCard {
                                    label: rsx!("Unbanked"),
                                    value: rsx!(
                                        cmp::Counter {
                                            from: 0.0,
                                            to: 4_000_000.0,
                                            duration: time::Duration::from_millis(9000)
                                        }
                                    )
                                }
                            }
                        }
                        HazardStripeContainerContent {
                            cmp::effect::RenderOnVisible {
                                ProblemCard {
                                    label: rsx!("Unbanked"),
                                    value: rsx!(
                                        cmp::Counter {
                                            from: 0.0,
                                            to: 4_000_000.0,
                                            duration: time::Duration::from_millis(9000)
                                        }
                                    )
                                }
                            }
                        }
                        HazardStripeContainerContent {
                            cmp::effect::RenderOnVisible {
                                ProblemCard {
                                    label: rsx!("Unbanked"),
                                    value: rsx!(
                                        cmp::Counter {
                                            from: 0.0,
                                            to: 4_000_000.0,
                                            duration: time::Duration::from_millis(9000)
                                        }
                                    )
                                }
                            }
                        }
                    }
                }
            }
            div {
                display: "flex",
                flex_direction: "row",
                justify_content: "end",
                align_items: "start",
                min_width: "100%",
                max_width: "100%",
                padding: "32px",
                cmp::glyph::Glyph {
                    top_left: rsx!(
                        cmp::glyph::TopLeftAngle {
                            border_width: "2px",
                            border_style: "solid",
                            border_color: format!("{}", theme.color.foreground),
                            border_radius: "2px",
                            border_length: "8px"
                        }
                    ),
                    top_right: rsx!(
                        cmp::glyph::TopRightAngle {
                            border_width: "2px",
                            border_style: "solid",
                            border_color: format!("{}", theme.color.foreground),
                            border_radius: "2px",
                            border_length: "8px"
                        }
                    ),
                    bottom_left: rsx!(
                        cmp::glyph::BottomLeftAngle {
                            border_width: "2px",
                            border_style: "solid",
                            border_color: format!("{}", theme.color.foreground),
                            border_radius: "2px",
                            border_length: "8px"
                        }
                    ),
                    bottom_right: rsx!(
                        cmp::glyph::BottomRightAngle {
                            border_width: "2px",
                            border_style: "solid",
                            border_color: format!("{}", theme.color.foreground),
                            border_radius: "2px",
                            border_length: "8px"
                        }
                    ),
                    cmp::glyph::Content {
                        padding: "16px",
                        cmp::typography::P {
                            max_width: "256px",
                            text_align: "justify",
                            "We were promised"
                        }
                    }
                }
            }
        }
    )
}

#[component]
fn TeamSection() -> Element {
    let theme: theme::Theme = use_context();
    
    rsx!(
        h1 {
            "Meet the team!"
        }
        div {
            display: "flex",
            flex_direction: "row",
            justify_content: "center",
            align_items: "start",
            min_width: "100%",
            max_width: "100%",
            cmp::Fieldset {
                label: rsx!("Core Contributors"),
                cmp::Fieldset {
                    label: rsx!("Council"),
                    div {
                        display: "flex",
                        flex_direction: "row",
                        flex_wrap: "wrap",
                        justify_content: "center",
                        align_items: "start",
                        min_width: "100%",
                        max_width: "100%",
                        gap: "32px",
                        CouncilMemberRoleCard {
                            name: rsx!("PascalCase"),
                            role: rsx!("Lead Engineer"),
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
                            name: rsx!("Jza"),
                            role: rsx!("Product Manager"),
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
                            name: rsx!("Frosty"),
                            role: rsx!("Project Manager"),
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
                div {
                    display: "flex",
                    padding: "32px",
                    CoreContributorRoleCard {
                        name: rsx!("Autismo"),
                        role: rsx!("Engineer"),
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
    )
}

#[component]
fn Banner(
    heading: Element,
    call_to_action_heading: Element,
    call_to_action_button_group: Element,
    call_to_action_image: Element,
    highlights: Vec<Element>,

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
            gap: "8px",
            ..more,
            div {
                display: "flex",
                flex_direction: "row",
                justify_content: "space-between",
                align_items: "end",
                padding: "16px",
                min_width: "100%",
                background: format!("{}", theme.color.foreground),
                border_radius: "2px",
                h1 {
                    display: "flex",
                    flex_direction: "column",
                    justify_content: "start",
                    align_items: "start",
                    font_family: format!("{}", theme.font.display),
                    font_weight: "normal",
                    color: format!("{}", theme.color.background),
                    max_width: "256px",
                    text_align: "left",
                    word_wrap: "break-word",
                    { heading }
                }
                // call-to-action-group
                div {
                    display: "flex",
                    flex_direction: "row",
                    justify_content: "start",
                    align_items: "end",
                    div {
                        class: "float",
                        display: "flex",
                        flex_direction: "column",
                        justify_content: "start",
                        align_items: "start",
                        max_width: "256px",
                        gap: "16px",
                        h3 {
                            font_family: format!("{}", theme.font.body),
                            font_weight: "normal",
                            color: format!("{}", theme.color.background),
                            { call_to_action_heading }
                        }
                        div {
                            display: "flex",
                            flex_direction: "row",
                            justify_content: "start",
                            align_items: "start",
                            gap: "8px",
                            { call_to_action_button_group }
                        }
                    }
                    div {
                        display: "flex",
                        flex_direction: "row",
                        justify_content: "start",
                        align_items: "start",
                        { call_to_action_image }
                    }
                }
            }
            // highlight-cards
            div {
                display: "flex",
                flex_direction: "row",
                justify_content: "start",
                align_items: "start",
                gap: "8px",
                for highlight in highlights.into_iter() {
                    div {
                        display: "flex",
                        flex_direction: "row",
                        justify_content: "start",
                        align_items: "start",
                        border_width: "2px",
                        border_style: "solid",
                        border_color: format!("{}", theme.color.foreground),
                        border_radius: "2px",
                        padding: "16px",
                        h3 {
                            display: "flex",
                            flex_direction: "row",
                            justify_content: "center",
                            align_items: "center",
                            font_family: format!("{}", theme.font.display),
                            font_weight: "bold",
                            color: format!("{}", theme.color.foreground),
                            { highlight }   
                        }
                    }
                }   
            }
        }
    )
}

#[component]
fn HazardStripeContainerContent(children: Option<Element>) -> Element {
    let theme: theme::Theme = use_context();

    rsx!(
        div {
            background: format!("{}", theme.color.background),
            border_radius: "2px",
            padding: "8px",
            { children }
        }
    )
}

#[component]
fn HazardStripeContainer(children: Option<Element>) -> Element {
    let theme: theme::Theme = use_context();

    rsx!(
        cmp::HazardStripe {
            min_width: "100%",
            max_width: "100%",
            foreground_color: theme.color.failure.to_owned(),
            background_color: theme.color.background.to_owned(),
            duration: time::Duration::from_millis(90000),
            padding: "8px",
            { children }
        }
    )
}

#[component]
fn ProblemCard(
    label: Element,
    value: Element,

    #[props(extends = GlobalAttributes)]
    more: Vec<Attribute>
) -> Element {
    let theme: theme::Theme = use_context();

    let mut is_visible: Signal<bool> = use_signal(|| {
        false
    });

    rsx!(
        div {
            onvisible: move |_| *is_visible.write() = true,
            display: "flex",
            flex_direction: "column",
            justify_content: "start",
            align_items: "start",
            ..more,
            h2 {
                display: "flex",
                flex_direction: "row",
                font_family: format!("{}", theme.font.body),
                font_weight: "normal",
                color: format!("{}", theme.color.foreground),
                gap: "4px",
                if *is_visible.read() {
                    { value }
                }
            }
            h2 {
                font_family: format!("{}", theme.font.display),
                font_weight: "normal",
                color: format!("{}", theme.color.foreground),
                { label }
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