// plotters-conrod
//
// Conrod backend for Plotters
// Copyright: 2020, Valerian Saliou <valerian@valeriansaliou.name>
// License: MIT

#[macro_use]
extern crate conrod_core;

use std::borrow::Cow;
use std::collections::VecDeque;
use std::path::Path;
use std::time::{Duration, Instant};

use conrod_core::{self as conrod, Positionable, Sizeable, Widget};
use conrod_winit::WinitWindow;
use glium::{self, Surface};
use plotters::prelude::*;
use plotters_conrod::{ConrodBackend, ConrodBackendReusableGraph};
use sysinfo::{ProcessorExt, RefreshKind, System, SystemExt};

const PLOT_WIDTH: u32 = 800;
const PLOT_HEIGHT: u32 = 480;
const PLOT_PIXELS: usize = (PLOT_WIDTH * PLOT_HEIGHT) as usize;
const PLOT_SECONDS: usize = 10;
const PLOT_LINE_COLOR: RGBColor = plotters::style::RGBColor(0, 196, 255);

const WINDOW_WIDTH: u32 = PLOT_WIDTH;

const WINDOW_HEIGHT: u32 = if REFERENCE_BITMAP_ENABLED {
    PLOT_HEIGHT * 2
} else {
    PLOT_HEIGHT
};

const TITLE_FONT_SIZE: u32 = 22;
const TITLE_MARGIN_LEFT: f64 = 10.0;
const TITLE_MARGIN_TOP: f64 = 8.0;

const SAMPLE_EVERY: Duration = Duration::from_secs(1);

const FRAME_TICK_RATE: usize = 30;

// This can be used to disable the reference Bitmap chart
const REFERENCE_BITMAP_ENABLED: bool = true;

pub struct ImageIds {
    pub bitmap_plot: conrod::image::Id,
}

pub struct GliumDisplayWinitWrapper(pub glium::Display);

impl WinitWindow for GliumDisplayWinitWrapper {
    fn get_inner_size(&self) -> Option<(u32, u32)> {
        let s = self.0.gl_window().window().inner_size();
        Some((s.width, s.height))
    }

    fn hidpi_factor(&self) -> f32 {
        self.0.gl_window().window().scale_factor() as f32
    }
}

conrod_winit::v025_conversion_fns!();

impl ImageIds {
    pub fn new(
        image_map: &mut conrod_core::image::Map<glium::texture::SrgbTexture2d>,
        bitmap_plot_texture: glium::texture::SrgbTexture2d,
    ) -> ImageIds {
        ImageIds {
            bitmap_plot: image_map.insert(bitmap_plot_texture),
        }
    }
}

widget_ids!(struct Ids {
    bitmap_wrapper,
    bitmap_text,
    bitmap_plot,
    conrod_wrapper,
    conrod_text
});

