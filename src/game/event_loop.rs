use std::sync::mpsc::{self, Receiver};
use crate::game::GameEvent;
use std::sync::{Arc, Mutex};
use std::{collections::HashMap};

pub type Payload = Vec<u8>;
pub trait Handler: Send + Sync {
    fn handle(&self, event: &GameEvent, payload: &Payload);
}

pub struct Event {
    pub event: GameEvent,
    pub payload: Payload
}

pub struct EventLoop {
    pub register: Arc<Mutex<HashMap<GameEvent, Vec<Arc<dyn Handler>>>>>,
    rx: Receiver<(GameEvent, Payload)>,
}

impl EventLoop {
    pub fn new(rx: Receiver<(GameEvent, Payload)>, tiles: Vec<GameEvent>) -> Self {
        EventLoop {
            register: Arc::new(Mutex::new(HashMap::new())), rx
        }
    }

    pub fn register_handler(&self, event: GameEvent, handler: Arc<dyn Handler>) {
        let mut registry = self.register.lock().unwrap();
        registry.entry(event).or_insert_with(Vec::new).push(handler);
    }
    pub fn start(&self) {
        let registry = Arc::clone(&self.register);
        loop {
            for rec in &self.rx {
                let event = rec;
                let registry = registry.lock().unwrap();
                if let Some(handlers) = registry.get(&event.0) {
                    for handler in handlers {
                        handler.handle(&event.0, &event.1);
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;

    struct TestHandler {
        pub called: Arc<Mutex<bool>>,
    }

    impl Handler for TestHandler {
        fn handle(&self, event: &GameEvent, payload: &Payload) {
            let mut called = self.called.lock().unwrap();
            *called = true;
        }
    }

    #[test]
    fn test_register_handler() {
        let (tx, rx) = mpsc::channel();
        let mut event_loop = EventLoop::new(rx,vec![]);
        let handler = Arc::new(TestHandler {
            called: Arc::new(Mutex::new(false)),
        });

        event_loop.register_handler(GameEvent::TileClicked, handler);

        let registry = event_loop.register.lock().unwrap();
        assert!(registry.contains_key(&GameEvent::TileClicked));
        assert_eq!(registry[&GameEvent::TileClicked].len(), 1);
    }

    #[test]
    fn test_event_handling() {
        let (tx, rx) = mpsc::channel();
        let mut event_loop = EventLoop::new(rx,vec![]);
        let called = Arc::new(Mutex::new(false));
        let handler = Arc::new(TestHandler { called: called.clone() });

        event_loop.register_handler(GameEvent::TileClicked, handler);

        // Run the event loop in a separate thread to avoid blocking
        let handle = std::thread::spawn(move || {
            event_loop.start();
        });
        tx.send((GameEvent::TileClicked, vec![])).unwrap();
        // Give the event loop some time to process the event
        std::thread::sleep(std::time::Duration::from_millis(100));

        // Check if the handler was called
        let called = called.lock().unwrap();
        assert!(*called);

        // Stop the event loop thread
        handle.thread().unpark();
    }
}