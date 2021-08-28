// This module is not necessary, but it makes the test output easier to read
// by prepending the test name with the module name.
#[cfg(test)]
mod stack {
    use dsa_in_rust::linear::stack::*;

    #[test]
    fn len_of_new_stack_is_zero() {
        let stack: Stack<i64> = Stack::new();
        assert_eq!(0, stack.len());
    }

    #[test]
    fn pushing_then_popping_should_be_idempotent() {
        let mut stack: Stack<i64> = Stack::new();
        for i in 0..10 {
            stack.push(i);
        }
        assert_eq!(9, *stack.peek().unwrap());
        assert_eq!(10, stack.len());

        for _i in 0..10 {
            stack.pop();
        }

        assert_eq!(0, stack.len());
    }

    #[test]
    fn popping_empty_stack_is_none() {
        let mut stack: Stack<i64> = Stack::new();
        stack.push(100);
        assert_eq!(100, stack.pop().unwrap());

        assert_eq!(None, stack.pop());
    }

    #[test]
    fn peeking_does_not_change_size() {
        let mut stack: Stack<i64> = Stack::new();
        stack.push(1);
        stack.push(2);
        stack.push(3);

        assert_eq!(3, stack.len());
        assert_eq!(3, *stack.peek().unwrap());
        assert_eq!(3, stack.len());

        assert_eq!(3, stack.pop().unwrap());
        assert_eq!(2, stack.pop().unwrap());
        assert_eq!(1, stack.pop().unwrap());
    }
}
