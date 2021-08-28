#[cfg(test)]
mod static_queue {
    use dsa_in_rust::linear::static_queue::*;

    #[test]
    fn len_of_new_queue_is_zero() {
        let sq: SQueue<i64> = SQueue::new();
        assert_eq!(sq.len(), 0);
    }

    #[test]
    fn first_elem_of_empty_queue_is_none() {
        let mut sq: SQueue<i64> = SQueue::new();
        assert_eq!(None, sq.dequeue());
    }

    #[test]
    fn repeated_dequeue_of_empty_queue_is_idempotent() {
        let mut sq: SQueue<i64> = SQueue::new();
        sq.dequeue();
        sq.dequeue();
        sq.dequeue();
        sq.dequeue();
        assert_eq!(None, sq.dequeue());
        assert_eq!(0, sq.len());
    }

    #[test]
    fn repeatedly_adding_and_removing_elements_is_idempotent() {
        let mut sq: SQueue<i64> = SQueue::new();
        for _i in 0..10 {
            sq.enqueue(42);
            sq.enqueue(10);
            assert_eq!(2, sq.len());
            assert_eq!(42, sq.dequeue().unwrap());
            assert_eq!(10, sq.dequeue().unwrap());
            assert_eq!(0, sq.len());
        }
        
        // Attempt to cause an internal inconsistency
        sq.dequeue();

        for _i in 0..10 {
            sq.enqueue(42);
            sq.enqueue(10);
            assert_eq!(2, sq.len());
            assert_eq!(42, sq.dequeue().unwrap());
            assert_eq!(10, sq.dequeue().unwrap());
            assert_eq!(0, sq.len());
        }
    }

    #[test]
    fn enqueue_and_dequeue_many_items() {
        let mut sq: SQueue<i64> = SQueue::new();
        for i in 0..100_000 {
            sq.enqueue(i);
        }

        assert_eq!(100_000, sq.len());

        for i in 0..100_000 {
            assert_eq!(i, sq.dequeue().unwrap());
            assert_eq!((100_000 - i-1) as usize, sq.len());
        }
        assert_eq!(None, sq.dequeue());
    }

    #[test]
    fn peek_empty_queue_is_none() {
        let sq: SQueue<i64> = SQueue::new();
        assert_eq!(None, sq.peek());
    }
    
    #[test]
    fn peek_does_not_change_len() {
        let mut sq: SQueue<i64> = SQueue::new();

        sq.enqueue(1);
        sq.enqueue(2);
        sq.enqueue(3);

        // Correct item is at the front
        assert_eq!(1, *sq.peek().unwrap());
        assert_eq!(3, sq.len());

        // Actually dequeuing after peeking gives the right answer
        assert_eq!(1, sq.dequeue().unwrap());
    }

    #[test]
    fn peek_rear_returns_correct_value() {
        let mut sq: SQueue<i64> = SQueue::new();

        for i in 0..10 {
            sq.enqueue(i);
            assert_eq!(i, *sq.peek_rear().unwrap());
        }

        // Back of the queue does not change as the front dequeues
        for _i in 0..10 {
            assert_eq!(9, *sq.peek_rear().unwrap());
            sq.dequeue();
        }
        assert_eq!(None, sq.peek_rear());
    }
}