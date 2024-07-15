use crate::{definitions::*, statemachine::*};

pub trait Machine {
    fn state(&self) -> State;
    fn offset(&self) -> usize;
    fn buf(&self) -> Vec<u8>;
    fn new() -> Self;
    fn next(&mut self);
    fn reverse(&mut self);
    fn run(&mut self, data: &[u8], input_event: Event) -> Event;
    fn increment(&mut self);
    fn decrement(&mut self);
}
