use super::*;

#[derive(Props, Clone, PartialEq)]
pub struct LayerProps {
    pub children: Option<Element>
}

#[component]
pub fn Layer(props: LayerProps) -> Element {
    rsx! {
        GridItem {
            from_x: 0,
            from_y: 0,
            to_x: 1,
            to_y: 1,
            { props.children }
        }
    }
}