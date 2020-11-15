// plotters-conrod
//
// Conrod backend for Plotters
// Copyright: 2020, Valerian Saliou <valerian@valeriansaliou.name>
// License: MIT

use conrod_core as conrod;
use plotters_backend::BackendCoord;

pub(crate) struct PositionParent {
    x_start: i32,
    y_end: i32,
}

impl PositionParent {
    #[inline(always)]
    pub(crate) fn from(ui: &conrod::UiCell, parent: conrod::widget::Id) -> Option<Self> {
        if let Some(parent_rect) = ui.rect_of(parent) {
            Some(Self {
                x_start: parent_rect.x.start as _,
                y_end: parent_rect.y.end as _,
            })
        } else {
            None
        }
    }

    #[inline(always)]
    pub(crate) fn abs_point_f64(&self, point: &BackendCoord) -> [f64; 2] {
        // Convert relative-positioned point (in backend coordinates) to absolute coordinates in \
        //   the full rendering space.
        [
            (point.0 + self.x_start) as f64,
            (-point.1 + self.y_end) as f64,
        ]
    }

    #[inline(always)]
    pub(crate) fn abs_point_i32(&self, point: &BackendCoord) -> [i32; 2] {
        // Convert relative-positioned point (in backend coordinates) to absolute coordinates in \
        //   the full rendering space.
        [
            (point.0 + self.x_start) as i32,
            (-point.1 + self.y_end) as i32,
        ]
    }
}
