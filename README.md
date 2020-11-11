# plotters-conrod

This is an implementation of a Conrod backend for Plotters. This is more efficient than using the default Bitmap backend when plotting in Conrod, as it has been observed that Conrod was quite inefficient at re-rendering images at high FPS (eg. for real-time plotting).

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

🚧 TODO
