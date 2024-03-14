use self::{poll_events::PollEvents, window_setup::WindowSetup};

pub mod poll_events;
pub mod window_setup;

#[derive(Default, Clone)]
pub struct Config<'a> {
    pub window_setup: WindowSetup<'a>,
    pub pull_events: Vec<PollEvents>,
}
