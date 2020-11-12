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

struct ConrodBackendColor(conrod::color::Color);

#[derive(Debug)]
pub struct ConrodBackendError;

impl std::fmt::Display for ConrodBackendError {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(fmt, "{:?}", self)
    }
}

impl std::error::Error for ConrodBackendError {}

pub struct ConrodBackend<'a, 'b> {
    ui: &'a mut conrod::UiCell<'b>,
    size: (u32, u32),
    parent: conrod::widget::Id,
    font: conrod::text::font::Id,
    points: ConrodBackendPoints<'b>,
    indexes: ConrodBackendIndexes,
}

pub struct ConrodBackendPoints<'a> {
    line: &'a conrod::widget::id::List,   // TODO: super heavy
    rect: &'a conrod::widget::id::List,   // TODO: super heavy
    path: &'a conrod::widget::id::List,   // TODO: super heavy
    circle: &'a conrod::widget::id::List, // TODO: super heavy
    text: &'a conrod::widget::id::List,   // TODO: super heavy
}

pub struct ConrodBackendIndexes {
    line: usize,   // TODO: ugly
    rect: usize,   // TODO: ugly
    path: usize,   // TODO: ugly
    circle: usize, // TODO: ugly
    text: usize,   // TODO: ugly
}

impl<'a, 'b> ConrodBackend<'a, 'b> {
    pub fn new(
        ui: &'a mut conrod::UiCell<'b>,
        size: (u32, u32),
        parent: conrod::widget::Id,
        font: conrod::text::font::Id,
        points_line: &'b conrod::widget::id::List,
        points_rect: &'b conrod::widget::id::List,
        points_path: &'b conrod::widget::id::List,
        points_circle: &'b conrod::widget::id::List,
        points_text: &'b conrod::widget::id::List,
    ) -> Self {
        Self {
            ui,
            parent,
            font,
            size,
            points: ConrodBackendPoints {
                line: points_line,
                rect: points_rect,
                path: points_path,
                circle: points_circle,
                text: points_text,
            },
            indexes: ConrodBackendIndexes {
                line: 0,
                rect: 0,
                path: 0,
                circle: 0,
                text: 0,
            },
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
        // TODO: commonize w/ draw_path? + inline fn
        let line_style = conrod::widget::primitive::line::Style::solid()
            .color(ConrodBackendColor::from(&style.color()).into())
            .thickness(style.stroke_width() as _);

        // TODO: commonize calls to this in a method
        // TODO: this is ugly, as we cannot reference by coords due to collisions
        let index = self.indexes.line;
        self.indexes.line += 1;

        // Acquire parent bounding box in absolute coordinates (required for line points \
        //   positioning)
        if let Some(parent_rect) = self.ui.rect_of(self.parent) {
            let (box_x_start, box_y_end) = (parent_rect.x.start as i32, parent_rect.y.end as i32);

            // TODO: remove the absolute positioning thing? (find a way to do clean relative \
            //   positioning straight out of the box)
            conrod::widget::line::Line::abs_styled(
                [(from.0 + box_x_start) as _, (-from.1 + box_y_end) as _],
                [(to.0 + box_x_start) as _, (-to.1 + box_y_end) as _],
                line_style,
            )
            .top_left_of(self.parent)
            .set(self.points.line[index], &mut self.ui);
        }

        Ok(())
    }

    fn draw_rect<S: BackendStyle>(
        &mut self,
        upper_left: BackendCoord,
        bottom_right: BackendCoord,
        style: &S,
        fill: bool,
    ) -> Result<(), DrawingErrorKind<Self::ErrorType>> {
        // TODO: commonize calls to this in a method
        // TODO: this is ugly, as we cannot reference by coords due to collisions
        let index = self.indexes.rect;
        self.indexes.rect += 1;

        let rectangle_style = if fill == true {
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

        conrod::widget::rectangle::Rectangle::styled(
            [
                (bottom_right.0 - upper_left.0) as _,
                (bottom_right.1 - upper_left.1) as _,
            ],
            rectangle_style,
        )
        .top_left_with_margins_on(self.parent, upper_left.1 as _, upper_left.0 as _)
        .set(self.points.rect[index], &mut self.ui);

        Ok(())
    }

    // TODO: implement this, this replaces draw_line and is more efficient!
    fn draw_path<S: BackendStyle, I: IntoIterator<Item = BackendCoord>>(
        &mut self,
        path: I,
        style: &S,
    ) -> Result<(), DrawingErrorKind<Self::ErrorType>> {
        // TODO: commonize w/ draw_line? + inline fn
        let line_style = conrod::widget::primitive::line::Style::solid()
            .color(ConrodBackendColor::from(&style.color()).into())
            .thickness(style.stroke_width() as _);

        // TODO: commonize calls to this in a method
        // TODO: this is ugly, as we cannot reference by coords due to collisions
        let index = self.indexes.path;
        self.indexes.path += 1;

        // Acquire parent bounding box in absolute coordinates (required for line points \
        //   positioning)
        // TODO: commonize this w/ the draw_line fn + always inline?
        if let Some(parent_rect) = self.ui.rect_of(self.parent) {
            let (box_x_start, box_y_end) = (parent_rect.x.start as i32, parent_rect.y.end as i32);

            // TODO: remove the absolute positioning thing? (find a way to do clean relative \
            //   positioning straight out of the box)
            // TODO: can we make this iterator thing zero-alloc? looks like we cannot as conrod \
            //   excepts an IntoIterator<Point> to be passed in, and not an Iterator<Point>, sadly.
            conrod::widget::point_path::PointPath::abs_styled(
                path.into_iter()
                    .map(|point| [(point.0 + box_x_start) as _, (-point.1 + box_y_end) as _])
                    .collect::<Vec<conrod::position::Point>>(),
                line_style,
            )
            .top_left_of(self.parent)
            .set(self.points.path[index], &mut self.ui);
        }

        Ok(())
    }

    fn draw_circle<S: BackendStyle>(
        &mut self,
        center: BackendCoord,
        radius: u32,
        style: &S,
        fill: bool,
    ) -> Result<(), DrawingErrorKind<Self::ErrorType>> {
        // TODO: this is ugly, as we cannot reference by coords due to collisions
        let index = self.indexes.circle;
        self.indexes.circle += 1;

        let circle_style = if fill == true {
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

        conrod::widget::circle::Circle::styled(radius as f64, circle_style)
            .top_left_with_margins_on(
                self.parent,
                (center.1 - radius as i32) as f64,
                (center.0 - radius as i32) as f64,
            )
            .set(self.points.circle[index], &mut self.ui);

        Ok(())
    }

    fn fill_polygon<S: BackendStyle, I: IntoIterator<Item = BackendCoord>>(
        &mut self,
        _vert: I,
        _style: &S,
    ) -> Result<(), DrawingErrorKind<Self::ErrorType>> {
        // Not supported yet (rendering ignored)

        Ok(())
    }

    fn draw_text<S: BackendTextStyle>(
        &mut self,
        text: &str,
        style: &S,
        pos: BackendCoord,
    ) -> Result<(), DrawingErrorKind<Self::ErrorType>> {
        // TODO: commonize calls to this in a method
        // TODO: this is ugly, as we cannot reference by coords due to collisions
        let index = self.indexes.text;
        self.indexes.text += 1;

        let (text_width_estimated, font_size_final) = convert_font_style(text, style.size());

        let mut text_style = conrod::widget::primitive::text::Style::default();

        text_style.color = Some(ConrodBackendColor::from(&style.color()).into());
        text_style.font_id = Some(Some(self.font));
        text_style.font_size = Some(font_size_final);

        text_style.justify = Some(match style.anchor().h_pos {
            text_anchor::HPos::Left => conrod::text::Justify::Left,
            text_anchor::HPos::Right => conrod::text::Justify::Right,
            text_anchor::HPos::Center => conrod::text::Justify::Center,
        });

        // TODO: support transform and style
        conrod::widget::Text::new(text)
            .with_style(text_style)
            .top_left_with_margins_on(
                self.parent,
                pos.1 as f64 - (style.size() / 2.0),
                pos.0 as f64 - text_width_estimated,
            )
            .set(self.points.text[index], &mut self.ui);

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

impl From<&BackendColor> for ConrodBackendColor {
    #[inline(always)]
    fn from(item: &BackendColor) -> Self {
        let ((r, g, b), a) = (item.rgb, item.alpha);

        // Notice: correct alpha channel value, looks like there is a sqrt ratio between plotters and \
        //   conrod color objects.
        // TODO: this does not work fine for all alpha levels, is that color HSL or something else?
        Self(conrod::color::Color::Rgba(
            r as f32 / 255.0,
            g as f32 / 255.0,
            b as f32 / 255.0,
            (a * a) as f32,
        ))
    }
}

impl Into<conrod::color::Color> for ConrodBackendColor {
    #[inline(always)]
    fn into(self) -> conrod::color::Color {
        self.0
    }
}

fn convert_font_style(text: &str, size: f64) -> (f64, u32) {
    // Font size needs to be adjusted using a 90% factor, as to appear the same size than \
    //   when redered using the reference Bitmap backend.
    let font_size_final = (size * 0.9) as u32;

    // TODO: this is also ugly, should not have to do that
    let text_width_estimated = (text.len() as f64 * size) * 0.6;

    (text_width_estimated, font_size_final)
}
