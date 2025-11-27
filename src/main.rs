use crate::event::Event;

pub mod client;
pub mod error;
pub mod event;
pub mod state;

pub type ClientId = u16;

fn main() {
    let transactions: Vec<Event> = vec![];
}
