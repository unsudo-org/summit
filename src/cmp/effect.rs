use super::*;

#[component]
pub fn RenderOnVisible(children: Option<Element>) -> Element {
    let mut visible: Signal<bool> = use_signal(|| false);
    
    rsx!(
        div {
            onvisible: move |_| *visible.write() = true,
            display: "contents",
            if *visible.read() {
                { children }
            }
        }
    )
}