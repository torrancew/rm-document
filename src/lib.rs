pub(crate) use rm_lines as rm;

mod doc;
pub use doc::{Collection, CollectionError, Document, DocumentError, Entry, EntryError};

mod meta;
pub(crate) use meta::*;

mod page;
pub(crate) use page::*;

mod pdf;

mod private {
    pub trait Sealed {}

    impl Sealed for crate::Collection {}
    impl Sealed for crate::ContentData {}
    impl Sealed for crate::Document {}
    impl Sealed for crate::Entry {}
    impl Sealed for crate::MetaData {}
    impl Sealed for crate::PageData {}
    impl Sealed for crate::Page {}
    impl Sealed for crate::Template {}
}

mod template;
pub(crate) use template::Template;
pub use template::TemplateError;

pub trait TryLoad: Sized + private::Sealed {
    type Error: std::error::Error;

    fn try_load<P: AsRef<std::path::Path>>(path: P) -> Result<Self, Self::Error>;
}
