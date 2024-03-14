use self::{context::AppContext, even_handler::EventHandler};

pub mod config;
pub mod context;
pub mod even_handler;

pub fn run(mut context: &mut AppContext, state: &mut dyn EventHandler) {
    while context.running {
        state.update(&mut context);
        state.draw(&mut context);
    }
}
