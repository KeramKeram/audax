use crate::game::GameEvent;
use std::sync::{Arc, Mutex};
use std::{collections::HashMap, thread};
use std::collections::VecDeque;

pub type Payload = Vec<u8>;
pub trait Handler: Send + Sync {
    fn handle(&self, event: GameEvent, payload: Payload);
}

pub struct Event {
    pub event: GameEvent,
    pub payload: Payload
}
pub struct Dispatcher {
    registry: Arc<Mutex<HashMap<GameEvent, Vec<Arc<dyn Handler>>>>>,
}

pub struct EventLoop {
    pub register: Arc<Mutex<HashMap<GameEvent, Vec<Arc<dyn Handler>>>>>,
    pub events: Arc<Mutex<VecDeque<Event>>>
}

impl EventLoop {
    pub fn new(tiles: Vec<GameEvent>) -> Self {
        EventLoop {
            register: Arc::new(Mutex::new(HashMap::new())),
            events: Arc::new(Mutex::new(VecDeque::new())),
        }
    }

    pub fn register_handler(&mut self, event: GameEvent, handler: Arc<dyn Handler>) {
        let mut registry = self.register.lock().unwrap();
        registry.entry(event).or_insert_with(Vec::new).push(handler);
    }
    pub fn add_event(&mut self, event: GameEvent, payload: Payload) {
        let mut events = self.events.lock().unwrap();
        events.push_back(Event { event, payload });
    }

    pub fn start(&self) {
        let registry = Arc::clone(&self.register);
        loop {
            let event_opt = {
                let mut events = self.events.lock().unwrap();
                events.pop_front()
            };

            if let Some(event) = event_opt {
                let registry = registry.lock().unwrap();
                if let Some(handlers) = registry.get(&event.event) {
                    for handler in handlers {
                        handler.handle(event.event.clone(), event.payload.clone());
                    }
                }
            } else {
                // Sleep for a short duration to prevent busy-waiting
                thread::sleep(std::time::Duration::from_millis(10));
            }

           if (self.events.lock().unwrap().len() > 0) {
               let event = self.events.lock().unwrap().pop_front().unwrap();
               let registry = registry.lock().unwrap();
               if let Some(handlers) = registry.get(&event.event) {
                   for handler in handlers {
                       handler.handle(event.event.clone(), event.payload.clone());
                   }
               }
           }
        }
    }
}

//1. Dodać rejestrację handlerów
//2. Dodać wywołanie handlerów
//3. Dodać obsługę zdarzeń
// https://dev.to/luisccc/learning-by-doing-event-loop-in-rust-hf1