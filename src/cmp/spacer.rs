use super::*;

#[component]
pub fn Spacer(
    min_w: Option<String>,
    max_w: Option<String>,
    min_h: Option<String>,
    max_h: Option<String>,
    fill: Option<Hex>
) -> Element {
    rsx!(
        div {
            display: "flex",
            flex_direction: "row",
            justify_content: "center",
            align_items: "center",
            min_width: min_w,
            max_width: max_w,
            min_height: min_h,
            max_height: max_h,
            background: if let Some(fill) = fill {
                fill.to_string()
            }
        }
    )
}




pub enum Typography {
    Caption,
    Body,
    BodyStrong,
    Heading,
    Title
}


#[derive(Debug)]
#[derive(Clone)]
#[derive(Copy)]
#[derive(PartialEq)]
#[derive(Eq)]
pub enum Space {
    VeryTiny,
    Tiny,
    Small,
    Medium,
    Large,
    VeryLarge,
    Huge,
    Massive
}

impl Space {
    pub fn absolute(&self) -> f64 {
        match self {
            Self::VeryTiny => 2.0,
            Self::Tiny => 4.0,
            Self::Small => 8.0,
            Self::Medium => 16.0,
            Self::Large => 32.0,
            Self::VeryLarge => 64.0,
            Self::Huge => 128.0,
            Self::Massive => 256.0
        }
    }
}

impl std::fmt::Display for Space {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.absolute())
    }
}


pub enum Density {
    Compact,
    Normal,
    Comfortable
}

impl Density {
    pub fn padding(&self) -> Space {
        match self {
            Self::Compact => Space::Small,
            Self::Normal => Space::Medium,
            Self::Comfortable => Space::Large
        }
    }
}

impl std::fmt::Display for Density {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.padding())
    }
}










use std::fmt;

trait CssExt
where 
    Self: fmt::Display {}

#[derive(Debug)]
#[derive(Clone)]
#[derive(Copy)]
#[derive(PartialEq)]
#[derive(Eq)]
pub enum Ease {
    Linear,
    Ease,
    EaseInOut,
    EaseIn,
    EaseOut,
    StepStart,
    StepEnd,
    Step(u32),
    Cubic(f32, f32, f32, f32)
}

impl CssExt for Ease {}

impl fmt::Display for Ease {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Linear => write!(f, "{}", "linear"),
            Self::Ease => write!(f, "{}", "ease"),
            Self::EaseInOut => write!(f, "{}", "ease-in-out"),
            Self::EaseIn => write!(f, "{}", "ease-in"),
            Self::EaseOut => write!(f, "{}", "ease-out"),
            Self::StepStart => write!(f, "{}", "step-start"),
            Self::StepEnd => write!(f, "{}", "step-end"),
            Self::Step(n) => write!(f, "steps({})", n),
            Self::Cubic(
                x_0,
                y_0,
                x_1,
                y_1
            ) => write!(f, "cubic-bezier({}, {}, {}, {})", x_0, y_0, x_1, y_1)
        }
    }
}





pub struct TransitionParam {
    properties: Vec<Property>,
    duration: time::Duration
}

impl TransitionParam {
    pub fn new() -> Self {
        Self {
            properties: vec![],
            duration: time::Duration::default()
        }
    }


}

impl fmt::Display for TransitionParam {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        
    }
}


mod css {

}



pub struct Idle {

}

pub struct ParatialParam;

impl Transition<ParatialParam> {
    pub fn opacity(mut self) -> Self {
        self.properties.push(Property::Opacity);
        self
    }

    pub fn visibility(mut self) -> Self {
        self.properties.push(Property::Visibility);
        self
    }

    pub fn gap(mut self) -> Self {
        self.properties.push(Property::Gap);
        self
    }
}

pub struct PartialDurationParam;


pub struct PartialCompletion;

impl Transition {
    pub fn param(self) -> Transition<ParatialParam> {
        self
    }
}


pub struct Complete;

impl fmt::Display for Transition<Complete> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        
    }
}


Transition::new()
    .param()
    .gap()
    .padding()
    .with_duration(3000)
    .stack()

    .param()
    .all()
    .with_duration(5000)
    .stack()

    .build()