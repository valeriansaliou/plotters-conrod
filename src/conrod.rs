// plotters-conrod
//
// Conrod backend for Plotters
// Copyright: 2020, Valerian Saliou <valerian@valeriansaliou.name>
// License: MIT

use plotters_backend::{
    text_anchor, BackendColor, BackendCoord, BackendStyle, BackendTextStyle, DrawingBackend,
    DrawingErrorKind,
};

use conrod_core::{self as conrod, Positionable, Widget};

#[derive(Debug)]
pub struct DummyBackendError;

impl std::fmt::Display for DummyBackendError {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(fmt, "{:?}", self)
    }
}

impl std::error::Error for DummyBackendError {}

pub struct ConrodBackend<'a, 'b> {
    ui: &'a mut conrod::UiCell<'b>,
    size: (u32, u32),
    parent: conrod::widget::Id,
    font: conrod_core::text::font::Id,
    points: ConrodBackendPoints<'b>,
    indexes: ConrodBackendIndexes,
}

pub struct ConrodBackendPoints<'a> {
    line: &'a conrod::widget::id::List,   // TODO: super heavy
    circle: &'a conrod::widget::id::List, // TODO: super heavy
    text: &'a conrod::widget::id::List,   // TODO: super heavy
}

pub struct ConrodBackendIndexes {
    line: usize,   // TODO: ugly
    circle: usize, // TODO: ugly
    text: usize,   // TODO: ugly
}

impl<'a, 'b> ConrodBackend<'a, 'b> {
    pub fn new(
        ui: &'a mut conrod::UiCell<'b>,
        size: (u32, u32),
        parent: conrod::widget::Id,
        font: conrod_core::text::font::Id,
        points_line: &'b conrod::widget::id::List,
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
                circle: points_circle,
                text: points_text,
            },
            indexes: ConrodBackendIndexes {
                line: 0,
                circle: 0,
                text: 0,
            },
        }
    }
}

impl<'a, 'b> DrawingBackend for ConrodBackend<'a, 'b> {
    type ErrorType = DummyBackendError;

    fn get_size(&self) -> (u32, u32) {
        self.size
    }

    fn ensure_prepared(&mut self) -> Result<(), DrawingErrorKind<DummyBackendError>> {
        Ok(())
    }

    fn present(&mut self) -> Result<(), DrawingErrorKind<DummyBackendError>> {
        Ok(())
    }

    fn draw_pixel(
        &mut self,
        _point: BackendCoord,
        _color: BackendColor,
    ) -> Result<(), DrawingErrorKind<Self::ErrorType>> {
        // TODO: disabled for now, as its heavy as we seemingly do not need it
        // TODO: maybe make it configurable as to enable it as needed? Or use lighter drawing \
        //   primitives, or else disable it in a static way

        // TODO: fn for that
        // pos = y(p.1) x width + x(p.0)
        // let id_linear_idx = (point.1 * self.size.0 as i32 + point.0) as usize;
        // let (pos_x, pos_y) = (point.0 as f64, point.1 as f64);

        // // TODO: use a point primitive rather?
        // // TODO: figure out this id thing, or create a custom widget? looks dirty here
        // conrod::widget::rectangle::Rectangle::fill_with([1.0, 1.0], make_conrod_color(&color))
        //     .top_left_with_margins_on(self.parent, pos_y, pos_x)
        //     .set(self.points[id_linear_idx], &mut self.ui);

        Ok(())
    }

