use super::*;

#[derive(Debug)]
#[derive(Clone)]
#[derive(Copy)]
pub enum Direction {
    Top,
    TopLeft,
    TopRight,
    Bottom,
    BottomLeft,
    BottomRight,
    Left,
    Right,
    Angle(Angle)
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Top => write!(f, "to top"),
            Self::TopLeft => write!(f, "to top left"),
            Self::TopRight => write!(f, "to top right"),
            Self::Bottom => write!(f, "to bottom"),
            Self::BottomLeft => write!(f, "to bottom left"),
            Self::BottomRight => write!(f, "to bottom right"),
            Self::Left => write!(f, "to left"),
            Self::Right => write!(f, "to right"),
            Self::Angle(angle) => write!(f, "{}", angle)
        }
    }
}