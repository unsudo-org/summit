use super::*;

#[derive(Debug)]
#[derive(Clone)]
#[derive(Copy)]
#[derive(PartialEq)]
#[derive(Eq)]
pub enum GradientShape {
    Circle,
    Ellipse
}

impl fmt::Display for GradientShape {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Circle => write!(f, "{}", "circle"),
            Self::Ellipse => write!(f, "{}", "ellipse")
        }
    }
}

#[derive(Debug)]
#[derive(Clone)]
pub enum Gradient {
    Linear(LinearGradient),
    Radial(RadialGradient),
    Conic(ConicGradient)
}

pub struct Gradients(Vec<Gradient>);

impl fmt::Display for Gradients {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s: String = self.0
            .iter()
            .map(|gradient| match gradient {
                Gradient::Linear(l) => l.to_string(),
                Gradient::Radial(r) => r.to_string(),
                Gradient::Conic(c) => c.to_string()
            })
            .collect::<Vec<_>>()
            .join(", ");
        write!(f, "{}", s)
    }
}

#[derive(Debug)]
#[derive(Clone)]
pub struct LinearGradient {
    direction: Direction,
    stops: Vec<ColorStop>
}

impl fmt::Display for LinearGradient {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let stops: String = self.stops
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<_>>()
            .join(", ");
        write!(f, "linear-gradient({}, {})", self.direction, stops)
    }
}

#[derive(Debug)]
#[derive(Clone)]
pub struct RadialGradient {
    shape: GradientShape,
    stops: Vec<ColorStop>
}

impl fmt::Display for RadialGradient {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let stops: String = self.stops
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<_>>()
            .join(", ");
        write!(f, "radial-gradient({}, closest-side, {})", self.shape, stops)
    }
}

#[derive(Debug)]
#[derive(Clone)]
pub struct ConicGradient {
    angle: Angle,
    stops: Vec<ColorStop>
}

impl fmt::Display for ConicGradient {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let stops: String = self.stops
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<_>>()
            .join(", ");
        write!(f, "conic-gradient(from {}, {})", self.angle, stops)
    }
}