    fn draw_line<S: BackendStyle>(
        &mut self,
        from: BackendCoord,
        to: BackendCoord,
        style: &S,
    ) -> Result<(), DrawingErrorKind<Self::ErrorType>> {
        let line_style = conrod::widget::primitive::line::Style::solid()
            .color(make_conrod_color(&style.color()))
            .thickness(style.stroke_width() as _);

        // TODO: commonize calls to this in a method
        // TODO: this is ugly, as we cannot reference by coords due to collisions
        let index = self.indexes.line;
        self.indexes.line += 1;

        // TODO: this may not be super optimized, but we need to change absolute point positions \
        //   to relative positions inside the parent.
        // TODO: 70% CPU overhead, nuke this and replace this.
        if let Some(parent_rect) = self.ui.rect_of(self.parent) {
            let box_x_start = parent_rect.x.start as i32;
            let box_y_end = parent_rect.y.end as i32;

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
        _upper_left: BackendCoord,
        _bottom_right: BackendCoord,
        _style: &S,
        _fill: bool,
    ) -> Result<(), DrawingErrorKind<Self::ErrorType>> {
        // TODO
        // if fill {
        //     rectangle(
        //         make_piston_rgba(&style.color()),
        //         make_point_pair(
        //             upper_left,
        //             (bottom_right.0 - upper_left.0, bottom_right.1 - upper_left.1),
        //             self.scale,
        //         ),
        //         self.context.transform,
        //         self.graphics,
        //     );
        // } else {
        //     let color = make_piston_rgba(&style.color());
        //     let [x0, y0, x1, y1] = make_point_pair(upper_left, bottom_right, self.scale);
        //     line(
        //         color,
        //         self.scale,
        //         [x0, y0, x0, y1],
        //         self.context.transform,
        //         self.graphics,
        //     );
        //     line(
        //         color,
        //         self.scale,
        //         [x0, y1, x1, y1],
        //         self.context.transform,
        //         self.graphics,
        //     );
        //     line(
        //         color,
        //         self.scale,
        //         [x1, y1, x1, y0],
        //         self.context.transform,
        //         self.graphics,
        //     );
        //     line(
        //         color,
        //         self.scale,
        //         [x1, y0, x0, y0],
        //         self.context.transform,
        //         self.graphics,
        //     );
        // }

        Ok(())
    }

    // TODO
    // fn draw_path<S: BackendStyle, I: Vec<BackendCoord>>(
    //     &mut self,
    //     path: I,
    //     style: &S
    // ) -> Result<(), DrawingErrorKind<Self::ErrorType>> {
    //     // TODO

    //     Ok(())
    // }

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
            conrod::widget::primitive::shape::Style::fill_with(make_conrod_color(&style.color()))
        } else {
            conrod::widget::primitive::shape::Style::outline_styled(
                conrod::widget::primitive::line::Style::new()
                    .color(make_conrod_color(&style.color()))
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

        // TODO: this is also ugly, should not have to do that
        let text_width_estimated = (text.len() as f64 * style.size()) * 0.6;

        // Font size needs to be adjusted using a 90% factor, as to appear the same size than \
        //   when redered using the reference Bitmap backend.
        let font_size_final = (style.size() * 0.9) as u32;

        let mut text_style = conrod_core::widget::primitive::text::Style::default();

        text_style.color = Some(make_conrod_color(&style.color()));
        text_style.font_id = Some(Some(self.font));
        text_style.font_size = Some(font_size_final);

        text_style.justify = Some(match style.anchor().h_pos {
            text_anchor::HPos::Left => conrod_core::text::Justify::Left,
            text_anchor::HPos::Right => conrod_core::text::Justify::Right,
            text_anchor::HPos::Center => conrod_core::text::Justify::Center,
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
        _text: &str,
        _style: &S,
    ) -> Result<(u32, u32), DrawingErrorKind<Self::ErrorType>> {
        // TODO: implement this, avoid using the font rasterizer

        Ok((0, 0))
    }
}

// TODO: implement Into or From on conrod Color trait? (cleaner)
fn make_conrod_color(color: &BackendColor) -> conrod_core::color::Color {
    let ((r, g, b), a) = (color.rgb, color.alpha);

    // Notice: correct alpha channel value, looks like there is a sqrt ratio between plotters and \
    //   conrod color objects.
    // TODO: this does not work fine for all alpha levels, is that color HSL or something else?
    conrod_core::color::Color::Rgba(
        r as f32 / 255.0,
        g as f32 / 255.0,
        b as f32 / 255.0,
        (a * a) as f32,
    )
}
