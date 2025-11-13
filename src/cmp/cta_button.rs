use super::*;

#[component]
pub fn CtaButton(
    class: Option<String>,
    style: Option<String>,
    children: Option<Element>
) -> Element {
    let conf: conf::Conf = use_context();
    let mut hover: Signal<bool> = use_signal(|| false);

    rsx!(
        button {
            onmouseenter: move |_| *hover.write() = true,
            onmouseleave: move |_| *hover.write() = false,
            class: class,
            style: format!(
                r#"
                    all: unset;
                    display: flex;
                    flex-direction: column;
                    justify-content: center;
                    align-items: center;
                    font-size: 1em;
                    font-family: {};
                    font-weight: normal;
                    color: {};
                    cursor: url('{}'), auto;
                    border-width: 1px;
                    border-style: solid;
                    border-image: {} 1;
                    border-radius: 2px;
                    padding: 8px;
                    background: {};
                    {}
                "#,
                font::BR_COBANE,
                if *hover.read() {
                    conf::RAISIN_BLACK
                } else {
                    conf::SILVER
                },
                cursor::FINGER,
                if *hover.read() {
                    format!("linear-gradient(to bottom right, {})", conf::SILVER)
                } else {
                    format!("linear-gradient(to bottom right, {}, {}, {})", conf::MEDIUM_SLATE_BLUE, conf::ROSE_POMPADOUR, conf::MINDARO)
                },
                if *hover.read() {
                    format!("linear-gradient(to right bottom, {}, {}, {})", conf::MEDIUM_SLATE_BLUE, conf::ROSE_POMPADOUR, conf::MINDARO)
                } else {
                    "transparent".to_owned()
                },
                style.unwrap_or_default()
            ),
            { children }
        }
    )
}