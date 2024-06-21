// plotters-conrod
//
// Conrod backend for Plotters
// Copyright: 2020, Valerian Saliou <valerian@valeriansaliou.name>
// License: MIT

use conrod_core::{self as conrod, position::Scalar as ConrodScalar};
use plotters_backend::BackendCoord;

use super::path::PathScalar;

type PositionScalar = i32;

pub(crate) struct PositionParent {
    x_start: PositionScalar,
    y_end: PositionScalar,
}

impl PositionParent {
    #[inline(always)]
    pub(crate) fn from(ui: &conrod::UiCell, parent: conrod::widget::Id) -> Option<Self> {
        ui.rect_of(parent).map(|parent_rect| Self {
            x_start: parent_rect.x.start as PositionScalar,
            y_end: parent_rect.y.end as PositionScalar,
        })
    }

    #[inline(always)]
    pub(crate) fn abs_point_conrod_scalar(&self, point: &BackendCoord) -> [ConrodScalar; 2] {
        // Convert relative-positioned point (in backend coordinates) to absolute coordinates in \
        //   the full rendering space.
        [
            (point.0 + self.x_start) as ConrodScalar,
            (-point.1 + self.y_end) as ConrodScalar,
        ]
    }

    #[inline(always)]
    pub(crate) fn abs_point_path_simplifier(&self, point: &BackendCoord) -> [PathScalar; 2] {
        // Convert relative-positioned point (in backend coordinates) to absolute coordinates in \
        //   the full rendering space.
        [
            (point.0 + self.x_start) as PathScalar,
            (-point.1 + self.y_end) as PathScalar,
        ]
    }
}
