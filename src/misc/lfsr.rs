pub struct LFSR {
    state: u32,
    taps: [u32; 6]
}

impl LFSR {
    // Maximum length tap positions from:
    // http://users.ece.cmu.edu/~koopman/lfsr/index.html

    pub fn new() -> LFSR {
        LFSR {
            state: 0xdeadbeef,
            taps: [0, 25, 27, 29, 30, 31]
        }
    }

    pub fn new_with_state(s: u32) -> LFSR {
        LFSR {
            state: s,
            taps: [0, 25, 27, 29, 30, 31]
        }
    }

    pub fn get_state(&self) -> u32 {
        self.state
    }

    pub fn rand(&mut self) -> u32 {
        let mut num: u32 = 0;
        for i in 0..32 {
            let ith_bit = self.next_bit();
            num |= ith_bit << i
        }
        num
    }

    pub fn next_bit(&mut self) -> u32 {
        let output_bit = self.state & 1;
        let new_bit = self.xor_taps();
        self.cycle(new_bit);

        output_bit
    }

    fn xor_taps(&self) -> u32 {
        let mut acc: u32  = self.nth(self.taps[0]);
        println!("{:b}", acc);
        for i in 1..self.taps.len() {
            acc ^= self.nth(self.taps[i]);
            println!("{:b}", acc);
        }
        acc
    }

    fn nth(&self, n: u32) -> u32 {
        (self.state >> n) & 1
    }

    fn cycle(&mut self, next: u32) {
        self.state = self.state >> 1 | (next << 31);
    }
}

impl Default for LFSR {
    fn default() -> Self {
        Self::new()
    }
}
