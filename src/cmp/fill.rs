use super::*;

#[derive(Props)]
#[derive(Clone)]
#[derive(PartialEq)]
pub struct FillProps {

    #[props(default = None)]
    pub children: Option<Element>,

    #[props(extends = GlobalAttributes)]
    pub attr: Vec<Attribute>
}

#[component]
pub fn Fill(props: FillProps) -> Element {
    rsx! {
        div {
            min_width: "100%",
            max_width: "100%",
            min_height: "100%",
            max_height: "100%",
            { props.children }
        }
    }
}