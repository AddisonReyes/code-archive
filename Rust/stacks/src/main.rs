struct Stack {
    items: Vec<i32>,
}

impl Stack {
    fn new() -> Self {
        return Stack { items: Vec::new() };
    }

    // push an element into the stack
    fn push(&mut self, value: i32) {
        self.items.push(value);
    }

    // pop a element from the top of the stack
    fn pop(&mut self) -> Option<i32> {
        return self.items.pop();
    }

    // peek at the top element without removing it
    fn peek(&self) -> Option<&i32> {
        return self.items.last();
    }
}

fn main() {
    // A Stack is a DS that follow the Last in, First out principle
    // LIFO ( eg.: A stack of plates )
    // push and pop

    let mut stack: Stack = Stack::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);
    stack.push(4);
    stack.push(5);

    println!("Top of the stack: {:?}", stack.peek());

    stack.pop();
    stack.pop();

    println!("After popping, top of the stack: {:?}", stack.peek());
}
