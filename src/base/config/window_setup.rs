#[derive(Clone)]
pub struct WindowSetup<'a> {
    pub title: &'a str,
    pub width: u32,
    pub height: u32,
    pub mode: glfw::WindowMode<'a>,
    pub resize: bool,
}

impl<'a> Default for WindowSetup<'a> {
    fn default() -> Self {
        Self {
            title: "Window",
            width: 640,
            height: 480,
            mode: glfw::WindowMode::Windowed,
            resize: true,
        }
    }
}
