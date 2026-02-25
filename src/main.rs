use std::sync::Arc;

use parking_lot::Mutex;
use winit::{
    application::ApplicationHandler, event::WindowEvent, event_loop::EventLoop, window::Window,
};

struct WgpuApp {
    window: Arc<Window>,
}

impl WgpuApp {
    async fn new(window: Arc<Window>) -> Self {
        Self { window }
    }
}

#[derive(Default)]
struct WgpuAppHandler {
    app: Arc<Mutex<Option<WgpuApp>>>,
}

impl ApplicationHandler for WgpuAppHandler {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        println!("window resumed");
        if self.app.as_ref().lock().is_some() {
            return;
        }
        let window_attributes = Window::default_attributes().with_title("tutorial1-window");
        let window = Arc::new(event_loop.create_window(window_attributes).unwrap());
        let wgpu_app = pollster::block_on(WgpuApp::new(window));
        self.app.lock().replace(wgpu_app);
    }

    fn suspended(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        println!("window suspended");
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => {
                println!("window event close requested");
                event_loop.exit();
            }
            WindowEvent::Resized(_size) => {
                println!("window event resized")
            }
            WindowEvent::KeyboardInput { .. } => {
                println!("window event keyboard input")
            }
            WindowEvent::RedrawRequested => {
                println!("window event redraw requested")
            }
            _ => (),
        }
    }
}

fn main() {
    println!("Hello, world!");
    env_logger::init();
    let events_loop = EventLoop::new().unwrap();
    let mut handler = WgpuAppHandler::default();
    // println!("handler is none {}",handler.app.as_ref().lock().is_none());
    events_loop.run_app(&mut handler);
}
