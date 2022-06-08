use crate::{
    rm::{self, Parse},
    TryLoad,
};

use std::{
    fs,
    io::{self, Read},
};

use thiserror::Error;

#[derive(Debug, Error)]
pub enum PageError {
    #[error(transparent)]
    Io(#[from] io::Error),
    #[error("Page failed to parse: {0}")]
    Parse(String),
}

#[derive(Debug)]
pub(crate) struct Page {
    pub(crate) inner: rm::Page,
}

impl TryLoad for Page {
    type Error = PageError;

    fn try_load<P: AsRef<std::path::Path>>(path: P) -> Result<Self, Self::Error> {
        let mut bytes = vec![];
        fs::File::open(path)?.read_to_end(&mut bytes)?;
        rm::Page::parse(&bytes)
            .map_err(|e| PageError::Parse(e.to_string()))
            .map(|(_, page)| Self { inner: page })
    }
}