use super::*;

#[repr(u8)]
#[derive(Clone)]
#[derive(PartialEq)]
pub enum PageScrollSnap {
    Mandatory,
    Proximity
}

#[component]
pub fn Page(
    scroll_snap: Option<PageScrollSnap>, 
    surface: Option<Element>,
    class: Option<String>,
    style: Option<String>,
    children: Option<Element>
) -> Element {
    rsx!(
        div {
            class: class,
            style: format!(
                r#"
                    position: relative;
                    display: flex;
                    flex-direction: column;
                    justify-content: start;
                    align-items: center;
                    min-width: 100vw;
                    max-width: 100vw;
                    min-height: 100vh;
                    max-height: 100vh;
                    overflow-x: hidden;
                    overflow-y: hidden;
                    cursor: url('{}'), auto;
                    {}
                "#,
                cursor::DEFAULT,
                style.unwrap_or_default()
            ),
            if let Some(surface) = surface {
                div {
                    style: format!(
                        r#"
                            z-index: 67000000;
                            position: absolute;
                            display: flex;
                            flex-direction: column;
                            justify-content: center;
                            align-items: center;
                            min-width: 100%;
                            max-width: 100%;
                            min-height: 100%;
                            max-height: 100%;
                            overflow-x: hidden;
                            overflow-y: hidden;
                            pointer-events: none;
                        "#
                    ),
                    { surface }
                }
            }
            div {
                style: format!(
                    r#"
                        display: flex;
                        flex-direction: column;
                        justify-content: start;
                        align-items: center;
                        min-width: 100%;
                        max-width: 100%;
                        overflow-x: hidden;
                        overflow-y: auto;
                        scroll-snap-type: {};
                        scroll-behaviour: smooth;
                    "#,
                    match scroll_snap {
                        Some(PageScrollSnap::Mandatory) => "y mandatory",
                        Some(PageScrollSnap::Proximity) => "y proximity",
                        None => "none"
                    }
                ),
                { children }
            }
        }
    )
}

#[component]
pub fn PageItem(
    class: Option<String>,
    style: Option<String>,
    children: Option<Element>
) -> Element {
    rsx!(
        div {
            class: class,
            style: format!(
                r#"
                    display: flex;
                    flex-direction: column;
                    justify-content: center;
                    align-items: center;
                    min-width: 100vw;
                    max-width: 100vw;
                    min-height: 100vh;
                    max-height: 100vh;
                    scroll-snap-align: start;
                    {}
                "#,
                style.unwrap_or_default()
            ),
            { children }
        }
    )
}