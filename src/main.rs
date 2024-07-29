use std::{num::NonZeroU32, rc::Rc};

use euclid::{default::Point2D};
use softbuffer::{Buffer, Context, Surface};
use winit::{application::ApplicationHandler, dpi::PhysicalPosition, event::WindowEvent, event_loop::{ControlFlow, EventLoop}, raw_window_handle::{HasDisplayHandle, HasWindowHandle}, window::Window};

fn draw_line<T: HasDisplayHandle + HasWindowHandle>(buffer: &mut Buffer<T, T>, width: u32, start: Point2D<u32>, end: Point2D<u32>, color: u32) {
    let start: Point2D<i32> = start.cast();
    let end: Point2D<i32> = end.cast();
    let width = width as i32;

    // horizontal
    if start.y == end.y {
        for i in 0..(end.x - start.x) {
            buffer[(start.y * width + start.x + i) as usize] = color;
        }
    }
    // vertical
    else if start.x == end.x {
        for i in 0..(end.y - start.y) {
            buffer[((start.y + i) * width + start.x) as usize] = color;
        }
    }
    // diagonal
    else {
        let dx = end.x - start.x;
        let dy = end.y - start.y;

        let slope = dy as f32 / dx as f32;

        if dx.abs() >= dy.abs() {
            for i in 0..dx.abs() {
                buffer[((start.y + (if dx.is_negative() {-i} else {i} as f32 * slope) as i32) * width + start.x + if dx.is_negative() {-i} else {i}) as usize] = color;
            }
        } else {
            for i in 0..dy.abs() {
                buffer[((start.y + if dy.is_negative() {-i} else {i}) * width + start.x + (if dy.is_negative() {-i} else {i} as f32 / slope) as i32) as usize] = color;
            }
        }
    }
}

#[derive(Default)]
struct App {
    window: Option<Rc<Window>>,
    surface: Option<Surface<Rc<Window>, Rc<Window>>>,
    pos: Point2D<u32>,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        let window = Rc::new(event_loop.create_window(Window::default_attributes()).unwrap());
        let context = Context::new(window.clone()).unwrap();
        let surface = Surface::new(&context, window.clone()).unwrap();

        self.window = Some(window);
        self.surface = Some(surface);
    }

    fn window_event(
            &mut self,
            event_loop: &winit::event_loop::ActiveEventLoop,
            _window_id: winit::window::WindowId,
            event: winit::event::WindowEvent,
        ) {
        match event {
            WindowEvent::RedrawRequested => {
                let (width, height) = {
                    let size = self.window.as_ref().unwrap().inner_size();
                    (size.width, size.height)
                };

                let surface = self.surface.as_mut().unwrap();

                surface.resize(NonZeroU32::new(width).unwrap(), NonZeroU32::new(height).unwrap()).unwrap();

                let mut buffer = surface.buffer_mut().unwrap();
                for index in 0..(width * height) {
                    let y = index / width;
                    let x = index % width;
                    let mut red = 0;
                    let mut green = 0;
                    let mut blue = 0;

                    buffer[index as usize] = blue | (green << 8) | (red << 16);
                }

                let color = 0 | (255 << 8) | (255 << 16);

                draw_line(&mut buffer, width, Point2D::new(width / 2, height / 2), self.pos, color);

                buffer.present().unwrap();
            },
            WindowEvent::CursorMoved { device_id, position } => {
                self.pos = Point2D::new(position.x as u32, position.y as u32);

                self.window.as_mut().unwrap().request_redraw();
            }
            WindowEvent::CloseRequested => {
                event_loop.exit();
            },
            _ => (),
        }
    }
}


fn main() {
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);

    let mut app = App::default();
    event_loop.run_app(&mut app).unwrap();
}
