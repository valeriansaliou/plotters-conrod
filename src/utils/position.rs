// plotters-conrod
//
// Conrod backend for Plotters
// Copyright: 2020, Valerian Saliou <valerian@valeriansaliou.name>
// License: MIT

use conrod_core::{self as conrod, position::Scalar as ConrodScalar};
use plotters_backend::BackendCoord;

use super::path::PathScalar;

type PositionScalar = f64;

pub(crate) struct PositionParent {
    x_start: PositionScalar,
    y_end: PositionScalar,
}

impl PositionParent {
    #[inline(always)]
    pub(crate) fn from(ui: &conrod::UiCell, parent: conrod::widget::Id) -> Option<Self> {
        if let Some(parent_rect) = ui.rect_of(parent) {
            Some(Self {
                x_start: parent_rect.x.start,
                y_end: parent_rect.y.end,
            })
        } else {
            None
        }
    }

    #[inline(always)]
    pub(crate) fn abs_point_conrod_scalar(&self, point: &BackendCoord) -> [ConrodScalar; 2] {
        // Convert relative-positioned point (in backend coordinates) to absolute coordinates in \
        //   the full rendering space.
        [
            point.0 as PositionScalar + self.x_start,
            -point.1 as PositionScalar + self.y_end,
        ]
    }

    #[inline(always)]
    pub(crate) fn abs_point_path_simplifier(&self, point: &BackendCoord) -> [PathScalar; 2] {
        // Convert relative-positioned point (in backend coordinates) to absolute coordinates in \
        //   the full rendering space.
        [
            point.0 as PositionScalar + self.x_start,
            -point.1 as PositionScalar + self.y_end,
        ]
    }
}
