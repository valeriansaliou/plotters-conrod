// plotters-conrod
//
// Conrod backend for Plotters
// Copyright: 2020, Valerian Saliou <valerian@valeriansaliou.name>
// License: MIT

#[macro_use]
extern crate conrod_core;

use std::borrow::Cow;
use std::collections::VecDeque;
use std::thread;
use std::time::Duration;

use chrono;
use conrod_core::{self as conrod, Positionable, Sizeable, Widget};
use conrod_glium;
use conrod_winit::WinitWindow;
use glium::{self, Surface};
use plotters::prelude::*;
use plotters::style::TextStyle;
use psutil::*;

const WINDOW_WIDTH: u32 = 800;
const WINDOW_HEIGHT: u32 = 480;

const PLOT_POINTS: usize = 30;

const SAMPLES_PER_SECOND: usize = 10;
const SAMPLE_TICK_WAIT: Duration = Duration::from_millis(1000 / SAMPLES_PER_SECOND as u64);

pub struct GliumDisplayWinitWrapper(pub glium::Display);
pub struct EventLoop;

pub struct EventsHandler {
    event_loop: EventLoop,
}

pub enum EventsHandlerOutcome {
    Break,
    Continue,
}

pub struct ImageIds {
    pub plot: conrod::image::Id,
}

impl WinitWindow for GliumDisplayWinitWrapper {
    fn get_inner_size(&self) -> Option<(u32, u32)> {
        self.0.gl_window().get_inner_size().map(Into::into)
    }

    fn hidpi_factor(&self) -> f32 {
        self.0.gl_window().get_hidpi_factor() as _
    }
}

impl EventLoop {
    pub fn new() -> Self {
        EventLoop {}
    }

    /// Produce an iterator yielding all available events.
    pub fn next(
        &mut self,
        events_loop: &mut glium::glutin::EventsLoop,
    ) -> Vec<glium::glutin::Event> {
        // Collect all pending events.
        let mut events = Vec::new();

        events_loop.poll_events(|event| events.push(event));

        events
    }
}

impl EventsHandler {
    pub fn new() -> EventsHandler {
        EventsHandler {
            event_loop: EventLoop::new(),
        }
    }

    pub fn handle(
        &mut self,
        display: &GliumDisplayWinitWrapper,
        interface: &mut conrod::Ui,
        mut events_loop: &mut glium::glutin::EventsLoop,
    ) -> EventsHandlerOutcome {
        for event in self.event_loop.next(&mut events_loop) {
            // Use the `winit` backend feature to convert the winit event to a conrod one.
            if let Some(event) = convert_event(event.clone(), display) {
                interface.handle_event(event);
            }

            // Break from the loop upon `Escape` or closed window.
            if let glium::glutin::Event::WindowEvent { event, .. } = event.clone() {
                match event {
                    glium::glutin::WindowEvent::CloseRequested
                    | glium::glutin::WindowEvent::KeyboardInput {
                        input:
                            glium::glutin::KeyboardInput {
                                virtual_keycode: Some(glium::glutin::VirtualKeyCode::Escape),
                                ..
                            },
                        ..
                    } => {
                        return EventsHandlerOutcome::Break;
                    }
                    _ => (),
                }
            }
        }

        EventsHandlerOutcome::Continue
    }
}

impl ImageIds {
    pub fn new(
        image_map: &mut conrod_core::image::Map<glium::texture::SrgbTexture2d>,
        plot_texture: glium::texture::SrgbTexture2d,
    ) -> ImageIds {
        ImageIds {
            plot: image_map.insert(plot_texture),
        }
    }
}

conrod_winit::conversion_fns!();
widget_ids!(struct Ids { plot });

