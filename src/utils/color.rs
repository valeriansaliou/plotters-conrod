// plotters-conrod
//
// Conrod backend for Plotters
// Copyright: 2020, Valerian Saliou <valerian@valeriansaliou.name>
// License: MIT

use conrod_core as conrod;
use plotters_backend::BackendColor;

pub(crate) struct Color(conrod::color::Color);

impl From<&BackendColor> for Color {
    #[inline(always)]
    fn from(item: &BackendColor) -> Self {
        let ((r, g, b), a) = (item.rgb, item.alpha);

        // Warning: 'Rgba' is actually 'Srgba', this naming in Conrod is misleading, hence why \
        //   we apply a transform on its alpha channel as to correct it. Looking at Conrod \
        //   source code, it was found out that the gamma channel is not encoded in the correct \
        //   space because 'linear gamma yields better results when doing math with colors'. \
        //   Though, this means that the alpha value passed would render to a brighter color and \
        //   would not blend enough with its 'Z-1' back-layer. Converting the alpha channel to \
        //   linear fixes this color rendering issue.
        Self(conrod::color::Color::Rgba(
            r as f32 / 255.0,
            g as f32 / 255.0,
            b as f32 / 255.0,
            Self::gamma_srgb_to_linear(a as f32),
        ))
    }
}

impl From<Color> for conrod::color::Color {
    #[inline(always)]
    fn from(c: Color) -> Self {
        c.0
    }
}

impl Color {
    #[inline(always)]
    fn gamma_srgb_to_linear(f: f32) -> f32 {
        // See: https://en.wikipedia.org/wiki/SRGB
        // Code from: https://docs.rs/conrod_glium/0.71.0/src/conrod_glium/lib.rs.html#310-320
        if f <= 0.04045 {
            f / 12.92
        } else {
            ((f + 0.055) / 1.055).powf(2.4)
        }
    }
}
