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
            font_family: format!("{}", theme.font.body),
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

    rsx!(
        div {
            style: format!(
                r#"
                    background: {};
                    -webkit-background-clip: text;
                    -webkit-text-fill-color: transparent;
                    background-clip: text;
                    color: transparent;
                    display: inline-block;
                "#,
                if let Some(bg) = bg {
                    bg
                } else {
                    theme.color.foreground.to_string()
                }
            ),
            ..more,
            { children }
        }
    )
}