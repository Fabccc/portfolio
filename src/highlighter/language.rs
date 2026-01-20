use std::fmt::Display;

#[derive(Debug, Clone, PartialEq)]
pub enum Lang {
    Yaml,
    Dockerfile,
    Shell
}


impl Display for Lang {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Lang::Yaml => "Yaml",
            Lang::Dockerfile => "Dockerfile",
            Lang::Shell => "Shell"
        })
    }
}