use crate::TryLoad;

use std::{
    fs,
    io::{self, BufRead},
};

use serde::Deserialize;
use thiserror::Error;
use uuid::Uuid;

#[derive(Debug, Error)]
pub enum ContentDataError {
    #[error(transparent)]
    Io(#[from] io::Error),
    #[error(transparent)]
    Parse(#[from] serde_json::Error),
}

#[allow(non_snake_case)]
#[derive(Deserialize)]
pub(crate) struct ContentData {
    pub orientation: Layout,
    pub pages: Vec<Uuid>,
}

impl TryLoad for ContentData {
    type Error = ContentDataError;

    fn try_load<P: AsRef<std::path::Path>>(path: P) -> Result<Self, Self::Error> {
        Ok(serde_json::from_reader(io::BufReader::new(
            fs::File::open(path)?,
        ))?)
    }
}

#[derive(Clone, Copy, Debug, Deserialize)]
pub(crate) enum Layout {
    #[serde(rename = "landscape")]
    Landscape,
    #[serde(rename = "portrait")]
    Portrait,
}

#[allow(clippy::from_over_into)]
impl Into<(f64, f64)> for Layout {
    fn into(self) -> (f64, f64) {
        let (width, height) = (1404., 1872.);
        match self {
            Self::Landscape => (height, width),
            Self::Portrait => (width, height),
        }
    }
}

#[derive(Debug, Error)]
pub enum MetaDataError {
    #[error(transparent)]
    Io(#[from] io::Error),
    #[error(transparent)]
    Parse(#[from] serde_json::Error),
}

#[allow(non_snake_case)]
#[derive(Deserialize)]
pub(crate) struct MetaData {
    pub(crate) visibleName: String,
}

impl TryLoad for MetaData {
    type Error = MetaDataError;

    fn try_load<P: AsRef<std::path::Path>>(path: P) -> Result<Self, Self::Error> {
        Ok(serde_json::from_reader(io::BufReader::new(
            fs::File::open(path)?,
        ))?)
    }
}

#[derive(Debug, Error)]
pub enum PageDataError {
    #[error(transparent)]
    Io(#[from] io::Error),
}

#[derive(Debug)]
pub(crate) struct PageData(Vec<String>);

impl IntoIterator for PageData {
    type Item = String;
    type IntoIter = std::vec::IntoIter<String>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl TryLoad for PageData {
    type Error = PageDataError;

    fn try_load<P: AsRef<std::path::Path>>(path: P) -> Result<Self, Self::Error> {
        let file = io::BufReader::new(fs::File::open(path.as_ref())?);
        let data: Result<Vec<String>, Self::Error> = file
            .lines()
            .map(|line| line.map_err(PageDataError::from).map(String::from))
            .collect();

        data.map(Self)
    }
}
