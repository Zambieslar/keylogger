#![allow(dead_code)]
#[derive(Clone)]

pub enum State {
    TvSec,
    TvUsec,
    Type,
    Code,
    Value,
}

impl State {
    pub const STATES: [State; 5] = [
        Self::TvSec,
        Self::TvUsec,
        Self::Type,
        Self::Code,
        Self::Value,
    ];
}

pub struct StateMachine {
    pub state: State,
    pub offset: usize,
    pub line: i32,
    pub buf: Vec<u8>,
}

#[derive(Debug)]
pub struct Epoch {
    pub tv_sec: Vec<u8>,
    pub tv_usec: Vec<u8>,
}

#[derive(Debug)]
pub struct Event {
    pub t_stamp: Epoch,
    pub r#type: Vec<u8>,
    pub code: Vec<u8>,
    pub value: Vec<u8>,
}

impl Epoch {
    fn new() -> Self {
        Self {
            tv_sec: Vec::<u8>::new(),
            tv_usec: Vec::<u8>::new(),
        }
    }
}

impl Event {
    pub fn new() -> Self {
        Self {
            t_stamp: Epoch::new(),
            r#type: Vec::<u8>::new(),
            code: Vec::<u8>::new(),
            value: Vec::<u8>::new(),
        }
    }
}
