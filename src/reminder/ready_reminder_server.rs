use std::time::Instant;
use std::collections::BinaryHeap;
use std::cmp::Ordering;

/**
 * An event that will become ready at some time in the future.
 */
#[derive(Eq, PartialEq, Clone, Debug)]
pub struct Entry {
    content: String,
    time: Instant
}

impl Entry {
    pub fn new(content: String, ready_at: Instant) -> Entry {
        Entry {
            content: content,
            time: ready_at
        }
    }
    pub fn get_content(&self) -> &String {
        &self.content
    }
    pub fn get_time(&self) -> &Instant {
        &self.time
    }
}

// partial order by time
impl PartialOrd for Entry {
    /* this is manually changed for builing a min-heap with std BinaryHeap */
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.time.partial_cmp(&self.time)
    }

    fn lt(&self, other: &Self) -> bool {
        self.time > other.time
    }

    fn le(&self, other: &Self) -> bool {
        self.time >= other.time
    }

    fn gt(&self, other: &Self) -> bool {
        self.time < other.time
    }

    fn ge(&self, other: &Self) -> bool {
        self.time <= other.time
    }
}

// total order by time
impl Ord for Entry {
    /* this is manually changed for builing a min-heap with std BinaryHeap */
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

/**
 * A server that stores events that will become ready at some time in the future.
 * Events can be sumbit to the server, and the server can be queried to see if
 * any events are ready.
 */
pub struct ReadyReminderServer {
    entries: BinaryHeap<Entry>
}

impl ReadyReminderServer {
    pub fn new() -> ReadyReminderServer {
        ReadyReminderServer {
            entries: BinaryHeap::new()
        }
    }

    pub fn submitEvent(&mut self, content: String, ready_at: Instant) {
        self.entries.push(Entry::new(content, ready_at));
    }

    pub fn extractEvent(&mut self) -> Option<Entry> {
        let first = self.entries.peek();
        match first {
            Some(entry) => {
                if entry.get_time() <= &Instant::now() {
                    let e = self.entries.pop();
                    return e;
                } else {
                    return None;
                }
            },
            None => {
                return None;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{Instant, Duration};

    #[test]
    fn lt_works() {
        let e1 = Entry::new("Hello World!".to_string(), Instant::now() + Duration::new(1, 0));
        let e2 = Entry::new("Goodbye World!".to_string(), Instant::now() + Duration::new(3, 0));
        assert!(e1 > e2);
    }

    #[test]
    fn cmp_works() {
        let e1 = Entry::new("Hello World!".to_string(), Instant::now() + Duration::new(1, 0));
        let e2 = Entry::new("Goodbye World!".to_string(), Instant::now() + Duration::new(2, 0));
        assert_eq!(e1.cmp(&e2), Ordering::Greater);
    }

    #[test]
    fn heap_works() {
        let mut h = BinaryHeap::new();
        let e1 = Entry::new("Hello World!".to_string(), Instant::now() + Duration::new(1, 0));
        let e2 = Entry::new("Goodbye World!".to_string(), Instant::now() + Duration::new(3, 0));
        h.push(e2);
        h.push(e1);
        assert!(h.peek().unwrap().get_content() == "Hello World!");
    }
}


