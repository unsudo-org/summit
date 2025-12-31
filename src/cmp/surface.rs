use super::*;

#[derive(Props, Clone, PartialEq)]
pub struct SurfaceProps {
    pub children: Option<Element>
}

#[component]
pub fn Surface(props: SurfaceProps) -> Element {
    rsx! {
        AutoGrid {
            col_count: 1,
            row_count: 1,
            { props.children }
        }
    }
}