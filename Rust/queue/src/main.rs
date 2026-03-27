use std::collections::VecDeque;

struct Queue {
    items: VecDeque<i32>,
}

impl Queue {
    fn new() -> Self {
        return Queue {
            items: VecDeque::new(),
        };
    }

    fn enqueue(&mut self, value: i32) {
        self.items.push_back(value);
    }

    fn dequeue(&mut self) -> Option<i32> {
        return self.items.pop_front();
    }

    fn peek(&mut self) -> Option<&i32> {
        return self.items.front();
    }
}

fn main() {
    // A queue is a DS that follow the First in, First out principle
    // ( eg.: A line in the groceries store )
    // enqueue, dequeue

    let mut queue = Queue::new();

    queue.enqueue(1);
    queue.enqueue(2);
    queue.enqueue(3);
    queue.enqueue(4);
    queue.enqueue(5);
    queue.enqueue(6);

    println!("Front of the queue: {:?}", queue.peek());

    queue.dequeue();
    queue.dequeue();

    println!("After dequeue (x2), front of the queue: {:?}", queue.peek());
}
