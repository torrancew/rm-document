use crate::{rm, Document, Layout, Page, Template, TemplateError};

use printpdf as pdf;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum RenderError {
    #[error("Error while drawing line: {0}")]
    Drawing(#[from] pdf::Error),
    #[error("Invalid template path: {0}")]
    InvalidTemplate(#[from] TemplateError),
    #[error("Invalid template data: {0}")]
    CorruptTemplate(#[from] pdf::svg::SvgParseError),
}

#[allow(clippy::from_over_into)]
impl Document {
    pub fn render_pdf(&mut self) -> Result<pdf::PdfDocumentReference, RenderError> {
        let layout = self.orientation;
        let (width, height): (f64, f64) = layout.into();
        let (width, height) = (pdf::Pt(width).into(), pdf::Pt(height).into());
        let (doc, p, l) = pdf::PdfDocument::new(&self.name, width, height, "Template");

        let mut pages = self.pages.iter_mut();
        if let Some((page, template)) = pages.next() {
            if let Some(dir) = &self.template_dir {
                page.with_template(dir.join(template).with_extension("svg"))?;
            }
            page.render_into((doc.get_page(p), doc.get_page(p).get_layer(l)), layout)?;

            pages
                .map(|(page, template)| {
                    let (p, l) = doc.add_page(width, height, "Template");
                    if let Some(dir) = &self.template_dir {
                        page.with_template(dir.join(template).with_extension("svg"))?;
                    }
                    page.render_into((doc.get_page(p), doc.get_page(p).get_layer(l)), layout)
                })
                .collect::<Result<Vec<_>, RenderError>>()?;
        }

        Ok(doc)
    }
}

pub(crate) trait RenderInto {
    type Container;
    type Error: std::error::Error;

    fn render_into(&self, container: Self::Container, layout: Layout) -> Result<(), Self::Error>;
}

impl RenderInto for Page {
    type Container = (pdf::PdfPageReference, pdf::PdfLayerReference);
    type Error = RenderError;

    fn render_into(&self, container: Self::Container, layout: Layout) -> Result<(), Self::Error> {
        let (page, first_layer) = container;
        if self.template.is_some() {
            self.template
                .as_ref()
                .unwrap()
                .render_into(first_layer, layout)?;
        }
        self.inner.render_into(page, layout)
    }
}

impl RenderInto for Template {
    type Container = pdf::PdfLayerReference;
    type Error = RenderError;

    fn render_into(&self, container: Self::Container, _: Layout) -> Result<(), Self::Error> {
        let xform = pdf::svg::SvgTransform {
            scale_x: Some(3.),
            scale_y: Some(3.),
            ..Default::default()
        };
        printpdf::svg::Svg::parse(self.as_ref())?.add_to_layer(&container, xform);
        Ok(())
    }
}

impl RenderInto for rm::Layer {
    type Container = pdf::PdfLayerReference;
    type Error = RenderError;

    fn render_into(&self, container: Self::Container, layout: Layout) -> Result<(), Self::Error> {
        let (_, height): (f64, f64) = layout.into();
        self.lines.iter().for_each(|line| {
            let color = pdf::Color::Rgb(match line.color {
                rm::Color::Black => pdf::Rgb::new(0., 0., 0., None),
                rm::Color::Blue => pdf::Rgb::new(0., 0., 255., None),
                rm::Color::Grey => pdf::Rgb::new(50., 50., 50., None),
                rm::Color::Red => pdf::Rgb::new(255., 0., 0., None),
                rm::Color::White => pdf::Rgb::new(255., 255., 255., None),
            });

            container.set_outline_color(color);
            container.set_outline_thickness(line.size as f64);
            let points = line
                .points
                .iter()
                .map(|p| {
                    (
                        pdf::Point::new(
                            pdf::Pt(p.x as f64).into(),
                            pdf::Pt(height - p.y as f64).into(),
                        ),
                        false,
                    )
                })
                .collect();

            container.add_shape(pdf::Line {
                points,
                is_closed: false,
                has_fill: false,
                has_stroke: true,
                is_clipping_path: false,
            })
        });

        Ok(())
    }
}

impl RenderInto for rm::Page {
    type Container = pdf::PdfPageReference;
    type Error = RenderError;

    fn render_into(&self, container: Self::Container, layout: Layout) -> Result<(), Self::Error> {
        let _ = self
            .layers
            .iter()
            .enumerate()
            .map(|(idx, layer)| {
                layer.render_into(container.add_layer(format!("Layer {}", idx + 1)), layout)
            })
            .collect::<Result<Vec<_>, Self::Error>>()?;
        Ok(())
    }
}
