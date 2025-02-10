use std::sync::mpsc::{self, Sender, Receiver};
use crate::game::GameEvent;
use std::sync::{Arc, Mutex};
use std::{collections::HashMap, thread};

pub type Payload = Vec<u8>;
pub trait Handler: Send + Sync {
    fn handle(&self, event: &GameEvent, payload: &Payload);
}

pub struct Event {
    pub event: GameEvent,
    pub payload: Payload
}
#[derive(Clone)]
pub struct Dispatcher {
    pub tx: Sender<(GameEvent, Payload)>,
}

impl Dispatcher {
    pub fn new() -> Self {
        let (tx, _rx) = mpsc::channel();
        Self { tx }
    }
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
                if let event = rec {
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
        let mut event_loop = EventLoop::new(vec![]);
        let handler = Arc::new(TestHandler {
            called: Arc::new(Mutex::new(false)),
        });

        event_loop.register_handler(GameEvent::TileClicked, handler.clone());

        let registry = event_loop.register.lock().unwrap();
        assert!(registry.contains_key(&GameEvent::TileClicked));
        assert_eq!(registry[&GameEvent::TileClicked].len(), 1);
    }

    #[test]
    fn test_add_event() {
        let mut event_loop = EventLoop::new(vec![]);
        event_loop.add_event(GameEvent::TileClicked, vec![1, 2, 3]);

        let events = event_loop.events.lock().unwrap();
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].event, GameEvent::TileClicked);
        assert_eq!(events[0].payload, vec![1, 2, 3]);
    }

    #[test]
    fn test_event_handling() {
        let mut event_loop = EventLoop::new(vec![]);
        let called = Arc::new(Mutex::new(false));
        let handler = Arc::new(TestHandler { called: called.clone() });

        event_loop.register_handler(GameEvent::TileClicked, handler.clone());
        event_loop.add_event(GameEvent::TileClicked, vec![]);

        // Run the event loop in a separate thread to avoid blocking
        let event_loop_clone = event_loop.clone();
        let handle = std::thread::spawn(move || {
            event_loop_clone.start();
        });

        // Give the event loop some time to process the event
        std::thread::sleep(std::time::Duration::from_millis(100));

        // Check if the handler was called
        let called = called.lock().unwrap();
        assert!(*called);

        // Stop the event loop thread
        handle.thread().unpark();
    }
}

//1. Dodać rejestrację handlerów
//2. Dodać wywołanie handlerów
//3. Dodać obsługę zdarzeń
// https://dev.to/luisccc/learning-by-doing-event-loop-in-rust-hf1