use crate::game::GameEvent;
use std::collections::HashMap;
use std::sync::mpsc::Receiver;
use std::sync::{Arc, Mutex};

pub type Payload = Vec<u8>;
pub trait Handler: Send + Sync {
    fn handle(&mut self, event: &GameEvent, payload: &Payload);
}

pub struct EventLoop {
    register: Arc<Mutex<HashMap<GameEvent, Vec<Arc<Mutex<dyn Handler>>>>>>,
    rx: Receiver<(GameEvent, Payload)>,
}

impl EventLoop {
    pub fn new(rx: Receiver<(GameEvent, Payload)>) -> Self {
        EventLoop {
            register: Arc::new(Mutex::new(HashMap::new())),
            rx,
        }
    }

    pub fn register_handler(&self, event: GameEvent, handler: Arc<Mutex<dyn Handler>>) {
        let mut registry = self.register.lock().unwrap();
        registry.entry(event).or_insert_with(Vec::new).push(handler);
    }

    fn handle_event(&self, event: &GameEvent, payload: &Payload) {
        let registry = self.register.lock().unwrap();
        if let Some(handlers) = registry.get(&event) {
            for handler in handlers {
                if let Ok(mut handler) = handler.lock() {
                    handler.handle(&event, &payload);
                }
            }
        }
    }
    pub fn start(&self) {
        loop {
            for (event, payload) in &self.rx {
                self.handle_event(&event, &payload);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::mpsc;

    #[derive(Clone)]
    struct TestHandler {
        pub called: Arc<Mutex<bool>>,
    }

    impl Handler for TestHandler {
        fn handle(&mut self, _event: &GameEvent, _payload: &Payload) {
            *self.called.lock().unwrap() = true;
        }
    }

    macro_rules! setup_event_loop_and_handler {
    () => {{
        let (_tx, rx) = mpsc::channel();
        let event_loop = EventLoop::new(rx);
        let called_flag = Arc::new(Mutex::new(false));
        let handler = Arc::new(Mutex::new(TestHandler {
            called: Arc::clone(&called_flag),
        }));
        (event_loop, called_flag, handler)
    }};
}

    #[test]
    fn test_register_handler() {
        let (_tx, rx) = mpsc::channel();
        let event_loop = EventLoop::new(rx);
        let handler = Arc::new(Mutex::new(TestHandler {
            called: Arc::new(Mutex::new(false)),
        }));

        event_loop.register_handler(GameEvent::TileClicked, handler);

        let registry = event_loop.register.lock().unwrap();
        assert!(registry.contains_key(&GameEvent::TileClicked));
        assert_eq!(registry[&GameEvent::TileClicked].len(), 1);
    }

    #[test]
    fn test_handle_empty_event() {
        let (event_loop, called_flag, handler) = setup_event_loop_and_handler!();
        event_loop.register_handler(GameEvent::TileClicked, handler);
        event_loop.handle_event(&GameEvent::TileClicked, &vec![]);

        let called = *called_flag.lock().unwrap();
        assert_eq!(called, true);
    }

    #[test]
    fn test_handle_event_with_payload() {
        let (event_loop, called_flag, handler) = setup_event_loop_and_handler!();
        event_loop.register_handler(GameEvent::TileClicked, handler);
        event_loop.handle_event(&GameEvent::TileClicked, &vec![1, 2, 3]);

        assert!(*called_flag.lock().unwrap());
    }
}
