// plotters-conrod
//
// Conrod backend for Plotters
// Copyright: 2020, Valerian Saliou <valerian@valeriansaliou.name>
// License: MIT

use plotters_backend::{
    BackendColor, BackendCoord, BackendStyle, DrawingBackend, DrawingErrorKind, BackendTextStyle,
};

#[derive(Debug)]
pub struct DummyBackendError;

impl std::fmt::Display for DummyBackendError {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(fmt, "{:?}", self)
    }
}

impl std::error::Error for DummyBackendError {}

pub struct ConrodBackend {
    size: (u32, u32)
}

impl ConrodBackend {
    pub fn new(size: (u32, u32)) -> Self {
        Self {
            size
        }
    }
}

impl DrawingBackend for ConrodBackend {
    type ErrorType = DummyBackendError;

    fn get_size(&self) -> (u32, u32) {
        self.size
    }

    fn ensure_prepared(&mut self) -> Result<(), DrawingErrorKind<DummyBackendError>> {
        // TODO: optimize
        Ok(())
    }

    fn present(&mut self) -> Result<(), DrawingErrorKind<DummyBackendError>> {
        // TODO: optimize
        Ok(())
    }

    fn draw_pixel(
        &mut self,
        point: BackendCoord,
        color: BackendColor,
    ) -> Result<(), DrawingErrorKind<Self::ErrorType>> {
        // TODO
        // piston_window::rectangle(
        //     make_piston_rgba(&color),
        //     make_point_pair(point, (1, 1), self.scale),
        //     self.context.transform,
        //     self.graphics,
        // );

        Ok(())
    }

    fn draw_line<S: BackendStyle>(
        &mut self,
        from: BackendCoord,
        to: BackendCoord,
        style: &S,
    ) -> Result<(), DrawingErrorKind<Self::ErrorType>> {
        // TODO
        // line(
        //     make_piston_rgba(&style.color()),
        //     self.scale,
        //     make_point_pair(from, to, self.scale),
        //     self.context.transform,
        //     self.graphics,
        // );

        Ok(())
    }

    fn draw_rect<S: BackendStyle>(
        &mut self,
        upper_left: BackendCoord,
        bottom_right: BackendCoord,
        style: &S,
        fill: bool,
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

    // TODO
    // fn draw_text<S: BackendTextStyle>(
    //     &mut self,
    //     text: &str,
    //     style: &S,
    //     pos: BackendCoord
    // ) -> Result<(), DrawingErrorKind<Self::ErrorType>> {
    //     // TODO

    //     Ok(())
    // }

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
