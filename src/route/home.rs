use super::*;

#[component]
pub fn Home() -> Element {
    let theme: theme::Theme = use_context();

    rsx!(
        component::page::Page {
            component::page::Item {
                component::navbar::Common {}
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
                                component::To {
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
                        Banner {
                            heading: rsx!(
                                span {
                                    "A Layer 1.5 For "
                                }
                                span {
                                    "Polkadot"
                                }
                            ),
                            call_to_action_heading: rsx!("Challenge the status quo"),
                            call_to_action_button_group: rsx!(
                                div {
                                    display: "flex",
                                    flex_direction: "column",
                                    justify_content: "start",
                                    align_items: "start",
                                    gap: "8px",
                                    component::To {
                                        to: "/",
                                        component::FingerDisabled {
                                            component::Button { { "explore" } }
                                        }
                                    }
                                    div {
                                        font_size: "8px",
                                        font_family: theme.font.body,
                                        font_weight: "normal",
                                        "Coming soon"
                                    }
                                }
                                component::To {
                                    to: "https://github.com/unsudo-org/doc/blob/main/doc/whitepaper.md",
                                    component::Button { { "learn more" } }
                                }
                            ),
                            call_to_action_image: rsx!(
                                component::Shape {
                                    w: "200px",
                                    h: "auto",
                                    color: theme.color.background.to_owned(),
                                    model: component::ShapeModel::FourLobedRoundedSquare
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
            TextSection {
                contents: vec![
                    (rsx!("Reimagining Web3 on Polkadot"), vec![
                        rsx!(
                            span { "Polkadot JAM is a RISC-V–based virtual machine purpose-built for Polkadot, opening a new design space for verifiable, high-performance applications." }
                            span { " " }
                            span { "At Unsudo, our focus is simple: help developers navigate this emerging ecosystem and give them the tools to build powerful systems without unnecessary complexity." }
                        ),
                        rsx!(
                            span { "While Web3 promised openness and trustlessness, much of today’s internet still relies on brittle infrastructure and opaque control." }
                            span { " " }
                            span { "Polkadot JAM is a meaningful step forward, but its impact depends on usability and real-world adoption." }
                            span { " " }
                            span { "Unsudo closes this gap by turning JAM’s technical potential into practical, developer-friendly foundations—so builders can move from experimentation to production with confidence." }
                        )
                    ])
                ]
            }
            ProblemSection {
                label: rsx!("Censorship"),
                summary: rsx!("Centralized control blocks speech, access, and essential services."),
                bullet_points: [
                    rsx!("1.3 billion people live under state-imposed internet restrictions that block platforms, messaging, or financial services."),
                    rsx!("62 countries actively shut down social or financial networks during political unrest, leaving populations cut off from banking and communications."),
                    rsx!("45% of activists report harassment or account suspensions by intermediaries acting under state influence."),
                    rsx!("Governments cause 3 out of 5 internet outages in authoritarian regimes, cutting access to news, payment systems, or healthcare portals."),
                    rsx!("40% of content takedowns by major platforms are prompted by government requests, affecting free expression and access to resources."),
                    rsx!("Centralized banking and payment systems allow governments to freeze accounts for political reasons, affecting millions in critical moments.")
                ]
            }
            TextSection {
                contents: vec![
                    (rsx!(), vec![
                        rsx!(
                            span { "Centralized infrastructure is a tool of control." }
                            span { " " }
                            span { "Beyond censoring speech, governments and intermediaries can cut off access to banking, remittances, healthcare, and essential digital services." }
                            span { " " }
                            span { "Activists, journalists, and ordinary citizens are vulnerable to arbitrary restrictions, leaving entire communities unable to participate in society or conduct basic commerce." }
                            span { " " }
                            span { "Layer 1.5 self-sovereign protocols remove this dependence: the system operates deterministically, without intermediaries, ensuring that critical services remain accessible, censorship-resistant, and fair, even in authoritarian contexts." }
                        )
                    ])
                ]
            }
            ProblemSection {
                label: rsx!("Mutable Betrayal"),
                summary: rsx!("Mutable systems allow agreements to be rewritten, exploited, or broken after deployment."),
                bullet_points: [
                    rsx!("Over 70% of SaaS contracts allow unilateral updates that can alter user entitlements or pricing after signup."),
                    rsx!("0% of online financial services have changed terms of service in ways that materially affect users within 12 months."),
                    rsx!("Over 60% of online shutdowns in 2023 were politically motivated, blocking media, financial services, or communication."),
                    rsx!("1 in 4 global tech projects experienced governance or protocol modifications that violated user expectations."),
                    rsx!("42% of cross-border business contracts fail due to parties not honoring terms."),
                    rsx!("61% of organizations experienced insider tampering or unapproved changes in deployed software in the last two years.")
                ]
            }
            TextSection {
                contents: vec![
                    (rsx!(), vec![
                        rsx!(
                            span { "After software or contracts are deployed, mutability enables parties to change the rules in ways users cannot anticipate or contest." }
                            span { " " }
                            span { "This creates systemic risk and erodes trust: users may rely on a service to handle payments, access information, or maintain workflows, only to find the terms altered or functionality restricted." }
                            span { " " }
                            span { "Such changes are particularly dangerous across borders, where legal recourse is limited and inconsistent." }
                            span { " " }
                            span { "Whether through software updates, hidden administrative privileges, or modified protocols, post-deployment mutability allows insiders to break agreements, favor certain users, or impose unexpected costs, undermining fairness, stability, and predictability." }
                            span { " " }
                            span { "This is why self-sovereign, deterministic protocols that cannot be arbitrarily altered are crucial: they embed the agreement into the system itself, ensuring trust is guaranteed by code rather than fragile human promises." }
                        )
                    ])
                ]
            }
            ProblemSection {
                label: rsx!("Sovereignty Drain"),
                summary: rsx!("Reliance on others invites control and coercion."),
                bullet_points: [
                    rsx!("Over 90% of mid- and large organizations report that each hour of downtime costs $300,000, pressuring operators to comply with external demands."),
                    rsx!("63–70% of outages involve external providers."),
                    rsx!("Even with 99.9% uptime, servers can be offline 9 hours per year, creating windows for intervention."),
                    rsx!("40% of servers experience unplanned downtime annually."),
                    rsx!("60% of small businesses affected by critical failures shut down within six months."),
                    rsx!("High-reliability systems often pay premiums to maintain uptime, incentivizing compromise under external pressure.")
                ]
            }
            TextSection {
                contents: vec![
                    (rsx!(), vec![
                        rsx!(
                            span { "Systems that depend on external services or infrastructure are exposed to censorship and coercion." }
                            span { " " }
                            span { "Cloud providers, network operators, and SaaS platforms can comply with legal orders, government requests, or corporate pressure — often without transparency." }
                            span { " " }
                            span { "Users and developers have no recourse when external dependencies fail or act under coercion." }
                            span { " " }
                            span { "This “sovereignty drain” undermines the principle of self-sovereignty: true autonomy requires that the protocol owns its own rules, execution, and uptime, making it resistant to both censorship and corrupt influence." }
                        )
                    ])
                ]
            }
            Section {
                div {
                    display: "flex",
                    flex_direction: "column",
                    align_items: "center",
                    gap: "32px",
                    min_width: "100%",
                    div {
                        display: "flex",
                        justify_content: "center",
                        min_width: "100%",
                        div {
                            display: "flex",
                            flex_direction: "column",
                            gap: "8px",
                            h1 {
                                font_family: format!("{}", theme.font.display),
                                font_weight: "normal",
                                color: format!("{}", theme.color.foreground),
                                "Meet the Council"
                            }
                            component::To {
                                to: "https://github.com/unsudo-org/doc/blob/main/doc/whitepaper.md",
                                div {
                                    class: "float",
                                    all: "unset",
                                    font_family: format!("{}", theme.font.body),
                                    font_weight: "normal",
                                    color: format!("{}", theme.color.foreground),
                                    cursor: format!("url('{}'), auto", theme.cursor.finger),
                                    text_decoration: "underline",
                                    "learn more about the council"
                                }
                            }
                        }
                    }
                    div {
                        display: "flex",
                        flex_wrap: "wrap",
                        justify_content: "center",
                        min_width: "100%",
                        gap: "32px",
                        CouncilRoleCard {
                            name: rsx!("LogicGate"),
                            role: rsx!("Lead Engineer"),
                            image: rsx!(
                                component::Shape {
                                    w: "64px",
                                    h: "64px",
                                    color: theme.color.foreground.to_owned(),
                                    model: component::ShapeModel::ConcentricCircleLayers
                                }
                            )
                        }
                        CouncilRoleCard {
                            name: rsx!("Jza"),
                            role: rsx!("Product Manager"),
                            image: rsx!(
                                component::Shape {
                                    w: "64px",
                                    h: "64px",
                                    color: theme.color.foreground.to_owned(),
                                    model: component::ShapeModel::ConcentricCircleLayers
                                }
                            )
                        }
                    }
                }
            }
            Section {
                div {
                    display: "flex",
                    flex_direction: "column",
                    align_items: "center",
                    min_width: "100%",
                    gap: "32px",
                    div {
                        display: "flex",
                        flex_direction: "column",
                        align_items: "center",
                        gap: "32px",
                        min_width: "100%",
                        h1 {
                            font_family: format!("{}", theme.font.display),
                            font_weight: "normal",
                            color: format!("{}", theme.color.foreground),
                            "Meet the Contributors"
                        }
                    }
                    div {
                        display: "flex",
                        flex_wrap: "wrap",
                        justify_content: "center",
                        min_width: "100%",
                        gap: "64px",
                        CoreRoleCard {
                            name: rsx!("Chineko"),
                            role: rsx!("Art Director"),
                            image: rsx!(
                                component::Shape {
                                    w: "32px",
                                    h: "32px",
                                    color: theme.color.foreground.to_owned(),
                                    model: component::ShapeModel::CrossBadge
                                }
                            )
                        }
                        CoreRoleCard {
                            name: rsx!("Autismo"),
                            role: rsx!("Engineer"),
                            image: rsx!(
                                component::Shape {
                                    w: "32px",
                                    h: "32px",
                                    color: theme.color.foreground.to_owned(),
                                    model: component::ShapeModel::CrossBadge
                                }
                            )
                        }
                    }
                }
            }
            component::footer::Common {}
        }
    )
}

#[component]
fn TextSection(
    contents: Vec<(Element, Vec<Element>)>
) -> Element {
    let theme: theme::Theme = use_context();

    rsx!(
        Section {
            div {
                display: "flex",
                min_width: "100%",
                justify_content: "center",
                div {
                    display: "flex",
                    flex_direction: "column",
                    gap: "32px",
                    max_width: "800px",
                    for (heading, contents) in contents {
                        div {
                            display: "flex",
                            flex_direction: "column",
                            gap: "16px",
                            h3 {
                                font_family: format!("{}", theme.font.display),
                                font_weight: "normal",
                                color: format!("{}", theme.color.foreground),
                                { heading }
                            }
                            div {
                                display: "flex",
                                flex_direction: "column",
                                gap: "16px",
                                for content in contents {
                                    p {
                                        font_family: format!("{}", theme.font.regular),
                                        font_weight: "normal",
                                        color: format!("{}", theme.color.foreground),
                                        { content }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    )
}

#[component]
fn ProblemSection(
    label: Element,
    summary: Element,
    bullet_points: [Element; 6]
) -> Element {
    let theme: theme::Theme = use_context();
    
    rsx!(
        Section {
            div {
                padding_bottom: "16px",
                padding_top: "16px",
                h5 {
                    font_family: format!("{}", theme.font.body),
                    font_weight: "normal",
                    color: format!("{}", theme.color.foreground),
                    { summary }
                }
            }
            div {
                display: "flex",
                flex_direction: "row",
                align_items: "center",
                min_width: "100%",
                gap: "32px",
                div {
                    component::Shape {
                        w: "128px",
                        h: "128px",
                        model: component::ShapeModel::AngularStar,
                        color: theme.color.failure.to_owned()
                    }
                }
                div {
                    flex: "1",
                    background: format!("{}", theme.color.failure),
                    border_radius: "2px",
                    border_top_left_radius: "32px",
                    border_bottom_right_radius: "32px",
                    div {
                        display: "flex",
                        flex_direction: "row",
                        justify_content: "center",
                        align_items: "center",
                        h1 {
                            font_size: "4em",
                            font_family: theme.font.display,
                            font_weight: "normal",
                            color: theme.color.background.to_string(),
                            { label }
                        }
                    }
                }
            }
            div {
                display: "flex",
                flex_wrap: "wrap",
                justify_content: "center",
                min_width: "100%",
                padding: "32px",
                gap: "32px",
                div {
                    display: "flex",
                    flex_direction: "column",
                    gap: "16px",
                    BulletPoint {
                        ProblemCard {
                            label: rsx!(""),
                            value: rsx!({ bullet_points.get(0).unwrap() })
                        }
                    }
                    BulletPoint {
                        ProblemCard {
                            label: rsx!(""),
                            value: rsx!({ bullet_points.get(1).unwrap() })
                        }
                    }
                    BulletPoint {
                        ProblemCard {
                            label: rsx!(""),
                            value: rsx!({ bullet_points.get(2).unwrap() })
                        }
                    }
                    BulletPoint {
                        ProblemCard {
                            label: rsx!(""),
                            value: rsx!({ bullet_points.get(3).unwrap() })
                        }
                    }
                    BulletPoint {
                        ProblemCard {
                            label: rsx!(""),
                            value: rsx!({ bullet_points.get(4).unwrap() })
                        }
                    }
                    BulletPoint {
                        ProblemCard {
                            label: rsx!(""),
                            value: rsx!({ bullet_points.get(5).unwrap() })
                        }
                    }
                }
            }
        }
    )
}

#[component]
fn Section(children: Option<Element>) -> Element {
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
            padding_left: "64px",
            padding_right: "64px",
            { children }
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
fn Fieldset(
    label: Element,
    children: Option<Element>
) -> Element {
    let theme: theme::Theme = use_context();

    rsx!(
        component::Fieldset {
            max_width: "512px",
            label: rsx!(
                component::typography::Gradient {
                    bg: format!(
                        "linear-gradient(to bottom right, {}, {})",
                        theme.color.success,
                        theme.color.highlight
                    ),
                    { label }
                }
            ),
            p {
                font_family: format!("{}", theme.font.body),
                font_weight: "normal",
                color: format!("{}", theme.color.foreground),
                { children }
            }
        } 
    )
}

#[component]
fn BulletPoint(children: Option<Element>) -> Element {
    let theme: theme::Theme = use_context();

    rsx!(
        div {
            display: "flex",
            flex_direction: "row",
            align_items: "center",
            gap: "16px",
            div {
                display: "flex",
                flex_direction: "column",
                justify_content: "center",
                align_items: "center",
                border_width: "2px",
                border_style: "solid",
                border_color: format!("{}", theme.color.foreground),
                border_radius: "2px",
                padding: "8px",
                component::Shape {
                    w: "16px",
                    h: "16px",
                    model: component::ShapeModel::FivePointCircleGrid,
                    color: theme.color.foreground.to_owned()
                }
            }
            div {
                font_family: format!("{}", theme.font.body),
                font_weight: "normal",
                color: format!("{}", theme.color.foreground),
                { children }
            }
        }
    )
}

#[component]
fn CouncilRoleCard(
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
fn CoreRoleCard(
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
            div {
                display: "flex",
                border_width: "2px",
                border_style: "solid",
                border_color: format!("{}", theme.color.foreground),
                border_top_left_radius: "2px",
                border_bottom_left_radius: "2px",
                padding: "16px",
                min_height: "100%",
                div {
                    display: "flex",
                    justify_content: "center",
                    align_items: "center",
                    min_width: "100%",
                    min_height: "100%",
                    { image }
                }
            }
            div {
                display: "flex",
                flex_direction: "column",
                padding: "16px",
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
                padding: "4px",
                min_height: "100%",
                border_top_right_radius: "2px",
                border_bottom_right_radius: "2px",
                background: format!("{}", theme.color.highlight)
            }
        }
    )
}

#[component]
pub fn ProblemCard(
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
            h5 {
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