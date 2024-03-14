use super::context::AppContext;

pub trait EventHandler {
    fn update(&mut self, context: &mut AppContext);
    fn draw(&mut self, context: &mut AppContext);
}
