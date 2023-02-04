use std::path::{Path, PathBuf};

use crate::{ContentData, Layout, MetaData, Page, PageData, TryLoad};

use thiserror::Error;

#[derive(Debug, Error)]
pub enum CollectionError {
    #[error(transparent)]
    MetaData(#[from] crate::MetaDataError),
    #[error("Document UUID is invalid: {0}")]
    Uuid(#[from] uuid::Error),
}

#[derive(Clone, Debug)]
pub struct Collection {
    name: String,
    parent: Option<uuid::Uuid>,
}

impl Collection {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn parent(&self) -> Option<&uuid::Uuid> {
        self.parent.as_ref()
    }
}

impl TryLoad for Collection {
    type Error = CollectionError;

    fn try_load<P: AsRef<std::path::Path>>(path: P) -> Result<Self, Self::Error> {
        let mdata = MetaData::try_load(path.as_ref().with_extension("metadata"))?;
        Ok(Self {
            name: mdata.visibleName,
            parent: mdata.parent.and_then(|p| uuid::Uuid::parse_str(&p).ok()),
        })
    }
}

#[derive(Debug, Error)]
pub enum DocumentError {
    #[error(transparent)]
    ContentData(#[from] crate::ContentDataError),
    #[error(transparent)]
    MetaData(#[from] crate::MetaDataError),
    #[error(transparent)]
    PageData(#[from] crate::PageDataError),
    #[error("Document did not parse correctly, and may contain corrupted contents")]
    Parse(#[from] crate::PageError),
    #[error("Document UUID is invalid: {0}")]
    Uuid(#[from] uuid::Error),
}

#[derive(Clone, Debug)]
pub struct Document {
    pub(crate) name: String,
    pub(crate) pages: Vec<(Page, String)>,
    pub(crate) parent: Option<uuid::Uuid>,
    pub(crate) orientation: Layout,
    pub(crate) template_dir: Option<PathBuf>,
}

impl Document {
    pub fn parent(&self) -> Option<&uuid::Uuid> {
        self.parent.as_ref()
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn set_template_dir(&mut self, path: &Path) {
        self.template_dir = Some(PathBuf::from(path));
    }

    pub fn with_template_dir<P: AsRef<Path>>(mut self, path: P) -> Self {
        self.set_template_dir(path.as_ref());
        self
    }
}

impl TryLoad for Document {
    type Error = DocumentError;

    fn try_load<P: AsRef<std::path::Path>>(path: P) -> Result<Self, Self::Error> {
        let mdata = MetaData::try_load(path.as_ref().with_extension("metadata"))?;
        let cdata = ContentData::try_load(path.as_ref().with_extension("content"))?;
        let pdata = PageData::try_load(path.as_ref().with_extension("pagedata"))?;

        let contents: Result<Vec<_>, crate::PageError> = cdata
            .pages
            .iter()
            .zip(pdata)
            .map(|(&pageid, tpl)| {
                Page::try_load(path.as_ref().join(pageid.to_string()).with_extension("rm"))
                    .map(|page| (page, tpl))
            })
            .collect();

        Ok(Self {
            name: mdata.visibleName,
            orientation: cdata.orientation,
            pages: contents?,
            parent: mdata.parent.and_then(|p| uuid::Uuid::parse_str(&p).ok()),
            template_dir: None,
        })
    }
}

#[derive(Debug, Error)]
pub enum EntryError {
    #[error(transparent)]
    Collection(#[from] CollectionError),
    #[error(transparent)]
    Document(#[from] DocumentError),
}

#[derive(Clone, Debug)]
pub enum Entry {
    Collection(Collection),
    Document(Document),
}

impl TryLoad for Entry {
    type Error = EntryError;

    fn try_load<P: AsRef<std::path::Path>>(path: P) -> Result<Self, Self::Error> {
        match Document::try_load(path.as_ref()) {
            Ok(doc) => Ok(Entry::Document(doc)),
            Err(e) => match e {
                DocumentError::MetaData(..) => Err(e.into()),
                _ => Ok(Entry::Collection(Collection::try_load(path)?)),
            },
        }
    }
}
