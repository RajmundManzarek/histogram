use std::fmt;

#[derive(Debug)]
pub struct Params {
    pub min: u64,
    pub max: u64,
    pub graph_max: u64,
    pub files: Vec<String>,
}

impl fmt::Display for Params {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "min:{}, max:{}, files:{}, graph_max:{}", self.min, self.max, self.files.len(), self.graph_max)
    }
}
