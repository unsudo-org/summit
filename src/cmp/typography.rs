use super::*;

macro_rules! heading {
    ($($name:ident $element:ident)*) => {
        $(
            #[component]
            pub fn $name(
                children: Option<Element>,

                #[props(extends = GlobalAttributes)]
                more: Vec<Attribute>
            ) -> Element {
                let theme: theme::Theme = use_context();

                rsx!(
                    $element {
                        font_family: format!("{}", theme.font.display),
                        font_weight: "normal",
                        color: format!("{}", theme.color.foreground),
                        ..more,
                        { children }
                    }
                )
            }
        )*
    };
}

heading!(
    H1 h1
    H2 h2
    H3 h3
    H4 h4
    H5 h5
    H6 h6
);

#[component]
pub fn P(
    children: Option<Element>,

    #[props(extends = GlobalAttributes)]
    more: Vec<Attribute>
) -> Element {
    let theme: theme::Theme = use_context();

    rsx!(
        p {
            font_family: format!("{}", "br cobane"),
            font_weight: "normal",
            color: format!("{}", theme.color.foreground),
            ..more,
            { children }
        }
    )
}

#[component]
pub fn Gradient(
    children: Option<Element>,
    bg: Option<String>,

    #[props(extends = GlobalAttributes)]
    more: Vec<Attribute>
) -> Element {
    let theme: theme::Theme = use_context();
    let bg: String = if let Some(bg) = bg {
        bg
    } else {
        theme.color.foreground.to_string()
    };
    let mut style: String = String::new();
    style.push_str(&format!("background: {};", bg));
    style.push_str("-webkit-background-clip: text;");
    style.push_str("-webkit-text-fill-color: transparent;");
    style.push_str("background-clip: text;");
    style.push_str("color: transparent;");
    style.push_str("display: inline-block;");

    rsx!(
        div {
            style,
            ..more,
            { children }
        }
    )
}

#[component]
pub fn Section(
    heading: Option<Element>,
    paragraphs: Vec<Element>
) -> Element {
    rsx!(
        div {
            display: "flex",
            flex_direction: "column",
            justify_content: "start",
            align_items: "start",
            gap: "16px",
            { heading }
            div {
                display: "flex",
                flex_direction: "column",
                justify_content: "start",
                align_items: "start",
                gap: "8px",
                for paragraph in paragraphs {
                    { paragraph }
                }
            }
        }
    )
}