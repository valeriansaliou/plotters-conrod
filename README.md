# plotters-conrod

This is an implementation of a Conrod backend for Plotters. This is more efficient than using the default Bitmap backend when plotting in Conrod, as it has been observed that Conrod was quite inefficient at re-rendering images at high FPS (eg. for real-time plotting).

This backend has been optimized as for speed, and as to render plots that look very similar to the default Bitmap backend, if not indistinguishable.

* [Documentation](https://docs.rs/crate/plotters-conrod)
* [Crate](https://crates.io/crates/plotters-conrod)

**🇫🇷 Crafted in Nantes, France.**

## Who uses it?

<table>
<tr>
<td align="center"><a href="https://makair.life/"><img src="https://valeriansaliou.github.io/plotters-conrod/images/makair.png" height="64" /></a></td>
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
plotters-conrod = "0.3.0"
```

## How to use?

First, import `ConrodBackend`:

```rust
use plotters_conrod::ConrodBackend;
```

Then, for each frame you draw, call:

```rust
// 'ids.parent' is the WidgetId of the canvas that contains your plot;
// 'ids.points' is a List of WidgetId, pre-allocated to a large-enough number \
//   of WidgetId so that all Conrod primitives can be inserted as to draw the \
//   full graph. If this number is too low, your app will panic.
let drawing = ConrodBackend::new(
    ui,
    (plot_width, plot_height),
    ids.parent,
    fonts.regular,
    &ids.points,
).into_drawing_area();

// Build your chart as usual here
```

If you are looking for a full example of an implementation, please check [cpu-monitor.rs](./examples/cpu-monitor.rs).

## Are there any limitations?

As Conrod is known to be quite inefficient at rendering images at any high-enough FPS (the likely cause is that it bypasses the GPU and does heavy CPU processing work), it was chosen to ignore the rendering of pixel primitives. The default Plotters rasterizer has been disabled in that case, as to avoid rendering performance to be degraded without the library user noticing. This guarantees that the GPU is used for rendering, while the CPU does minimal work.

_It means that, some complex plot types may not render well._ Though, rest assured that common plot types have been tested to render exactly as expected, eg. the `LineSeries` or `Histogram`.

There are plans to implement those pixel-based rendering methods in the future. If you already have an implementation, feel free to PR this library!

## How to run the examples?

### Example #1: `cpu-monitor`

This example samples your CPU load every second, and renders it in a real-time chart:

```sh
cargo run --release --example cpu-monitor
```

_The first plot uses `plotters-conrod`, while the second plot uses the default Bitmap backend as a reference. This can be used to compare the output and performance of both plotting backends. The Bitmap reference plot can be disabled by setting `REFERENCE_BITMAP_ENABLED` to `false`._
