use super::*;

#[derive(Debug)]
#[derive(Clone)]
#[derive(Copy)]
#[derive(PartialEq)]
pub enum Easing {
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

impl fmt::Display for Easing {
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