use super::*;

#[component]
pub fn Button(
    is_locked: Option<bool>,

    #[props(default = None)]
    children: Option<Element>,

    #[props(extends = GlobalAttributes)]
    more: Vec<Attribute>
) -> Element {
    let mut hovering: Signal<bool> = use_signal(|| {
        false
    });
 
    let theme: theme::Theme = use_context();

    rsx! {
        button {
            onmouseenter: move |_| {
                if let Some(true) = is_locked {
                    return
                }
                *hovering.write() = true
            },
            onmouseleave: move |_| *hovering.write() = false,
            all: "unset",
            display: "flex",
            flex_direction: "column",
            justify_content: "start",
            align_items: "start",
            font_family: format!("{}", theme.font.body),
            font_weight: "normal",
            color: if *hovering.read() {
                format!("{}", theme.color.foreground)
            } else {
                format!("{}", theme.color.background)
            },
            border_width: "2px",
            border_style: "solid",
            border_image: format!("linear-gradient(to bottom right, {}) 1", theme.color.background),
            border_radius: "2px",
            background: if *hovering.read() {
                format!("{}", theme.color.background)
            } else {
                format!("{}", "transparent")
            },
            cursor: format!("url('{}'), auto", theme.cursor.finger),
            transition: "color linear 0.1s, background linear 0.1s",
            ..more,
            div {
                display: "flex",
                flex_direction: "row",
                justify_content: "start",
                align_items: "start",
                padding: "8px",
                opacity: if let Some(true) = is_locked {
                    "0.5"
                } else {
                    "1"
                },
                { children }
            }
        }
    }
}