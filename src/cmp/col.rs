use super::*;

#[derive(Props)]
#[derive(Clone)]
#[derive(PartialEq)]
pub struct ColProps {
    
    #[props(default = None)]
    pub align: Option<Element>,

    #[props(default = None)]
    pub align_cross: Option<Element>,

    #[props(default = None)]
    pub children: Option<Element>,

    #[props(extends = GlobalAttributes)]
    pub attr: Vec<Attribute>
}

#[component]
pub fn Col(props: ColProps) -> Element {
    rsx! {
        div {
            display: "flex",
            flex_direction: "column",
            justify_content: props.align,
            align_items: props.align_cross,
            ..props.attr,
            { props.children }
        }
    }
}

#[derive(Props)]
#[derive(Clone)]
#[derive(PartialEq)]
pub struct ColFillProps {

    #[props(default = None)]
    pub align: Option<Element>,

    #[props(default = None)]
    pub align_cross: Option<Element>,

    #[props(default = None)]
    pub children: Option<Element>,    

    #[props(extends = GlobalAttributes)]
    pub attr: Vec<Attribute>
}

#[component]
pub fn ColFill(props: ColFillProps) -> Element {
    rsx! {
        div {
            display: "flex",
            flex_direction: "column",
            justify_content: props.align,
            align_items: props.align_cross,
            min_width: "100%",
            max_width: "100%",
            min_height: "100%",
            max_height: "100%",
            ..props.attr,
            { props.children }
        }
    }
}