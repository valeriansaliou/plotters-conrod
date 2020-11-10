// plotters-conrod
//
// Conrod backend for Plotters
// Copyright: 2020, Valerian Saliou <valerian@valeriansaliou.name>
// License: MIT

#[macro_use]
extern crate conrod_core;

use std::collections::VecDeque;
use std::thread;
use std::time::Duration;

use chrono;
use conrod_core::{self as conrod, Colorable, Positionable, Widget};
use conrod_glium;
use glium::{self, Surface};
use plotters::prelude::*;
use plotters::style::TextStyle;
use psutil::*;

const WINDOW_WIDTH: u32 = 800;
const WINDOW_HEIGHT: u32 = 600;

const PLOT_POINTS: usize = 30;

const SAMPLE_TICK_WAIT: Duration = Duration::from_millis(250);

widget_ids!(struct Ids { text });

fn main() {
    // Bootstrap Glium
    let mut events_loop = glium::glutin::EventsLoop::new();

    let window = glium::glutin::WindowBuilder::new()
        .with_title("CPU Monitor Example")
        .with_dimensions((WINDOW_WIDTH, WINDOW_HEIGHT).into())
        .with_resizable(false);
    let context = glium::glutin::ContextBuilder::new()
        .with_vsync(true)
        .with_multisampling(4);
    let display = glium::Display::new(window, context, &events_loop).unwrap();

    // Bootstrap Conrod
    let mut ui = conrod::UiBuilder::new([WINDOW_WIDTH as f64, WINDOW_HEIGHT as f64]).build();
    let mut renderer = conrod_glium::Renderer::new(&display).unwrap();

    let ids = Ids::new(ui.widget_id_generator());
    let image_map = conrod::image::Map::<glium::texture::Texture2d>::new();

    // Bootstrap CPU percent collector
    let mut cpu_percent_collector = cpu::CpuPercentCollector::new().unwrap();
    let mut data_points: VecDeque<(chrono::DateTime<chrono::Utc>, u8)> =
        VecDeque::with_capacity(PLOT_POINTS);

    // Start drawing loop
    loop {
        let ui = &mut ui.set_widgets();

        plot(&mut cpu_percent_collector, &mut data_points);

        conrod::widget::Text::new("Hello World!")
            .middle_of(ui.window)
            .color(conrod::color::WHITE)
            .font_size(32)
            .set(ids.text, ui);

        if let Some(primitives) = ui.draw_if_changed() {
            renderer.fill(&display, primitives, &image_map);

            let mut target = display.draw();

            target.clear_color(1.0, 1.0, 1.0, 1.0);

            renderer.draw(&display, &mut target, &image_map).unwrap();

            target.finish().unwrap();
        }

        thread::sleep(SAMPLE_TICK_WAIT);
    }
}

fn plot(
    cpu_percent_collector: &mut cpu::CpuPercentCollector,
    data_points: &mut VecDeque<(chrono::DateTime<chrono::Utc>, u8)>,
) {
    // Sample current CPU usage and append to data points (pop expired points)
    let cpu_percent = cpu_percent_collector.cpu_percent().unwrap();

    data_points.truncate(PLOT_POINTS - 1);
    data_points.push_front((chrono::Utc::now(), cpu_percent as u8));

    let mut buffer_rgb: Vec<u8> = vec![0; (WINDOW_WIDTH * WINDOW_HEIGHT * 3) as usize];

    let drawing = BitMapBackend::with_buffer(&mut buffer_rgb, (WINDOW_WIDTH, WINDOW_HEIGHT))
        .into_drawing_area();

    let mut chart = ChartBuilder::on(&drawing)
        .x_label_area_size(0)
        .y_label_area_size(20)
        .build_cartesian_2d(0..PLOT_POINTS, 0.0..100.0)
        .expect("failed to build chart");

    chart
        .configure_mesh()
        .bold_line_style(&plotters::style::colors::BLACK.mix(0.22))
        .light_line_style(&plotters::style::colors::BLACK)
        .y_labels(10)
        .y_label_style(TextStyle::from(
            ("sans-serif", 15)
                .into_font()
                .color(&plotters::style::colors::BLACK),
        ))
        .draw()
        .expect("failed to draw chart mesh");

    chart
        .draw_series(
            LineSeries::new(
                data_points.iter(),
                ShapeStyle::from(&plotters::style::colors::BLACK)
                    .filled()
                    .stroke_width(1),
            )
            .point_size(2),
        )
        .expect("failed to draw chart data");
}
