// plotters-conrod
//
// Conrod backend for Plotters
// Copyright: 2020, Valerian Saliou <valerian@valeriansaliou.name>
// License: MIT

use plotters_backend::{
    BackendColor, BackendCoord, BackendStyle, BackendTextStyle, DrawingBackend, DrawingErrorKind,
};

use conrod_core::{self as conrod, Positionable, Sizeable, Widget};

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
    points: &'b conrod::widget::id::List, // TODO: super heavy
    lines: &'b conrod::widget::id::List,  // TODO: super heavy
    line_last_idx: usize,                 // TODO: ugly
}

impl<'a, 'b> ConrodBackend<'a, 'b> {
    pub fn new(
        ui: &'a mut conrod::UiCell<'b>,
        size: (u32, u32),
        parent: conrod::widget::Id,
        points: &'b conrod::widget::id::List,
        lines: &'b conrod::widget::id::List,
    ) -> Self {
        Self {
            ui,
            parent,
            size,
            points,
            lines,
            line_last_idx: 0,
        }
    }
}

impl<'a, 'b> DrawingBackend for ConrodBackend<'a, 'b> {
    type ErrorType = DummyBackendError;

    fn get_size(&self) -> (u32, u32) {
        self.size
    }

    fn ensure_prepared(&mut self) -> Result<(), DrawingErrorKind<DummyBackendError>> {
        // TODO: use this (do something w/ it)
        Ok(())
    }

    fn present(&mut self) -> Result<(), DrawingErrorKind<DummyBackendError>> {
        // TODO: use this (do something w/ it)
        Ok(())
    }

    fn draw_pixel(
        &mut self,
        point: BackendCoord,
        color: BackendColor,
    ) -> Result<(), DrawingErrorKind<Self::ErrorType>> {
        // TODO: disable this heavy impl? (we better off)

        // TODO: fn for that
        // pos = y(p.1) x width + x(p.0)
        let id_linear_idx = (point.1 * self.size.0 as i32 + point.0) as usize;
        let (pos_x, pos_y) = (point.0 as f64, point.1 as f64);

        // TODO: use a point primitive rather?
        // TODO: figure out this id thing, or create a custom widget? looks dirty here
        conrod::widget::rectangle::Rectangle::fill_with([1.0, 1.0], make_conrod_color(&color))
            .top_left_with_margins_on(self.parent, pos_y, pos_x)
            .set(self.points[id_linear_idx], &mut self.ui);

        Ok(())
    }

    fn draw_line<S: BackendStyle>(
        &mut self,
        from: BackendCoord,
        to: BackendCoord,
        style: &S,
    ) -> Result<(), DrawingErrorKind<Self::ErrorType>> {
        let mut style = conrod::widget::primitive::line::Style::solid()
            .color(make_conrod_color(&style.color()))
            .thickness(style.stroke_width() as _);

        // TODO: this is ugly, as we cannot reference by coords due to collisions
        let id_linear_idx = self.line_last_idx;
        self.line_last_idx += 1;

        let half_width = (self.size.0 / 2) as i32;
        let full_height = self.size.1 as i32;

        // TODO: remove the absolute positioning shit
        conrod::widget::line::Line::abs_styled(
            [(from.0 - half_width) as _, -from.1 as _],
            [(to.0 - half_width) as _, -to.1 as _],
            style,
        )
        .top_left_of(self.parent)
        .set(self.lines[id_linear_idx], &mut self.ui);

        Ok(())
    }

    fn draw_rect<S: BackendStyle>(
        &mut self,
        upper_left: BackendCoord,
        bottom_right: BackendCoord,
        style: &S,
        fill: bool,
    ) -> Result<(), DrawingErrorKind<Self::ErrorType>> {
        // TODO: remove this debug
        dbg!("ConrodBackend : draw_rect");

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
        // TODO: remove this debug
        dbg!("ConrodBackend : draw_circle");

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
        // TODO: remove this debug
        dbg!("ConrodBackend : draw_text");

        // TODO

        Ok(())
    }

    // TODO
    // fn estimate_text_size<S: BackendTextStyle>(
    //     &self,
    //     text: &str,
    //     style: &S
    // ) -> Result<(u32, u32), DrawingErrorKind<Self::ErrorType>> {
    //     // TODO

    //     Ok((0, 0))
    // }
}

// TODO: implement Into or From on conrod Color trait? (cleaner)
fn make_conrod_color(color: &BackendColor) -> conrod_core::color::Color {
    let ((r, g, b), a) = (color.rgb, color.alpha);

    conrod_core::color::Color::Rgba(
        r as f32 / 255.0,
        g as f32 / 255.0,
        b as f32 / 255.0,
        a as f32,
    )
}
