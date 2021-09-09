use Vec;

/// Rust Vectors already implement stack behavior, so 'implementing'
/// a stack just functions as a warmup/refresher on Rust.
///
/// We will implement our own Vector later, but it's quite complicated
/// so it will be tackled at a later point.
pub struct Stack<T> {
    buf: Vec<T>,
}

impl<T> Stack<T> {
    /// Create a new stack
    pub fn new() -> Stack<T> {
        Stack {
            buf: Vec::new()
        }
    }

    /// Returns the number of elements on the stack.
    pub fn len(&self) -> usize {
        return self.buf.len();
    }

    /// Adds a new item to the top of the stack
    pub fn push(&mut self, item: T) {
        self.buf.push(item);
    }

    /// Removes and returns the top item off the stack
    /// or None if the stack is empty
    pub fn pop(&mut self) -> Option<T> {
        self.buf.pop()
    }

    /// Returns an immutable reference to the top element of the stack
    /// or None if the stack is empty
    pub fn peek(&self) -> Option<&T> {
        self.buf.last()
    }

    /// Returns a mutable reference to the top element of the stack
    /// or None if the stack is empty
    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.buf.last_mut()
    }
}


