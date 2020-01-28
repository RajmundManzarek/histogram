use std::fmt;

#[derive(Debug)]
pub struct Params {
    pub min: u64,
    pub max: u64,
}

impl fmt::Display for Params {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "min:{}, max:{}", self.min, self.max)
    }
}

