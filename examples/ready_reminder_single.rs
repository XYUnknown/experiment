use experiment::reminder::ready_reminder_server::{Entry, ReadyReminderServer};
use std::{thread};
use std::time::{Instant, Duration};

/**
 * A program that specifies times at which some events will become ready, then
 * checks to see if those events are ready.
 * 
 */
fn main() {
    let mut r = ReadyReminderServer::new();
    r.submitEvent("Goodbye World!".to_string(), Instant::now() + Duration::new(3, 0));
    r.submitEvent("Hellow World!".to_string(), Instant::now() + Duration::new(1, 0));
    println!("The first event is: {:?}", r.extractEvent());
    thread::sleep(Duration::new(4, 0));
    println!("The first event is: {:?}", r.extractEvent());
    println!("The first event is: {:?}", r.extractEvent());
}