fn main() {
    // Bootstrap Glium
    let event_loop = glium::glutin::event_loop::EventLoop::new();

    let window = glium::glutin::window::WindowBuilder::new()
        .with_title("CPU Monitor Example")
        .with_inner_size(glium::glutin::dpi::LogicalSize::new(
            WINDOW_WIDTH,
            WINDOW_HEIGHT,
        ))
        .with_resizable(false)
        .with_decorations(true)
        .with_always_on_top(false);
    let context = glium::glutin::ContextBuilder::new()
        .with_vsync(true)
        .with_multisampling(4);

    // Bootstrap Conrod
    let mut interface = conrod::UiBuilder::new([WINDOW_WIDTH as f64, WINDOW_HEIGHT as f64]).build();

    let display =
        GliumDisplayWinitWrapper(glium::Display::new(window, context, &event_loop).unwrap());

    let ids = Ids::new(interface.widget_id_generator());

    let mut image_map = conrod::image::Map::<glium::texture::SrgbTexture2d>::new();
    let mut renderer = conrod_glium::Renderer::new(&display.0).unwrap();

    // Load fonts
    let font_regular = interface
        .fonts
        .insert_from_file(Path::new("./examples/fonts/notosans-regular.ttf"))
        .unwrap();
    let font_bold = interface
        .fonts
        .insert_from_file(Path::new("./examples/fonts/notosans-bold.ttf"))
        .unwrap();

    // Bootstrap CPU percent collector
    let mut system = System::new_with_specifics(RefreshKind::new().with_cpu());
    let (mut cpu_last_sample_value, mut cpu_last_sample_time) = (0, Instant::now());
    let mut data_points: VecDeque<(chrono::DateTime<chrono::Utc>, i32)> =
        VecDeque::with_capacity(FRAME_TICK_RATE * PLOT_SECONDS);

    // Bootstrap images map
    let image_ids = ImageIds::new(
        &mut image_map,
        render_bitmap_plot(&display, &mut data_points),
    );

    // Bootstrap Conrod backend IDs graph
    let mut conrod_graph = ConrodBackendReusableGraph::build();

    // Initialize common canvas style
    let mut canvas_style = conrod::widget::canvas::Style::default();

    canvas_style.border = Some(0.0);
    canvas_style.border_color = Some(conrod::color::TRANSPARENT);
    canvas_style.color = Some(conrod::color::TRANSPARENT);

    // Initialize common title text style
    let mut title_text_style = conrod_core::widget::primitive::text::Style::default();

    title_text_style.font_id = Some(Some(font_bold));
    title_text_style.color = Some(conrod::color::WHITE);
    title_text_style.font_size = Some(TITLE_FONT_SIZE);

    // Run events handler
    let mut ui_updated = true;
    let mut plot_data_updated = true;
    event_loop.run(move |event, _window_target, control_flow| {
        let tick_start_time = Instant::now();

        // Sample CPU point?
        if tick_start_time.duration_since(cpu_last_sample_time) > SAMPLE_EVERY {
            system.refresh_cpu();
            cpu_last_sample_value = system.global_processor_info().cpu_usage() as i32;
            cpu_last_sample_time = tick_start_time;
            plot_data_updated = true;
        }

        // Append point in data points (plus, trim to maximum size & clean expired points)
        if plot_data_updated {
            data_points.truncate(FRAME_TICK_RATE * PLOT_SECONDS - 1);

            if !data_points.is_empty() {
                let older =
                    data_points.front().unwrap().0 - chrono::Duration::seconds(PLOT_SECONDS as _);

                while data_points.back().map(|p| p.0 < older).unwrap_or(false) {
                    data_points.pop_back();
                }
            }

            data_points.push_front((chrono::Utc::now(), cpu_last_sample_value));
        }

        // Handle incoming UI events (ie. from the window, eg. 'ESC' key is pressed)
        match &event {
            glium::glutin::event::Event::WindowEvent { event, .. } => match event {
                // Break from the loop upon `Escape`.
                glium::glutin::event::WindowEvent::CloseRequested
                | glium::glutin::event::WindowEvent::KeyboardInput {
                    input:
                        glium::glutin::event::KeyboardInput {
                            virtual_keycode: Some(glium::glutin::event::VirtualKeyCode::Escape),
                            ..
                        },
                    ..
                } => *control_flow = glium::glutin::event_loop::ControlFlow::Exit,
                _ => {}
            },
            _ => {}
        }

        // Use the `winit` backend feature to convert the winit event to a conrod one.
        if let Some(event) = convert_event(&event, &display.0.gl_window().window()) {
            interface.handle_event(event);
            ui_updated = true;
        }

        let should_update_ui = ui_updated || plot_data_updated;

        match &event {
            glium::glutin::event::Event::MainEventsCleared => {
                if should_update_ui {
                    ui_updated = false;
                    plot_data_updated = false;

                    let ui = &mut interface.set_widgets();

                    // Draw Conrod chart
                    conrod::widget::canvas::Canvas::new()
                        .w_h(PLOT_WIDTH as _, PLOT_HEIGHT as _)
                        .with_style(canvas_style)
                        .top_left()
                        .set(ids.conrod_wrapper, ui);

                    render_conrod_plot(ui, &mut data_points, &ids, font_regular, &mut conrod_graph);

                    conrod::widget::Text::new("Conrod test chart")
                        .with_style(title_text_style)
                        .top_left_with_margins_on(
                            ids.conrod_wrapper,
                            TITLE_MARGIN_TOP,
                            TITLE_MARGIN_LEFT,
                        )
                        .set(ids.conrod_text, ui);

                    // Draw Bitmap chart?
                    if REFERENCE_BITMAP_ENABLED {
                        conrod::widget::canvas::Canvas::new()
                            .w_h(PLOT_WIDTH as _, PLOT_HEIGHT as _)
                            .with_style(canvas_style)
                            .down_from(ids.conrod_wrapper, 0.0)
                            .set(ids.bitmap_wrapper, ui);

                        image_map.replace(
                            image_ids.bitmap_plot,
                            render_bitmap_plot(&display, &mut data_points),
                        );

                        conrod::widget::Image::new(image_ids.bitmap_plot)
                            .w_h(PLOT_WIDTH as _, PLOT_HEIGHT as _)
                            .top_left_of(ids.bitmap_wrapper)
                            .set(ids.bitmap_plot, ui);

                        conrod::widget::Text::new("Bitmap reference chart")
                            .with_style(title_text_style)
                            .top_left_with_margins_on(
                                ids.bitmap_wrapper,
                                TITLE_MARGIN_TOP,
                                TITLE_MARGIN_LEFT,
                            )
                            .set(ids.bitmap_text, ui);
                    }

                    display.0.gl_window().window().request_redraw();
                }
            }
            glium::glutin::event::Event::RedrawRequested(_) => {
                if let Some(primitives) = interface.draw_if_changed() {
                    renderer.fill(&display.0, primitives, &image_map);

                    let mut target = display.0.draw();

                    target.clear_color(0.0, 0.0, 0.0, 1.0);

                    renderer.draw(&display.0, &mut target, &image_map).unwrap();

                    target.finish().unwrap();
                }
            }
            _ => {}
        }
    });
}

