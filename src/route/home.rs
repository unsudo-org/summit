use super::*;

modwire::expose! {
    pub banner
    pub flickering_down_arrow_icon
}

#[component]
pub fn Home() -> Element {
    let conf: theme::Conf = use_context();

    rsx! {
        cmp::Page {
            cmp::PageItem {
                cmp::NavbarBuild {}
                div {
                    min_height: "32px"
                }
                div {
                    display: "flex",
                    flex_direction: "row",
                    justify_content: "center",
                    align_items: "center",
                    min_width: "100%",
                    cmp::Cross {
                        padding: cmp::CrossFieldsetPadding::Normal,
                        Banner {
                            heading: rsx! {
                                "A Layer 1.5 For Polkadot"
                            },
                            call_to_action_heading: rsx! {
                                "It's time for a better web3 experience"
                            },
                            call_to_action_button_group: rsx! {
                                Link {
                                    all: "unset",
                                    to: "/explore",
                                    cmp::Button { { "explore" } }
                                }
                                Link {
                                    all: "unset",
                                    to: "/learn_more",
                                    cmp::Button { { "learn more" } }
                                }
                            },
                            call_to_action_image: rsx! {
                                cmp::Shape {
                                    w: "200px",
                                    h: "auto",
                                    fill: conf.color.raisin_black.to_owned(),
                                    model: cmp::ShapeModel::FourLobedRoundedSquare
                                }
                            },
                            highlights: vec![
                                rsx! { "Transparency" },
                                rsx! { "Resilience" },
                                rsx! { "100% Uptime" },
                                rsx! { "Gassless" }
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
                    align_items: "center",
                    min_width: "100%",
                    max_width: "100%",
                    FlickeringDownArrowIcon {}
                }
                div {
                    min_height: "8px"
                }
            }

            cmp::Fieldset {
                label: rsx! { "Council Members" },
                div {
                    display: "flex",
                    flex_direction: "row",
                    flex_wrap: "wrap",
                    justify_content: "center",
                    align_items: "center",
                    gap: "8px",
                    CouncilMemberRoleCard {
                        name: rsx!(
                            "PascalCase"
                        ),
                        role: rsx!(
                            "Lead Engineer"
                        ),
                        image: rsx!(
                            cmp::Shape {
                                w: "128px",
                                h: "128px",
                                model: cmp::ShapeModel::FivePointCircleGrid,
                                fill: conf.color.timberwolf.to_owned()
                            }
                        )
                    }
                    CouncilMemberRoleCard {
                        name: rsx! { "Jza" },
                        role: rsx! { "Product Manager" },
                        image: rsx! {
                            cmp::Shape {
                                w: "128px",
                                h: "128px",
                                model: cmp::ShapeModel::FourLobedFlower,
                                fill: conf.color.timberwolf.to_owned()
                            }
                        }
                    }
                    CouncilMemberRoleCard {
                        name: rsx! { "Frosty" },
                        role: rsx! { "Project Lead" },
                        image: rsx! {
                            cmp::Shape {
                                w: "128px",
                                h: "128px",
                                model: cmp::ShapeModel::FourLobedRoundedSquare,
                                fill: conf.color.timberwolf.to_owned()
                            }
                        }
                    }
                }   
            }
            cmp::Fieldset {
                label: rsx! { "Core Contributors" },
                div {
                    CoreContributorCard {
                        model: cmp::ShapeModel::DoubleBean,
                        name: "Autismo",
                        description: "Engineer"
                    }
                }
            }
            cmp::PageItem {
                HonourableMentionCard {
                    name: rsx! { "Unsudo" },
                    icon: rsx! {
                        cmp::Shape {
                            w: "16px",
                            h: "16px",
                            model: cmp::ShapeModel::TwoPartEmblem,
                            fill: conf.color.timberwolf.to_owned()
                        }
                    }
                }
                div {
                    padding: "64px",
                    cmp::Fieldset {
                        label: rsx! { "Core" },
                        "aaaaaaaaaaaaaa bbbbb bbbbb bbbbb cccccccc ccccccccccccc ddddddddddddd ddddd ddddd"
                    }
                }
            }
        }
    }
}



#[component]
fn MoreIcon() -> Element {
    let conf = use_context::<theme::Conf>();

    rsx! {
        div {
            class: "soft_flicker",
            display: "flex",
            flex_direction: "column",
            justify_content: "center",
            align_items: "center",
            font_size: "32px",
            font_family: conf.font.br_cobane,
            font_weight: "normal",
            color: conf.color.timberwolf.to_string(),
            "↡"
        }
    }
}








// MARK: R

#[derive(Default)]
enum RouteState {
    #[default]
    Spawn,
    Idle
}

#[component]
fn Routing() -> Element {
    let conf: theme::Conf = use_context();
    let mut state: Signal<_> = use_signal(RouteState::default);

    use_effect(move || {
        *state.write() = RouteState::Idle;
    });

    rsx!(
        div {
            display: "flex",
            flex_direction: "row",
            justify_content: "center",
            align_items: "center",
            min_width: "1050px",
            padding: "16px",
            background: conf.color.medium_slate_blue.to_string(),
            border_width: "2px",
            border_style: "solid",
            border_color: conf.color.timberwolf.to_string(),
            div {
                class: "float",
                display: "flex",
                flex_direction: "row",
                justify_content: "center",
                align_items: "center",
                min_height: "100%",
                max_height: "100%",
                gap: "8px",
                transition: "transform 1s",
                transform: if let RouteState::Idle = *state.read() {
                    "translate(0, 0)"
                } else {
                    "translate(-100vw, 0)"
                },
                cmp::Spacer {
                    min_w: "2px",
                    max_w: "2px",
                    min_h: "128px",
                    max_h: "128px",
                    fill: conf.color.raisin_black.to_owned()
                }
                GuideCard {
                    label: "Build",
                    description: "Write your synth using rust compile to WASM.",
                    label_button: "build"
                }
                cmp::Spacer {
                    min_w: "2px",
                    max_w: "2px",
                    min_h: "128px",
                    max_h: "128px",
                    fill: conf.color.raisin_black.to_owned()
                }
                GuideCard {
                    label: "Deployment",
                    description: "Start deploying",
                    label_button: "deploy"
                }
                cmp::Spacer {
                    min_w: "2px",
                    max_w: "2px",
                    min_h: "128px",
                    max_h: "128px",
                    fill: conf.color.raisin_black.to_owned()
                }
                GuideCard {
                    label: "Maintainance",
                    description: "Start deploying",
                    label_button: "maintain"
                }
                cmp::Spacer {
                    min_w: "2px",
                    max_w: "2px",
                    min_h: "128px",
                    max_h: "128px",
                    fill: conf.color.raisin_black.to_owned()
                }
                GuideCard {
                    label: "Funding",
                    description: "Start deploying",
                    label_button: "fund"
                }
            }
        }
    )
}

#[component]
fn GuideCard(
    label: String,
    label_button: String,
    description: String,
) -> Element {
    let conf: theme::Conf = use_context();

    rsx!(
        div {
            display: "flex",
            flex_direction: "column",
            padding: "16px",
            min_width: "200px",
            max_width: "200px",
            min_height: "100%",
            div {
                cmp::Shape {
                    w: "16px",
                    h: "auto",
                    fill: conf.color.raisin_black.to_owned(),
                    model: cmp::ShapeModel::FourLobedFlower
                }
            }
            div {
                min_height: "2px"
            }
            div {
                display: "flex",
                flex_direction: "column",
                justify_content: "start",
                align_items: "start",
                h5 {
                    font_family: conf.font.borneox,
                    font_weight: "normal",
                    color: conf.color.raisin_black.to_string(),
                    { label }
                }
                div {
                    min_height: "2px"
                }
                h5 {
                    font_family: conf.font.brulia_test,
                    font_weight: "normal",
                    color: conf.color.raisin_black.to_string(),
                    text_align: "start",
                    { description }
                }
            }
            div {
                min_height: "16px"
            }
            div {
                cmp::Button {
                    { label_button }
                }
            }
        }
    )
}

#[component]
fn Feature(children: Element) -> Element {
    let conf: theme::Conf = use_context();
    
    rsx!(
        div {
            display: "flex",
            flex_direction: "column",
            justify_content: "start",
            align_items: "start",
            padding: "16px",
            border_width: "2px",
            border_style: "solid",
            border_color: conf.color.timberwolf.to_string(),
            border_radius: "2px",
            h3 {
                display: "flex",
                flex_direction: "row",
                justify_content: "center",
                align_items: "center",
                font_family: conf.font.borneox,
                font_weight: "bold",
                color: conf.color.timberwolf.to_string(),
                { children }
            }
        }
    )
}


#[derive(Props)]
#[derive(Clone)]
#[derive(PartialEq)]
struct ProblemInfograhicProps {
    pub children: Option<Element>
}

#[component]
fn ProblemInfograhic(props: ProblemInfograhicProps) -> Element {
    rsx!(
        div {
            display: "flex",
            flex_direction: "row",
            justify_content: "space-between",
            align_items: "center",
            min_width: "100%",
            max_width: "100%",
            min_height: "100%",
            max_height: "100%",
            padding: "16px",
            transition: "transform 1s",
            { props.children }
        }
    )
}


#[component]
fn ProblemCard() -> Element {
    let conf: theme::Conf = use_context();

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
            h2 {
                display: "flex",
                flex_direction: "row",
                font_family: conf.font.brulia_test,
                font_weight: "normal",
                color: conf.color.timberwolf.to_string(),
                gap: "4px",
                if *is_visible.read() {
                    span {
                        cmp::Counter {
                            from: 0.0,
                            to: 9000000000.0,
                            duration: time::Duration::from_millis(3000),
                            detail: Some(cmp::CounterPrecision::Single)
                        }
                    }
                }
            }
            h2 {
                font_family: conf.font.borneox,
                font_weight: "normal",
                color: conf.color.timberwolf.to_string(),
                "Billion People"
            }
        }
    )
}

#[component]
fn Container(
    fill: bool,
    children: Option<Element>
) -> Element {
    let conf: theme::Conf = use_context();

    rsx!(
        div {
            display: "flex",
            flex_direction: "column",
            justify_content: "start",
            align_items: "start",
            padding: "16px",
            min_width: "100%",
            max_width: "100%",
            min_height: "100%",
            max_height: "100%",
            background: if fill {
                conf.color.timberwolf.to_string()
            } else {
                "transparent".to_owned()
            },
            border_width: "1px",
            border_style: "solid",
            border_color: if fill {
                "transparent"
            } else {
                conf.color.timberwolf.to_string()
            },
            border_radius: "2px",
            { children }
        }
    )
}







#[component]
pub fn S() -> Element {
    let conf: theme::Conf = use_context();

    rsx!(
        div {
            display: "flex",
            flex_direction: "column",
            justify_content: "center",
            align_items: "center",
            gap: "8px",
            div {
                display: "flex",
                flex_direction: "row",
                justify_content: "center",
                align_items: "center",
                gap: "8px",
                Section {
                    mode: SectionMode::Full,
                    h1 {
                        color: conf.color.raisin_black.to_string(),
                        font_family: conf.font.brulia_test,
                        font_weight: "normal",
                        "Building software to be resilient is important"
                    }
                }
                Section {
                    mode: SectionMode::Silent,
                    div {
                        display: "flex",
                        flex_direction: "column",
                        justify_content: "center",
                        align_items: "center",
                        min_width: "100%",
                        flex: "1",
                        cmp::Shape {
                            w: "128px",
                            h: "128px",
                            model: cmp::ShapeModel::LayeredStructure,
                            fill: conf.color.timberwolf.to_owned()
                        }
                    }
                }
            }
            div {
                display: "flex",
                flex_direction: "row",
                justify_content: "center",
                align_items: "center",
                gap: "8px",
                Section {
                    mode: SectionMode::Outline,

                }
                Section {
                    mode: SectionMode::Full,

                }
            }
        }
    )
}






#[derive(Clone)]
#[derive(PartialEq)]
enum SectionMode {
    Outline,
    Full = 1,
    Silent
}

#[component]
fn Section(
    mode: SectionMode,
    children: Option<Element>
) -> Element {
    let conf: theme::Conf = use_context();

    rsx!(
        div {
            display: "flex",
            flex_direction: "column",
            justify_content: "start",
            align_items: "start",
            min_width: "700px",
            max_width: "700px",
            min_height: "300px",
            max_height: "300px",
            padding: "16px",
            border_width: if let SectionMode::Outline = mode {
                "2px"
            },
            border_style: if let SectionMode::Outline = mode {
                "solid"
            },
            border_color: if let SectionMode::Outline = mode {
                conf.color.timberwolf.to_string()
            },
            border_radius: "2px",
            background: if let SectionMode::Full = mode {
                conf.color.timberwolf.to_string()
            },
            { children }
        }
    )
}




#[component]
fn CouncilMemberRoleCard(
    #[props(default = None)]
    image: Option<Element>,

    #[props(default = None)]
    name: Option<Element>,

    #[props(default = None)]
    role: Option<Element>,

    #[props(extends = GlobalAttributes)]
    attr: Vec<Attribute>
) -> Element {
    let theme: theme::Theme = use_context();

    rsx!(
        div {
            display: "flex",
            flex_direction: "column",
            justify_content: "start",
            align_items: "start",
            min_width: "450px",
            max_width: "450px",
            ..attr,
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
fn CoreContributorCard(
    model: cmp::ShapeModel,
    name: String,
    description: String
) -> Element {
    let conf: theme::Conf = use_context();

    rsx!(
        div {
            display: "flex",
            flex_direction: "row",
            justify_content: "start",
            align_items: "start",
            div {
                display: "flex",
                flex_direction: "column",
                justify_content: "start",
                align_items: "start",
                border_width: "2px",
                border_style: "solid",
                border_color: conf.color.timberwolf.to_string(),
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
                    cmp::Shape {
                        model,
                        w: "64px",
                        h: "64px",
                        fill: conf.color.timberwolf.to_owned()
                    }
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
                border_color: conf.color.timberwolf.to_string(),
                background: conf.color.timberwolf.to_string(),
                h2 {
                    font_family: conf.font.borneox,
                    font_weight: "normal",
                    color: conf.color.raisin_black.to_string(),
                    { name }
                }
                h4 {
                    font_family: conf.font.brulia_test,
                    font_weight: "normal",
                    color: conf.color.raisin_black.to_string(),
                    { description }
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
                background: conf.color.medium_slate_blue.to_string()
            }
        }
    )
}

#[component]
fn HonourableMentionCard(icon: Element, name: Element) -> Element {
    let conf: theme::Conf = use_context();

    rsx!(
        div {
            display: "flex",
            flex_direction: "row",
            justify_content: "start",
            align_items: "center",
            padding: "16px",
            gap: "8px",
            { icon }
            h5 {
                font_family: conf.font.borneox,
                font_weight: "normal",
                color: conf.color.timberwolf.to_string(),
                { name }
            }
        }
    )
}