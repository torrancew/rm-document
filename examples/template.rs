use rm_document::{self as rm, TryLoad};

use anyhow::Result;
use std::{fs::File, io::BufWriter};

fn main() -> Result<()> {
    Ok(
        rm::Document::try_load("./data/713e619e-ed13-4157-81d6-9a6a311cf99b")?
            .with_template_dir("/home/tray/share/remarkable/templates")
            .render_pdf()?
            .save(&mut BufWriter::new(File::create("/tmp/template.pdf")?))?,
    )
}
