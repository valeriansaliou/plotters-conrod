// plotters-conrod
//
// Conrod backend for Plotters
// Copyright: 2020, Valerian Saliou <valerian@valeriansaliou.name>
// License: MIT

use conrod_core::{position::Scalar as ConrodScalar, FontSize as ConrodFontSize};

#[inline(always)]
pub(crate) fn font_style(text: &str, size: ConrodScalar) -> (ConrodScalar, ConrodFontSize) {
    // Font size needs to be adjusted using a 90% factor, as to appear the same size than \
    //   when redered using the reference Bitmap backend.
    // Format: (text_width_estimated, font_size_final)
    (
        (text.len() as ConrodScalar * size) * 0.6,
        (size * 0.9) as ConrodFontSize,
    )
}
