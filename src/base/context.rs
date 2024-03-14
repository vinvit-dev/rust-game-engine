use std::{
    borrow::{Borrow, BorrowMut},
    cell::RefCell,
    ops::Deref,
    rc::Rc,
};

use glfw::{Context, PWindow};

use crate::set_clear_color;

use super::config::Config;

pub struct AppContext<'a> {
    pub config: Config<'a>,
    pub window: Option<glfw::PWindow>,
    pub events: Option<glfw::GlfwReceiver<(f64, glfw::WindowEvent)>>,
    pub glfw: Option<glfw::Glfw>,
    pub running: bool,
}

impl<'a> AppContext<'a> {
    pub fn new() -> Self {
        Self {
            config: Config::default(),
            window: None,
            events: None,
            glfw: None,
            running: true,
        }
    }

    pub fn conf(&mut self, conf: Config<'a>) {
        self.config = conf;
    }
    pub fn build(&mut self) {
        // Init glfw
        use glfw::fail_on_errors;
        let mut glfw = glfw::init(fail_on_errors!()).unwrap();

        // Init windo
        glfw.window_hint(glfw::WindowHint::ContextVersion(3, 2));
        glfw.window_hint(glfw::WindowHint::OpenGlProfile(
            glfw::OpenGlProfileHint::Core,
        ));
        glfw.window_hint(glfw::WindowHint::DoubleBuffer(true));
        glfw.window_hint(glfw::WindowHint::Resizable(self.config.window_setup.resize));

        let (mut window, events) = glfw
            .create_window(
                self.config.window_setup.width,
                self.config.window_setup.height,
                self.config.window_setup.title,
                self.config.window_setup.mode,
            )
            .expect("Fail to create widnow");
        window.make_current();

        for event in self.config.clone().pull_events {
            match event {
                crate::base::config::poll_events::PollEvents::Char => window.set_char_polling(true),
                crate::base::config::poll_events::PollEvents::Key => window.set_key_polling(true),
                crate::base::config::poll_events::PollEvents::CursorPos => {
                    window.set_cursor_pos_polling(true)
                }
                crate::base::config::poll_events::PollEvents::MauseButton => {
                    window.set_mouse_button_polling(true)
                }
            }
        }

        // Init OpenGL
        gl::load_with(|f_name| glfw.get_proc_address_raw(f_name));

        self.window = Some(window);
        self.events = Some(events);
        self.glfw = Some(glfw);
    }
}
