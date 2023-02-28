use super::Binary;

#[derive(Default)]
pub struct Equation {

}

impl Equation {

    pub fn solve(&self) -> Option<Binary> {
        Some(Binary::High)
    }
}

impl From<String> for Equation {
    fn from(value: String) -> Self {
        Self::default()
    }
}