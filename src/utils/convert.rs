// plotters-conrod
//
// Conrod backend for Plotters
// Copyright: 2020, Valerian Saliou <valerian@valeriansaliou.name>
// License: MIT

#[inline(always)]
pub fn font_style(text: &str, size: f64) -> (f64, u32) {
    // Font size needs to be adjusted using a 90% factor, as to appear the same size than \
    //   when redered using the reference Bitmap backend.
    // Format: (text_width_estimated, font_size_final)
    ((text.len() as f64 * size) * 0.6, (size * 0.9) as u32)
}
