# plotters-conrod

[![Test and Build](https://github.com/valeriansaliou/plotters-conrod/workflows/Test%20and%20Build/badge.svg?branch=master)](https://github.com/valeriansaliou/plotters-conrod/actions?query=workflow%3A%22Test+and+Build%22) [![Buy Me A Coffee](https://img.shields.io/badge/buy%20me%20a%20coffee-donate-yellow.svg)](https://www.buymeacoffee.com/valeriansaliou)

This is an implementation of a Conrod backend for Plotters. This is more efficient than using the default Bitmap backend when plotting in Conrod, as it has been observed that Conrod was quite inefficient at re-rendering images at high FPS (eg. for real-time plotting).

This backend has been optimized as for speed, and as to render plots that look very similar to the default Bitmap backend, if not indistinguishable. Note that some specific plotting features supported in the Bitmap backend may not be implemented there, though.

* [Documentation](https://docs.rs/crate/plotters-conrod)
* [Crate](https://crates.io/crates/plotters-conrod)

**🇫🇷 Crafted in Nantes, France.**

## Who uses it?

<table>
<tr>
<td align="center"><a href="https://makair.life/"><img src="https://valeriansaliou.github.io/plotters-conrod/images/makair.png" width="64" /></a></td>
</tr>
<tr>
<td align="center">MakAir</td>
</tr>
</table>

* _👋 You use `plotters-conrod` and you want to be listed there? [Contact me](https://valeriansaliou.name/)._
* _ℹ️ The [MakAir](https://makair.life/) open-source medical ventilator uses `plotters-conrod` on its Rust-based [MakAir Control UI](https://github.com/makers-for-life/makair-control-ui/)._

## What is Plotters?

Plotters is an extensible Rust drawing library that can be used to plot data on nice-looking graphs, rendering them through a plotting backend (eg. to a Bitmap image raw buffer, to your GUI backend, to an SVG file, etc.).

**For more details on Plotters, please check the following links:**

- For an introduction of Plotters, see: [Plotters on Crates.io](https://crates.io/crates/plotters);
- Check the main repository on [GitHub](https://github.com/38/plotters);
- You can also visit the Plotters [homepage](https://plotters-rs.github.io/);

## How to install?

Include `plotters-conrod` in your `Cargo.toml` dependencies:

```toml
[dependencies]
plotters-conrod = "0.3"
```

_The `plotters-conrod` version used should match your `plotters` version. If there is no such `plotters-conrod` version yet, using an older `plotters-conrod` version than your `plotters` should usually work._

## How to use?

First, import `ConrodBackend` and `ConrodBackendReusableGraph`:

```rust
use plotters_conrod::{ConrodBackend, ConrodBackendReusableGraph};
```

Then, build the re-usable graph instance (outside of your drawing loop):

```rust
let mut conrod_graph = ConrodBackendReusableGraph::build();
```

**⚠️ This should be put outside of your loop and called once; failing to do so will result in heavy CPU usage due to the graph being rebuilt for every frame!**

Finally, for each frame you draw (ie. your main loop), call:

```rust
// Where:
//  - 'ui' is the UiCell that was derived from Ui for this frame;
//  - '(plot_width, plot_height)' is the size of your plot in pixels (make sure it matches its parent canvas size);
//  - 'ids.parent' is the widget::Id of the canvas that contains your plot (of the same size than the plot itself);
//  - 'fonts.regular' is the font::Id of the font to use to draw text (ie. a Conrod font identifier);
//  - 'conrod_graph' is a mutable reference to the graph instance you built outside of the drawing loop (pass it as a mutable reference);
let drawing = ConrodBackend::new(
    ui,
    (plot_width, plot_height),
    ids.parent,
    fonts.regular,
    &mut conrod_graph,
).into_drawing_area();

//-
// Build your chart as usual here, using the regular Plotters syntax
//-
```

_If you are looking for a full example of an implementation, please check [cpu-monitor.rs](./examples/cpu-monitor.rs)._

## How to run the examples?

### Example #1: `cpu-monitor`

This example samples your CPU load every second, and renders it in a real-time chart:

```sh
cargo run --release --example cpu-monitor
```

_The first plot uses `plotters-conrod`, while the second plot uses the default Bitmap backend as a reference. This can be used to compare the output and performance of both plotting backends. The Bitmap reference plot can be disabled by setting `REFERENCE_BITMAP_ENABLED` to `false`._

## Are there any limitations?

### Limitation #1: No pixel-by-pixel rendering

As Conrod is known to be quite inefficient at rendering image widgets at any high-enough FPS (the likely cause is that it bypasses the GPU and does heavy CPU processing work), it was chosen to ignore the rendering of pixel primitives.

The default Plotters rasterizer has been disabled in that case, as to avoid rendering performance to be degraded without the library user noticing. This guarantees that the GPU is used for rendering, while the CPU does minimal work.

_It means that, some complex plot types may not render well._ Though, rest assured that common plot types have been tested to render exactly as expected, eg. `LineSeries` or `Histogram`.

There are plans to implement those pixel-based rendering methods in the future. If you already have an implementation, feel free to PR this library!

### Limitation #2: Limited text rendering

Only a single font family (ie. `serif`, `sans-serif`, etc.) and a single font style (ie. `regular`, `bold`, etc.) are supported for text rendering. The reason is that Conrod makes it quite tedious to load fonts and pass them over, so we better off limit the backend API to a single font for simplicity's sake. As well, font transforms are not supported due to the underlying Conrod renderer, which does not seem to support text rotations.
