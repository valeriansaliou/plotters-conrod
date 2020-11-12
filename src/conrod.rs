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

const BACKEND_GRAPH_RESIZE_CHUNK: usize = 100;

struct ConrodBackendPosition {
    x_start: i32,
    y_end: i32,
}

struct ConrodBackendColor(conrod::color::Color);

#[derive(Debug)]
/// Indicates that some error occured within the Conrod backend
pub enum ConrodBackendError {
    /// The parent widget position could not be acquired, is the parent widget drawn in Conrod?
    NoParentPosition,
}

/// The Conrod drawing backend
pub struct ConrodBackend<'a, 'b> {
    ui: &'a mut conrod::UiCell<'b>,
    size: (u32, u32),
    parent: conrod::widget::Id,
    font: conrod::text::font::Id,
    graph: &'a mut ConrodBackendReusableGraph,
}

/// The re-usable graph of Conrod widget IDs, to be re-used for each plot draw (building it is expensive, re-using it is cheap; so build it once and re-use it across loop calls)
pub struct ConrodBackendReusableGraph {
    line: ConrodBackendReusableGraphAtom,
    rect: ConrodBackendReusableGraphAtom,
    path: ConrodBackendReusableGraphAtom,
    circle: ConrodBackendReusableGraphAtom,
    text: ConrodBackendReusableGraphAtom,
    fill: ConrodBackendReusableGraphAtom,
}

