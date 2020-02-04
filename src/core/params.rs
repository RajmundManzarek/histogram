use std::fmt;

#[derive(Debug)]
pub struct Params {
    pub min: u64,
    pub max: u64,
    pub title: String,
    pub sub_title: String,
    pub graph_max: u64,
    pub files: Vec<String>,
}

impl fmt::Display for Params {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut out = format!("min:{}, max:{}, files:{}, graph_max:{}, title:‘{}’", self.min, self.max, self.files.len(), self.graph_max, self.title);

        if self.sub_title.len() > 0 {
            out.push_str(&format!(", sub_title:‘{}’", self.sub_title));
        }

        write!(f, "{}", out)
    }
}
