pub mod game_event;
mod event_loop;
mod handlers;

pub use game_event::GameEvent;
pub use event_loop::EventLoop;
pub use handlers::MouseClickHandler;
pub use event_loop::Dispatcher;