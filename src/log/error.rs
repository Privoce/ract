use std::fmt::{Debug, Display};

#[derive(Debug)]
pub enum Error {
    Toml(TomlError),
    AppIO(std::io::Error),
    Other{
        ty: Option<String>,
        msg: String,
    },
    AppExit
}

impl std::error::Error for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Toml(e) => e.fmt(f),
            Error::AppIO(e) => write!(f, "IO error: {}", e),
            Error::Other { ty, msg } => {
                if let Some(ty) = ty {
                    write!(f, "{}: {}", ty, msg)
                } else {
                    write!(f, "{}", msg)
                }
            },
            Error::AppExit => write!(f, "App exit"),
        }
    }
}

#[derive(Debug, Clone)]
pub enum TomlError {
    Parse,
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Error::AppIO(value)
    }
}
