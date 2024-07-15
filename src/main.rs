use std::{fs::File, io::Read};

mod definitions;
mod statemachine;
mod traits;

use traits::Machine;

use crate::definitions::*;

fn main() {
    loop {
        let mut machine = StateMachine::new();
        let input_event = Event::new();
        let mut k_events =
            File::open("/dev/input/by-path/pci-0000:11:00.4-usbv2-0:2:1.0-event-kbd").unwrap();
        let mut buf: [u8; 81] = [0; 81];
        k_events.read(&mut buf).unwrap();
        let event = machine.run(&buf, input_event);
        println!(
            "Epoch:{:?}.{:?}\n Type:{:?}\n Code:{:?}\n Value:{:?}\n",
            u64::from_le_bytes(event.t_stamp.tv_sec.as_slice().try_into().unwrap()),
            u64::from_le_bytes(event.t_stamp.tv_usec.as_slice().try_into().unwrap()),
            event.r#type,
            event.code,
            event.value,
        );
    }
}
