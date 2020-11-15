// plotters-conrod
//
// Conrod backend for Plotters
// Copyright: 2020, Valerian Saliou <valerian@valeriansaliou.name>
// License: MIT

use std::convert::From;

use conrod_core::{self as conrod, Positionable, Widget};
use plotters_backend::{
    text_anchor, BackendColor, BackendCoord, BackendStyle, BackendTextStyle, DrawingBackend,
    DrawingErrorKind,
};

use crate::error::ConrodBackendError;
use crate::graph::ConrodBackendReusableGraph;
use crate::utils::{color, convert, path, position, shape};

/// The Conrod drawing backend
pub struct ConrodBackend<'a, 'b> {
    ui: &'a mut conrod::UiCell<'b>,
    size: (u32, u32),
    parent: conrod::widget::Id,
    font: conrod::text::font::Id,
    graph: &'a mut ConrodBackendReusableGraph,
}

impl<'a, 'b> ConrodBackend<'a, 'b> {
    /// Create a new Conrod backend drawer, with:
    /// - `ui`: the `UiCell` that was derived from `Ui` for this frame
    /// - `(plot_width, plot_height)`: the size of your plot in pixels (make sure it matches its parent canvas size)
    /// - `ids.parent`: the `widget::Id` of the canvas that contains your plot (of the same size than the plot itself)
    /// - `fonts.regular`: the `font::Id` of the font to use to draw text (ie. a Conrod font identifier)
    /// - `conrod_graph`: a mutable reference to the graph instance you built outside of the drawing loop (pass it as a mutable reference)
    pub fn new(
        ui: &'a mut conrod::UiCell<'b>,
        size: (u32, u32),
        parent: conrod::widget::Id,
        font: conrod::text::font::Id,
        graph: &'a mut ConrodBackendReusableGraph,
    ) -> Self {
        // Important: prepare the IDs graph, and reset all incremented IDs counters back to zero; \
        //   if we do not do that, counts will increment forever and the graph will be enlarged \
        //   infinitely, which would result in a huge memory leak.
        graph.prepare();

        Self {
            ui,
            parent,
            font,
            size,
            graph,
        }
    }
}

impl<'a, 'b> DrawingBackend for ConrodBackend<'a, 'b> {
    type ErrorType = ConrodBackendError;

    fn get_size(&self) -> (u32, u32) {
        self.size
    }

    fn ensure_prepared(&mut self) -> Result<(), DrawingErrorKind<ConrodBackendError>> {
        Ok(())
    }

    fn present(&mut self) -> Result<(), DrawingErrorKind<ConrodBackendError>> {
        Ok(())
    }

    fn draw_pixel(
        &mut self,
        _point: BackendCoord,
        _color: BackendColor,
    ) -> Result<(), DrawingErrorKind<Self::ErrorType>> {
        // Not supported yet (rendering ignored)
        // Notice: doing this efficiently would require building an internal buffer on 'self', and \
        //   rendering it as a Conrod image widget when the final call to 'present()' is done. \
        //   doing it solely by drawing Conrod rectangle primitives from there has been deemed \
        //   super inefficient. Note that this buffer would be shared with 'blit_bitmap()', and \
        //   thus alpha-channel pixels would need to be blended accordingly.

        Ok(())
    }

    fn draw_line<S: BackendStyle>(
        &mut self,
        from: BackendCoord,
        to: BackendCoord,
        style: &S,
    ) -> Result<(), DrawingErrorKind<Self::ErrorType>> {
        // Acquire absolute position generator (in parent container)
        if let Some(position) = position::PositionParent::from(&self.ui, self.parent) {
            // Generate line style
            let line_style = conrod::widget::primitive::line::Style::solid()
                .color(color::Color::from(&style.color()).into())
                .thickness(style.stroke_width() as _);

            // Render line widget
            conrod::widget::line::Line::abs_styled(
                position.abs_point_f64(&from),
                position.abs_point_f64(&to),
                line_style,
            )
            .top_left_of(self.parent)
            .set(self.graph.line.next(&mut self.ui), &mut self.ui);

            Ok(())
        } else {
            Err(DrawingErrorKind::DrawingError(
                ConrodBackendError::NoParentPosition,
            ))
        }
    }

