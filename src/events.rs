use rand::Rng;
use std::{iter::FilterMap, slice::Iter};

#[derive(Clone, Debug)]
pub enum Action {
    INC,
    DEC,
    SET,
}

#[derive(Clone, Debug)]
pub struct MessageEvent {
    pub id: usize,
    pub from: usize,
    pub to: usize,
    pub action: Action,
    pub value: f64,
    pub message: String,
}

#[derive(Debug)]
pub enum Event {
    /// Used for communication between components
    Message(MessageEvent),
}
impl Event {
    pub fn id(&self) -> usize {
        match self {
            Event::Message(e) => e.id,
        }
    }
}
#[derive(Debug)]
pub struct Events(pub Vec<Event>);

impl Events {
    pub fn remove(&mut self, to_remove: Vec<usize>) {
        self.0.retain(|x| !to_remove.contains(&x.id()));
    }
    pub fn receive_message_events(
        &mut self,
        receiver_id: usize,
    ) -> FilterMap<Iter<'_, Event>, impl Fn(&Event) -> Option<MessageEvent>> {
        let x = |receiver_id: usize| {
            move |event: &Event| match event {
                Event::Message(y) => {
                    if y.to == receiver_id {
                        return Some(y.clone());
                    }
                    None
                }
            }
        };
        self.0.iter().filter_map(x(receiver_id))
    }
    pub fn add_message_event(
        &mut self,
        from: usize,
        to: usize,
        action: Action,
        value: f64,
        message: String,
    ) {
        self.0.push(Event::Message(MessageEvent {
            id: rand::thread_rng().gen(),
            from,
            to,
            action,
            value,
            message,
        }));
    }
}