fn main() {
    // Bootstrap Glium
    let mut events_loop = glium::glutin::EventsLoop::new();

    let window = glium::glutin::WindowBuilder::new()
        .with_title("CPU Monitor Example")
        .with_dimensions((WINDOW_WIDTH, WINDOW_HEIGHT).into())
        .with_resizable(false)
        .with_decorations(true)
        .with_always_on_top(false);
    let context = glium::glutin::ContextBuilder::new()
        .with_vsync(true)
        .with_multisampling(4);

    // Bootstrap Conrod
    let mut interface = conrod::UiBuilder::new([WINDOW_WIDTH as f64, WINDOW_HEIGHT as f64]).build();

    let display =
        GliumDisplayWinitWrapper(glium::Display::new(window, context, &events_loop).unwrap());

    let ids = Ids::new(interface.widget_id_generator());
    let mut image_map = conrod::image::Map::<glium::texture::SrgbTexture2d>::new();

    let mut renderer = conrod_glium::Renderer::new(&display.0).unwrap();

    // Bootstrap CPU percent collector
    let mut cpu_percent_collector = cpu::CpuPercentCollector::new().unwrap();
    let mut data_points: VecDeque<(chrono::DateTime<chrono::Utc>, u8)> =
        VecDeque::with_capacity(PLOT_POINTS);

    // Bootstrap images map
    let image_ids = ImageIds::new(
        &mut image_map,
        plot(&display, &mut cpu_percent_collector, &mut data_points),
    );

    // Start evens handler
    let mut events_handler = EventsHandler::new();

    // Start drawing loop
    'main: loop {
        // Handle incoming UI events (ie. from the window, eg. 'ESC' key is pressed)
        match events_handler.handle(&display, &mut interface, &mut events_loop) {
            EventsHandlerOutcome::Break => break 'main,
            EventsHandlerOutcome::Continue => {}
        }

        // Draw UI for tick
        {
            let mut ui = interface.set_widgets();

            image_map.replace(
                image_ids.plot,
                plot(&display, &mut cpu_percent_collector, &mut data_points),
            );

            conrod::widget::Image::new(image_ids.plot)
                .w_h(WINDOW_WIDTH as _, WINDOW_HEIGHT as _)
                .middle()
                .set(ids.plot, &mut ui);

            ui.needs_redraw();
        }

        // Draw interface (if it was updated)
        if let Some(primitives) = interface.draw_if_changed() {
            renderer.fill(&display.0, primitives, &image_map);

            let mut target = display.0.draw();

            target.clear_color(1.0, 1.0, 1.0, 1.0);

            renderer.draw(&display.0, &mut target, &image_map).unwrap();

            target.finish().unwrap();
        }

        thread::sleep(SAMPLE_TICK_WAIT);
    }
}

fn plot(
    display: &GliumDisplayWinitWrapper,
    cpu_percent_collector: &mut cpu::CpuPercentCollector,
    data_points: &mut VecDeque<(chrono::DateTime<chrono::Utc>, u8)>,
) -> glium::texture::SrgbTexture2d {
    let mut buffer_rgb: Vec<u8> = vec![0; (WINDOW_WIDTH * WINDOW_HEIGHT * 3) as usize];

    let drawing = BitMapBackend::with_buffer(&mut buffer_rgb, (WINDOW_WIDTH, WINDOW_HEIGHT))
        .into_drawing_area();

    // Sample current CPU usage and append to data points (pop expired points)
    let cpu_percent = cpu_percent_collector.cpu_percent().unwrap();

    data_points.truncate(PLOT_POINTS - 1);
    data_points.push_front((chrono::Utc::now(), cpu_percent as u8));

    // Acquire time range
    let newest_time = data_points
        .front()
        .unwrap_or(&(
            chrono::DateTime::from_utc(chrono::NaiveDateTime::from_timestamp(0, 0), chrono::Utc),
            0,
        ))
        .0;
    let oldest_time =
        newest_time - chrono::Duration::seconds((SAMPLES_PER_SECOND * PLOT_POINTS) as i64);

    let mut chart = ChartBuilder::on(&drawing)
        .x_label_area_size(0)
        .y_label_area_size(28)
        .build_cartesian_2d(oldest_time..newest_time, 0..100)
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
                data_points.iter().map(|x| (x.0, x.1 as i32)),
                ShapeStyle::from(&plotters::style::colors::BLACK)
                    .filled()
                    .stroke_width(1),
            )
            .point_size(2),
        )
        .expect("failed to draw chart data");

    drop(chart);
    drop(drawing);

    let buffer_reversed = reverse_rgb(&buffer_rgb, WINDOW_WIDTH, WINDOW_HEIGHT);

    glium::texture::SrgbTexture2d::new(
        &display.0,
        glium::texture::RawImage2d {
            data: Cow::Borrowed(&buffer_reversed),
            width: WINDOW_WIDTH,
            height: WINDOW_HEIGHT,
            format: glium::texture::ClientFormat::U8U8U8,
        },
    )
    .unwrap()
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