fn render_bitmap_plot(
    display: &GliumDisplayWinitWrapper,
    data_points: &VecDeque<(chrono::DateTime<chrono::Utc>, i32)>,
) -> glium::texture::SrgbTexture2d {
    if REFERENCE_BITMAP_ENABLED {
        let mut buffer_rgb: Vec<u8> = vec![0; PLOT_PIXELS * 3];

        // Switch context so that we can re-use 'buffer_rgb' later in read mode (mutable here)
        {
            let bitmap_drawing =
                BitMapBackend::with_buffer(&mut buffer_rgb, (PLOT_WIDTH, PLOT_HEIGHT))
                    .into_drawing_area();

            plot(data_points, &bitmap_drawing);
        }

        let buffer_reversed = reverse_rgb(&buffer_rgb, PLOT_WIDTH, PLOT_HEIGHT);

        glium::texture::SrgbTexture2d::new(
            &display.0,
            glium::texture::RawImage2d {
                data: Cow::Borrowed(&buffer_reversed),
                width: PLOT_WIDTH,
                height: PLOT_HEIGHT,
                format: glium::texture::ClientFormat::U8U8U8,
            },
        )
        .unwrap()
    } else {
        glium::texture::SrgbTexture2d::empty(&display.0, PLOT_WIDTH, PLOT_HEIGHT).unwrap()
    }
}

fn render_conrod_plot<'a, 'b>(
    ui: &'a mut conrod::UiCell<'b>,
    data_points: &VecDeque<(chrono::DateTime<chrono::Utc>, i32)>,
    ids: &'b Ids,
    font: conrod_core::text::font::Id,
    graph: &mut ConrodBackendReusableGraph,
) {
    let conrod_drawing = ConrodBackend::new(
        ui,
        (PLOT_WIDTH, PLOT_HEIGHT),
        ids.conrod_wrapper,
        font,
        graph,
    )
    .into_drawing_area();

    plot(data_points, &conrod_drawing);
}

fn plot<D: IntoDrawingArea>(
    data_points: &VecDeque<(chrono::DateTime<chrono::Utc>, i32)>,
    drawing: &DrawingArea<D, plotters::coord::Shift>,
) {
    // Acquire time range
    let newest_time = data_points
        .front()
        .unwrap_or(&(
            chrono::DateTime::from_utc(chrono::NaiveDateTime::from_timestamp(0, 0), chrono::Utc),
            0,
        ))
        .0;
    let oldest_time = newest_time - chrono::Duration::seconds(PLOT_SECONDS as i64);

    let mut chart = ChartBuilder::on(&drawing)
        .x_label_area_size(0)
        .y_label_area_size(28)
        .margin(20)
        .build_cartesian_2d(oldest_time..newest_time, 0..100)
        .expect("failed to build chart");

    chart
        .configure_mesh()
        .bold_line_style(&plotters::style::colors::WHITE.mix(0.1))
        .light_line_style(&plotters::style::colors::WHITE.mix(0.05))
        .axis_style(ShapeStyle::from(&plotters::style::colors::WHITE.mix(0.45)).stroke_width(1))
        .y_labels(10)
        .y_label_style(
            ("sans-serif", 15)
                .into_font()
                .color(&plotters::style::colors::WHITE.mix(0.65)),
        )
        .y_label_formatter(&|y| format!("{}%", y))
        .draw()
        .expect("failed to draw chart mesh");

    chart
        .draw_series(
            AreaSeries::new(
                data_points.iter().map(|x| (x.0, x.1 as i32)),
                0,
                &PLOT_LINE_COLOR.mix(0.175),
            )
            .border_style(ShapeStyle::from(&PLOT_LINE_COLOR).stroke_width(2)),
        )
        .expect("failed to draw chart data");
}

fn reverse_rgb(image: &[u8], width: u32, height: u32) -> Vec<u8> {
    // Reverses an image over the Y axis, so that it is displayed on screen correctly, as the \
    //   renderer works on an inverted Y axis.
    // Notice: this is a more efficient implementation for RGB images, which is not the norm over \
    //   here, but is useful as to reverse frequently updated images like the graphs.
    let (width_value, height_value) = (width as usize, height as usize);

    let mut buffer_reversed: Vec<u8> = vec![0; width_value * height_value * 3];

    for row in 0..(height_value - 1) {
        let (row_start_start, row_start_end) =
            (row * width_value, (height_value - row - 1) * width_value);

        for column in 0..(width_value - 1) {
            let (start_index, end_index) =
                ((row_start_start + column) * 3, (row_start_end + column) * 3);

            buffer_reversed[end_index] = image[start_index];
            buffer_reversed[end_index + 1] = image[start_index + 1];
            buffer_reversed[end_index + 2] = image[start_index + 2];
        }
    }

    buffer_reversed
}
