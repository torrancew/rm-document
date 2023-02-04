use crate::TryLoad;

use std::{
    fs,
    io::Read,
    path::{self, PathBuf},
};

use thiserror::Error;

#[derive(Debug, Error)]
#[error("Unable to load template at {0}")]
pub struct TemplateError(PathBuf);

#[derive(Clone, Debug)]
pub(crate) struct Template(String);

impl AsRef<str> for Template {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl TryLoad for Template {
    type Error = TemplateError;

    fn try_load<P: AsRef<path::Path>>(path: P) -> Result<Self, Self::Error> {
        let path = path.as_ref();
        let mut contents = String::new();

        fs::File::open(path)
            .map_err(|_| TemplateError(path.into()))?
            .read_to_string(&mut contents)
            .map_err(|_| TemplateError(path.into()))?;

        Ok(Self(contents))
    }
}
