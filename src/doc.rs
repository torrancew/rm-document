use crate::{ContentData, Layout, MetaData, Page, PageData, TryLoad};

use thiserror::Error;

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

#[derive(Debug)]
pub struct Document {
    pub(crate) name: String,
    pub(crate) pages: Vec<(Page, String)>,
    pub(crate) orientation: Layout,
}

impl TryLoad for Document {
    type Error = DocumentError;

    fn try_load<P: AsRef<std::path::Path>>(path: P) -> Result<Self, Self::Error> {
        let mdata = MetaData::try_load(path.as_ref().with_extension("metadata"))?;
        let cdata = ContentData::try_load(path.as_ref().with_extension("content"))?;
        let pdata = PageData::try_load(path.as_ref().with_extension("pagedata"))?;

        let contents: Result<Vec<_>, DocumentError> = cdata
            .pages
            .iter()
            .zip(pdata)
            .map(|(&pageid, tpl)| {
                Page::try_load(path.as_ref().join(pageid.to_string()).with_extension("rm"))
                    .map_err(DocumentError::from)
                    .map(|page| (page, tpl))
            })
            .collect();

        Ok(Self {
            name: mdata.visibleName,
            orientation: cdata.orientation,
            pages: contents?,
        })
    }
}
