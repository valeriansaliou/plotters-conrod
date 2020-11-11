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
    points: &'b conrod::widget::id::List, // TODO: super heavy
    point_index: usize,                   // TODO: ugly
}

impl<'a, 'b> ConrodBackend<'a, 'b> {
    pub fn new(
        ui: &'a mut conrod::UiCell<'b>,
        size: (u32, u32),
        parent: conrod::widget::Id,
        font: conrod_core::text::font::Id,
        points: &'b conrod::widget::id::List,
    ) -> Self {
        Self {
            ui,
            parent,
            font,
            size,
            points,
            point_index: 0,
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
        let index = self.point_index;
        self.point_index += 1;

        let half_width = (self.size.0 / 2) as i32;

        // TODO: remove the absolute positioning thing
        conrod::widget::line::Line::abs_styled(
            [(from.0 - half_width) as _, -from.1 as _],
            [(to.0 - half_width) as _, -to.1 as _],
            line_style,
        )
        .top_left_of(self.parent)
        .set(self.points[index], &mut self.ui);

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
        _center: BackendCoord,
        _radius: u32,
        _style: &S,
        _fill: bool,
    ) -> Result<(), DrawingErrorKind<Self::ErrorType>> {
        // TODO
        // let rect = circle(center.0 as f64, center.1 as f64, radius as f64);
        // if fill {
        //     ellipse(
        //         make_piston_rgba(&style.color()),
        //         rect,
        //         self.context.transform,
        //         self.graphics,
        //     );
        // } else {
        //     circle_arc(
        //         make_piston_rgba(&style.color()),
        //         self.scale,
        //         std::f64::consts::PI,
        //         0.0,
        //         rect,
        //         self.context.transform,
        //         self.graphics,
        //     );
        //     circle_arc(
        //         make_piston_rgba(&style.color()),
        //         self.scale,
        //         0.0,
        //         std::f64::consts::PI,
        //         rect,
        //         self.context.transform,
        //         self.graphics,
        //     );
        // }

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
        let index = self.point_index;
        self.point_index += 1;

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
            .set(self.points[index], &mut self.ui);

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
