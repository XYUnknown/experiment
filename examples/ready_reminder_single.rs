use experiment::reminder::ready_reminder_server::{Entry, ReadyReminderServer};
use std::{thread};
use std::time::{Instant, Duration};

fn main() {
    let mut r = ReadyReminderServer::new();
    r.submitEvent("Goodbye World!".to_string(), Instant::now() + Duration::new(3, 0));
    r.submitEvent("Hellow World!".to_string(), Instant::now() + Duration::new(1, 0));
    println!("The first event is: {:?}", r.exactEvent());
    thread::sleep(Duration::new(4, 0));
    println!("The first event is: {:?}", r.exactEvent());
    println!("The first event is: {:?}", r.exactEvent());
}