struct ConrodBackendReusableGraphAtom(conrod::widget::id::List, usize);

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
        if let Some(position) = ConrodBackendPosition::from(&self.ui, self.parent) {
            // Generate line style
            let line_style = conrod::widget::primitive::line::Style::solid()
                .color(ConrodBackendColor::from(&style.color()).into())
                .thickness(style.stroke_width() as _);

            // Render line widget
            conrod::widget::line::Line::abs_styled(
                position.abs_point(&from),
                position.abs_point(&to),
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
                ConrodBackendColor::from(&style.color()).into(),
            )
        } else {
            conrod::widget::primitive::shape::Style::outline_styled(
                conrod::widget::primitive::line::Style::new()
                    .color(ConrodBackendColor::from(&style.color()).into())
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
        if let Some(position) = ConrodBackendPosition::from(&self.ui, self.parent) {
            // Generate line style
            let line_style = conrod::widget::primitive::line::Style::solid()
                .color(ConrodBackendColor::from(&style.color()).into())
                .thickness(style.stroke_width() as _);

            // Render point path widget
            conrod::widget::point_path::PointPath::abs_styled(
                path.into_iter()
                    .map(|point| position.abs_point(&point))
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
                ConrodBackendColor::from(&style.color()).into(),
            )
        } else {
            conrod::widget::primitive::shape::Style::outline_styled(
                conrod::widget::primitive::line::Style::new()
                    .color(ConrodBackendColor::from(&style.color()).into())
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
        if let Some(position) = ConrodBackendPosition::from(&self.ui, self.parent) {
            // Generate polygon style
            let polygon_style = conrod::widget::primitive::shape::Style::fill_with(
                ConrodBackendColor::from(&style.color()).into(),
            );

            // Render polygon widget
            conrod::widget::polygon::Polygon::abs_styled(
                vert.into_iter()
                    .map(|vertex| position.abs_point(&vertex))
                    .collect::<Vec<conrod::position::Point>>(),
                polygon_style,
            )
            .top_left_of(self.parent)
            .set(self.graph.fill.next(&mut self.ui), &mut self.ui);

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
        let (text_width_estimated, font_size_final) = convert_font_style(text, style.size());

        // Generate text style
        let mut text_style = conrod::widget::primitive::text::Style::default();

        text_style.color = Some(ConrodBackendColor::from(&style.color()).into());
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
        let (text_width_estimated, text_height_estimated) = convert_font_style(text, style.size());

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

impl ConrodBackendPosition {
    fn from(ui: &conrod::UiCell, parent: conrod::widget::Id) -> Option<Self> {
        if let Some(parent_rect) = ui.rect_of(parent) {
            Some(Self {
                x_start: parent_rect.x.start as _,
                y_end: parent_rect.y.end as _,
            })
        } else {
            None
        }
    }

    fn abs_point(&self, point: &BackendCoord) -> [f64; 2] {
        // Convert relative-positioned point (in backend coordinates) to absolute coordinates in \
        //   the full rendering space.
        [(point.0 + self.x_start) as _, (-point.1 + self.y_end) as _]
    }
}

impl From<&BackendColor> for ConrodBackendColor {
    #[inline(always)]
    fn from(item: &BackendColor) -> Self {
        let ((r, g, b), a) = (item.rgb, item.alpha);

        // Warning: 'Rgba' is actually 'Srgba', this naming in Conrod is misleading, hence why \
        //   we apply a transform on its alpha channel as to correct it. Looking at Conrod \
        //   source code, it was found out that the gamma channel is not encoded in the correct \
        //   space because 'linear gamma yields better results when doing math with colors'. \
        //   Though, this means that the alpha value passed would render to a brighter color and \
        //   would not blend enough with its 'Z-1' back-layer. Converting the alpha channel to \
        //   linear fixes this color rendering issue.
        Self(conrod::color::Color::Rgba(
            r as f32 / 255.0,
            g as f32 / 255.0,
            b as f32 / 255.0,
            Self::gamma_srgb_to_linear(a as f32),
        ))
    }
}

impl Into<conrod::color::Color> for ConrodBackendColor {
    #[inline(always)]
    fn into(self) -> conrod::color::Color {
        self.0
    }
}

impl ConrodBackendColor {
    #[inline(always)]
    fn gamma_srgb_to_linear(f: f32) -> f32 {
        // See: https://en.wikipedia.org/wiki/SRGB
        // Code from: https://docs.rs/conrod_glium/0.71.0/src/conrod_glium/lib.rs.html#310-320
        if f <= 0.04045 {
            f / 12.92
        } else {
            ((f + 0.055) / 1.055).powf(2.4)
        }
    }
}

impl ConrodBackendReusableGraph {
    /// Build a new Conrod backend re-usable graph of widget identifiers
    ///
    /// **This should be put outside of your drawer loop and built once; failing to do so will result in heavy CPU usage due to the graph being rebuilt for every frame!**
    pub fn build() -> Self {
        Self {
            line: ConrodBackendReusableGraphAtom::new(),
            rect: ConrodBackendReusableGraphAtom::new(),
            path: ConrodBackendReusableGraphAtom::new(),
            circle: ConrodBackendReusableGraphAtom::new(),
            text: ConrodBackendReusableGraphAtom::new(),
            fill: ConrodBackendReusableGraphAtom::new(),
        }
    }

    fn prepare(&mut self) {
        // Notice: destructuring is used there as a safety measure, so that no field is \
        //   forgotten, which could be dangerous (ie. risk of memory leak).
        let Self {
            line,
            rect,
            path,
            circle,
            text,
            fill,
        } = self;

        // Proceed all resets
        line.reset();
        rect.reset();
        path.reset();
        circle.reset();
        text.reset();
        fill.reset();
    }
}

impl ConrodBackendReusableGraphAtom {
    fn new() -> Self {
        Self(conrod::widget::id::List::new(), 0)
    }

    fn next(&mut self, ui: &mut conrod::UiCell) -> conrod::widget::Id {
        // Acquire current index (ie. last 'next index')
        let current_index = self.1;

        // IDs list has not a large-enough capacity for all dynamically-allocated IDs? Enlarge it \
        //   by a pre-defined chunk size (this prevents enlarging the list one by one, requiring \
        //   frequent re-allocations)
        // Notice: this upsizes the graph list to allow current ID to be stored. This is always \
        //   called on the very first call, and may be periodically called whenever the last \
        //   chunked upsize was not enough to store all IDs. This is a trade-off between memory \
        //   and performances.
        if current_index >= self.0.len() {
            self.0.resize(
                self.0.len() + BACKEND_GRAPH_RESIZE_CHUNK,
                &mut ui.widget_id_generator(),
            );
        }

        // Mutate state for next index
        self.1 += 1;

        self.0[current_index]
    }

    fn reset(&mut self) {
        // Rollback the incremented IDs counter back to zero
        self.1 = 0;
    }
}

impl std::fmt::Display for ConrodBackendError {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(fmt, "{:?}", self)
    }
}

impl std::error::Error for ConrodBackendError {}

#[inline(always)]
fn convert_font_style(text: &str, size: f64) -> (f64, u32) {
    // Font size needs to be adjusted using a 90% factor, as to appear the same size than \
    //   when redered using the reference Bitmap backend.
    // Format: (text_width_estimated, font_size_final)
    ((text.len() as f64 * size) * 0.6, (size * 0.9) as u32)
}