    fn draw_rect<S: BackendStyle>(
        &mut self,
        upper_left: BackendCoord,
        bottom_right: BackendCoord,
        style: &S,
        fill: bool,
    ) -> Result<(), DrawingErrorKind<Self::ErrorType>> {
        // Generate rectangle style
        let rectangle_style = if fill {
            conrod::widget::primitive::shape::Style::fill_with(
                color::Color::from(&style.color()).into(),
            )
        } else {
            conrod::widget::primitive::shape::Style::outline_styled(
                conrod::widget::primitive::line::Style::new()
                    .color(color::Color::from(&style.color()).into())
                    .thickness(style.stroke_width() as _),
            )
        };

        // Render rectangle widget
        conrod::widget::rectangle::Rectangle::styled(
            [
                (bottom_right.0 - upper_left.0) as _,
                (bottom_right.1 - upper_left.1) as _,
            ],
            rectangle_style,
        )
        .top_left_with_margins_on(self.parent, upper_left.1 as _, upper_left.0 as _)
        .set(self.graph.rect.next(&mut self.ui), &mut self.ui);

        Ok(())
    }

    fn draw_path<S: BackendStyle, I: IntoIterator<Item = BackendCoord>>(
        &mut self,
        path: I,
        style: &S,
    ) -> Result<(), DrawingErrorKind<Self::ErrorType>> {
        // Acquire absolute position generator (in parent container)
        if let Some(position) = position::PositionParent::from(&self.ui, self.parent) {
            // Generate line style
            let line_style = conrod::widget::primitive::line::Style::solid()
                .color(color::Color::from(&style.color()).into())
                .thickness(style.stroke_width() as _);

            // Render point path widget
            conrod::widget::point_path::PointPath::abs_styled(
                path.into_iter()
                    .map(|point| position.abs_point_f64(&point))
                    .collect::<Vec<conrod::position::Point>>(),
                line_style,
            )
            .top_left_of(self.parent)
            .set(self.graph.path.next(&mut self.ui), &mut self.ui);

            Ok(())
        } else {
            Err(DrawingErrorKind::DrawingError(
                ConrodBackendError::NoParentPosition,
            ))
        }
    }

    fn draw_circle<S: BackendStyle>(
        &mut self,
        center: BackendCoord,
        radius: u32,
        style: &S,
        fill: bool,
    ) -> Result<(), DrawingErrorKind<Self::ErrorType>> {
        // Generate circle style
        let circle_style = if fill {
            conrod::widget::primitive::shape::Style::fill_with(
                color::Color::from(&style.color()).into(),
            )
        } else {
            conrod::widget::primitive::shape::Style::outline_styled(
                conrod::widget::primitive::line::Style::new()
                    .color(color::Color::from(&style.color()).into())
                    .thickness(style.stroke_width() as _),
            )
        };

        // Render circle widget
        conrod::widget::circle::Circle::styled(radius as f64, circle_style)
            .top_left_with_margins_on(
                self.parent,
                (center.1 - radius as i32) as f64,
                (center.0 - radius as i32) as f64,
            )
            .set(self.graph.circle.next(&mut self.ui), &mut self.ui);

        Ok(())
    }

