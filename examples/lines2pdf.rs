use anyhow::Result;
use printpdf::*;
use rm_lines::{self as lines, Parse};

use std::{fs::File, io::BufWriter};

fn main() -> Result<()> {
    let (doc, p, l) = PdfDocument::new(
        "Test Document",
        Pt(1404.).into(),
        Pt(1872.).into(),
        "Layer 1",
    );
    let layer = doc.get_page(p).get_layer(l);

    let (_, data) = lines::Layer::parse(include_bytes!("../../rm-lines/data/test-layer.rm"))?;
    data.lines.iter().for_each(|l| {
        let points = l
            .points
            .iter()
            .map(|p| {
                (
                    Point::new(Pt(p.x as f64).into(), Pt((1872. - p.y) as f64).into()),
                    false,
                )
            })
            .collect();

        let line = Line {
            points,
            is_closed: false,
            has_fill: false,
            has_stroke: true,
            is_clipping_path: false,
        };

        let black = Color::Rgb(Rgb::new(0., 0., 0., None));
        layer.set_outline_color(black);
        layer.set_outline_thickness(2.);

        layer.add_shape(line);
    });
    Ok(doc.save(&mut BufWriter::new(File::create("/tmp/test-layer.pdf")?))?)
}
