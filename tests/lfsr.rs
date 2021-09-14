#[cfg(test)]
mod lfsr {
    use dsa_in_rust::misc::lfsr::*;

    /*
    Testing the output bits is trivial for the first 32 of them
    It'd be better to check state and check generated bits I think
    */

    #[test]
    fn next_bit_correct() {
        let mut l = LFSR::new();
        let expected = [1, 1];
        println!("{:b}", l.get_state());
        for i in 0..expected.len() {
            assert_eq!(l.next_bit(), expected[i]);
            println!("{:032b}", l.get_state());
        }
    }
}