    fn fill_polygon<S: BackendStyle, I: IntoIterator<Item = BackendCoord>>(
        &mut self,
        vert: I,
        style: &S,
    ) -> Result<(), DrawingErrorKind<Self::ErrorType>> {
        // Acquire absolute position generator (in parent container)
        if let Some(position) = position::PositionParent::from(&self.ui, self.parent) {
            // Paint a simplified path, where empty areas are removed and un-necessary points are \
            //   cleared. This is required for triangulation to work properly, and it reduces \
            //   the number of triangles on screen to a strict minimum.
            let simplified_path: Vec<_> = path::PathSimplifier::from(
                vert.into_iter()
                    .map(|vertex| position.abs_point_i32(&vertex)),
            )
            .collect();

            // Find closed shapes (eg. when the plot area goes from positive to negative, we need \
            //   to split the path into two distinct paths, otherwise we will not be able to \
            //   triangulate properly, and thus we will not be able to fill the shape)
            if let Ok(mut shape_splitter) = shape::ShapeSplitter::try_from(&simplified_path) {
                // Generate polygon style
                let polygon_style = conrod::widget::primitive::shape::Style::fill_with(
                    color::Color::from(&style.color()).into(),
                );

                // Triangulate the polygon points, giving back a list of triangles that can be \
                //   filled into a contiguous area.
                // Notice: this method takes into account concave shapes
                for shape_points in shape_splitter.collect() {
                    // Is that enough points to form at least two triangles?
                    if shape_points.len() >= 4 {
                        let triangles = poly2tri::triangulate_points(shape_points.iter());

                        for index in 0..triangles.size() {
                            conrod::widget::polygon::Polygon::abs_styled(
                                triangles.get_triangle(index).points.iter().copied(),
                                polygon_style,
                            )
                            .top_left_of(self.parent)
                            .set(self.graph.fill.next(&mut self.ui), &mut self.ui);
                        }
                    }
                }
            }

            Ok(())
        } else {
            Err(DrawingErrorKind::DrawingError(
                ConrodBackendError::NoParentPosition,
            ))
        }
    }

    fn draw_text<S: BackendTextStyle>(
        &mut self,
        text: &str,
        style: &S,
        pos: BackendCoord,
    ) -> Result<(), DrawingErrorKind<Self::ErrorType>> {
        // Adapt font style from rasterizer style to Conrod
        let (text_width_estimated, font_size_final) = convert::font_style(text, style.size());

        // Generate text style
        let mut text_style = conrod::widget::primitive::text::Style::default();

        text_style.color = Some(color::Color::from(&style.color()).into());
        text_style.font_id = Some(Some(self.font));
        text_style.font_size = Some(font_size_final);

        text_style.justify = Some(match style.anchor().h_pos {
            text_anchor::HPos::Left => conrod::text::Justify::Left,
            text_anchor::HPos::Right => conrod::text::Justify::Right,
            text_anchor::HPos::Center => conrod::text::Justify::Center,
        });

        // Render text widget
        conrod::widget::Text::new(text)
            .with_style(text_style)
            .top_left_with_margins_on(
                self.parent,
                pos.1 as f64 - (style.size() / 2.0 + 1.0),
                pos.0 as f64 - text_width_estimated,
            )
            .set(self.graph.text.next(&mut self.ui), &mut self.ui);

        Ok(())
    }

    fn estimate_text_size<S: BackendTextStyle>(
        &self,
        text: &str,
        style: &S,
    ) -> Result<(u32, u32), DrawingErrorKind<Self::ErrorType>> {
        let (text_width_estimated, text_height_estimated) = convert::font_style(text, style.size());

        // Return as (size_on_x, size_on_y)
        Ok((text_width_estimated as u32, text_height_estimated))
    }

    fn blit_bitmap(
        &mut self,
        _pos: BackendCoord,
        (_iw, _ih): (u32, u32),
        _src: &[u8],
    ) -> Result<(), DrawingErrorKind<Self::ErrorType>> {
        // Not supported yet (rendering ignored)
        // Notice: doing this efficiently would require building an internal buffer on 'self', and \
        //   rendering it as a Conrod image widget when the final call to 'present()' is done. \
        //   Note that this buffer would be shared with 'draw_pixel()', and thus alpha-channel \
        //   pixels would need to be blended accordingly.

        Ok(())
    }
}
