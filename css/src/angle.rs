use super::*;

#[derive(Debug)]
#[derive(Clone)]
#[derive(Copy)]
pub struct Angle {
    n: f32
}

impl fmt::Display for Angle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}deg", self.n)
    }
}