use crate::{definitions::*, traits::*};

impl Machine for StateMachine {
    fn state(&self) -> State {
        self.state.clone()
    }

    fn offset(&self) -> usize {
        self.offset.clone()
    }

    fn buf(&self) -> Vec<u8> {
        self.buf.clone()
    }

    fn new() -> Self {
        Self {
            state: (State::TvSec),
            offset: 0,
            line: 0,
            buf: Vec::<u8>::new(),
        }
    }

    fn next(&mut self) {
        match self.state() {
            state => {
                self.state = State::STATES[state as usize + 1].clone();
            }
        }
    }

    fn reverse(&mut self) {
        match self.state() {
            state => {
                self.state = State::STATES[state as usize - 1].clone();
            }
        }
    }

    fn increment(&mut self) {
        self.offset += 1;
    }

    fn decrement(&mut self) {
        self.offset -= 1;
    }

    fn run(&mut self, data: &[u8], mut input_event: Event) -> Event {
        loop {
            match self.offset() {
                8 => {
                    //                    self.buf.reverse();
                    input_event.t_stamp.tv_sec = self.buf();
                    self.buf.clear();
                }
                16 => {
                    //                   self.buf.reverse();
                    self.buf.push(data[self.offset()]);
                    input_event.t_stamp.tv_usec = self.buf();
                    self.buf.clear();
                }
                32 => {
                    self.buf.push(data[self.offset()]);
                    input_event.r#type = self.buf();
                    self.buf.clear();
                }
                48 => {
                    self.buf.push(data[self.offset()]);
                    input_event.code = self.buf();
                    self.buf.clear();
                }
                80 => {
                    self.buf.push(data[self.offset()]);
                    input_event.value = self.buf();
                    return input_event;
                }
                _ => {
                    self.buf.push(data[self.offset()]);
                }
            }
            self.increment();
        }
    }
}
