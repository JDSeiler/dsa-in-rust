use std::mem;

/// A "static" (Vec based) queue. In contrast to a reference based
/// queue (perhaps using a linked list as the backing structure).
/// 
/// Rust does not support variable sized arrays (the size MUST be known
/// at compile time), so until I implement my own ArrayList this is the
/// best we can do.
pub struct SQueue<T : Default> {
    buf: Vec<T>,
    len: usize,
    front: usize,
    rear: usize
}

impl<T : Default> SQueue<T> {
    /// Constructs a new, empty, SQueue
    pub fn new() -> SQueue<T> {
        SQueue {
            buf: Vec::new(),
            len: 0,
            front: 0,
            rear: 0
        }
    }

    /// Adds a new item to the rear of this SQueue
    pub fn enqueue(&mut self, item: T) {
        self.buf.push(item);

        self.rear = self.buf.len() - 1;
        self.len += 1;
    }

    /// Removes and returns the element from the front of the SQueue, or None
    /// if the SQueue is empty.
    pub fn dequeue(&mut self) -> Option<T> {
        if let Some(v_ref) = self.buf.get_mut(self.front) {
            let v = mem::take(v_ref);
            /* 
            If we're removing the last element of the queue, reset it to its 
            default state to prevent the backing Vec from growing in size forever.
            
            There are probably more efficient choices here, but the simple
            approach of truncating and dropping the entire Vec is probably fine
            for now.

            In fact, other approaches would be incorrect because the Vec
            would contain junk default values that `enqueue` would not handle.
            The correctness of this approach relies on the fact that after we
            dequeue an element from the Vec, we never look at indices *before*
            that element ever again until the SQueue is reset.
            */
            if self.front == self.rear {
                self.buf.clear();
                self.buf.shrink_to_fit();
                
                self.front = 0;
                self.rear = 0;
                self.len = 0;
            } else {
                self.front += 1;
                self.len -= 1;
            }

            Some(v)
        } else {
            None
        }
    }

    /// Returns a reference to the element at the front of the SQueue, but does
    /// not remove it.
    pub fn peek(&self) -> Option<&T> {
        self.buf.get(self.front)
    }

    /// Returns a reference to the element at the rear of the SQueue, but does
    /// not remove it.
    pub fn peek_rear(&self) -> Option<&T> {
        self.buf.get(self.rear)
    }

    /// Returns the number of elements in the SQueue
    pub fn len(&self) -> usize {
        self.len
    }
}