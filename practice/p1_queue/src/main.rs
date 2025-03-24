use std::fmt;
use std::mem::MaybeUninit;

struct Queue<T, const N: usize> {
    front: usize,
    rear: usize,
    count: usize,
    buffer: [MaybeUninit<T>; N],
}

impl<T, const N: usize> Queue<T, N> {
    fn new() -> Self {
        /*
            SAFETY: We are creating an uninitialized array of `MaybeUninit<T>`.
            `MaybeUninit<T>` is a special wrapper that allows us to allocate memory
            for a value of type `T` without actually initializing it.

            Rust does not let us create `[MaybeUninit<T>; N]` directly unless `T: Copy`,
            so we construct a `MaybeUninit<[MaybeUninit<T>; N]>` (an uninitialized array),
            and then use `.assume_init()` to treat it as fully initialized.

            This is safe *only* because `MaybeUninit<T>` does not require initialization.
            We will manually initialize each element later using `.write()` before reading.

            IMPORTANT: We must never read from an element unless we’ve written to it first,
            and we must ensure we `drop` each initialized element exactly once.
        */
        let buffer: [MaybeUninit<T>; N] = unsafe { MaybeUninit::uninit().assume_init() };

        Queue {
            front: 0,
            rear: 0,
            count: 0,
            buffer,
        }
    }

    fn enqueue(&mut self, value: T) -> Result<(), &str> {
        if self.is_full() {
            return Err("Queue is full");
        }

        self.buffer[self.rear].write(value);

        self.rear = (self.rear + 1) % N;
        self.count += 1;
        Ok(())
    }

    fn dequeue(&mut self) -> Option<T> {
        if self.is_empty() {
            None
        } else {
            let value = unsafe { self.buffer[self.front].assume_init_read() };
            self.front = (self.front + 1) % N;
            self.count -= 1;
            Some(value)
        }
    }

    fn is_empty(&self) -> bool {
        self.count == 0
    }

    fn is_full(&self) -> bool {
        self.count == N
    }
}

/*
    This `impl` block uses a `where` clause to restrict usage of `print()`
    to types that implement the `Display` trait.

    The reason is that `println!("{}", value)` requires that `T` implements
    `std::fmt::Display` — otherwise the compiler will produce an error.

    The `where T: Display` constraint ensures this method is only available
    for types that can be printed using `{}`.

    Other common `where` constraints in Rust include:
    - `where T: Clone + Debug` → T must implement both traits
    - `where T: 'static`       → T must live for the entire program lifetime

    This approach makes your code more robust and type-safe.
*/
impl<T: std::fmt::Display, const N: usize> Queue<T, N> {
    fn print(&self) {
        let mut index = self.front;

        print!("[");
        for i in 0..self.count {
            let val = unsafe { self.buffer[index].assume_init_ref() };
            if i > 0 {
                print!(", ");
            }
            print!("{}", val);
            index = (index + 1) % N;
        }
        println!("]");
    }
}

impl<T: fmt::Debug, const N: usize> fmt::Debug for Queue<T, N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut list = f.debug_struct("Queue");
        list.field("front", &self.front)
            .field("rear", &self.rear)
            .field("count", &self.count);

        let mut index = self.front;
        let mut items = vec![];

        for _ in 0..self.count {
            let val = unsafe { self.buffer[index].assume_init_ref() };
            items.push(val);
            index = (index + 1) % N;
        }

        list.field("contents", &items).finish()
    }
}
fn main() {
    const SIZE: usize = 5;
    let array: [i32; SIZE] = [10, 11, 12, 13, 14];
    let mut queue = Queue::<i32, SIZE>::new();

    for i in 0..SIZE {
        queue.enqueue(array[i]).unwrap();
    }

    for _ in 0..2 {
        match queue.dequeue() {
            Some(val) => println!("Dequeued: {}", val),
            None => println!("Queue is empty!"),
        }
    }

    match queue.enqueue(42) {
        Ok(()) => (),
        Err(err) => println!("Error: {}", err),
    }

    queue.print();

    dbg!(&queue);
}
