pub(crate) use rm_lines as rm;

mod doc;
pub use doc::Document;

mod meta;
pub(crate) use meta::*;

mod page;
pub(crate) use page::*;

mod pdf;

mod private {
    pub trait Sealed {}

    impl Sealed for crate::Document {}
    impl Sealed for crate::ContentData {}
    impl Sealed for crate::MetaData {}
    impl Sealed for crate::PageData {}
    impl Sealed for crate::Page {}
}

pub trait TryLoad: Sized + private::Sealed {
    type Error: std::error::Error;

    fn try_load<P: AsRef<std::path::Path>>(path: P) -> Result<Self, Self::Error>;
}
