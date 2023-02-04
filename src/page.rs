use crate::{
    rm::{self, Parse},
    Template, TemplateError, TryLoad,
};

use std::{
    fs,
    io::{self, Read},
    path,
};

use thiserror::Error;

#[derive(Debug, Error)]
pub enum PageError {
    #[error(transparent)]
    Io(#[from] io::Error),
    #[error("Page failed to parse: {0}")]
    Parse(String),
}

#[derive(Clone, Debug)]
pub(crate) struct Page {
    pub(crate) template: Option<Template>,
    pub(crate) inner: rm::Page,
}

impl Page {
    pub(crate) fn with_template<P: AsRef<path::Path>>(
        &mut self,
        path: P,
    ) -> Result<&mut Self, TemplateError> {
        self.template = Some(Template::try_load(path)?);
        Ok(self)
    }
}

impl TryLoad for Page {
    type Error = PageError;

    fn try_load<P: AsRef<std::path::Path>>(path: P) -> Result<Self, Self::Error> {
        let mut bytes = vec![];
        fs::File::open(path)?.read_to_end(&mut bytes)?;
        rm::Page::parse(&bytes)
            .map_err(|e| PageError::Parse(e.to_string()))
            .map(|(_, page)| Self {
                template: None,
                inner: page,
            })
    }
}
