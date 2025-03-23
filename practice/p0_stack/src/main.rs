struct Node<T> {
    back: Option<Box<Node<T>>>,
    value: T,
}

struct Stack<T> {
    tail: Option<Box<Node<T>>>,
    size: usize,
}

impl<T> Stack<T> {
    fn new() -> Self {
        Stack {
            tail: None,
            size: 0,
        }
    }

    fn push(&mut self, value: T) {
        // Create a new node.
        // Move the current tail (if any) into the back field of the new node.
        // We use `.take()` to move the ownership of `self.tail` into `back`.
        // This leaves `self.tail` as None, ensuring we don't clone or duplicate the Box.
        let new_node = Box::new(Node {
            value,
            back: self.tail.take(), // Take ownership of the old tail, leaving None behind.
        });

        // Set the new node as the new tail of the stack.
        self.tail = Some(new_node);

        // Increment the size counter.
        self.size += 1;
    }

    fn pop(&mut self) -> Option<T> {
        // We use `.take()` to move the current tail out of the stack.
        // This replaces `self.tail` with None, giving us ownership of the node.
        //
        // If `self.tail` is None (stack is empty), this returns None directly.
        // Otherwise, `.map(...)` will process the value inside `Some(...)`.
        self.tail.take().map(|old_tail| {
            // At this point, we own the node.
            // Set `self.tail` to the previous node in the stack.
            self.tail = old_tail.back;

            // Decrement the size counter, since one element was removed.
            self.size -= 1;

            // Return the value stored in the removed node.
            // `.map()` will wrap this in `Some(...)` as the final result.
            old_tail.value
        })

        // NOTE:
        // We can't return `T` directly, because the stack might be empty.
        // In that case, there’s no value to return at all.
        // Using `Option<T>` allows us to represent two safe cases:
        // - `Some(value)` if a value was popped
        // - `None` if the stack was empty
    }
}

// We use a separate implementation block that requires T to implement Display.
// This allows the values to be printed using the `{}` formatter.
impl<T: std::fmt::Display> Stack<T> {
    fn print_and_clear(&mut self) {
        // Pop and print all elements in LIFO order.
        // `while let` repeatedly runs the loop as long as pop() returns Some(value).
        while let Some(value) = self.pop() {
            println!("{}", value);
        }

        // When the loop ends, the stack is empty.
        // All nodes have been dropped automatically.
        // Rust handles memory deallocation without manual intervention.
    }
}

fn main() {
    let text: &str = "🚀|||^";

    // Create a new stack to hold characters.
    let mut stack: Stack<char> = Stack::new();

    // Push every character in the string into the stack.
    // This includes all characters — no filtering is applied.
    for ch in text.chars() {
        stack.push(ch);
    }

    // Print and clear the stack.
    // Characters are printed in reverse order of insertion (LIFO).
    stack.print_and_clear();

    // At this point, the stack has been cleared.
}

