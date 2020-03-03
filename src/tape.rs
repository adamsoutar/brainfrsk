pub struct Tape {
    values: Vec<u8>,
    pointer: i32
}

impl Tape {
    fn right(mut self, amnt: i32) {
        self.pointer += amnt
    }

    fn left (mut self, amnt: i32) {
        self.pointer -= amnt
    }
}

pub fn new_tape () -> Tape {
    return Tape {
        values: vec![0],
        pointer: 0
    }
}
