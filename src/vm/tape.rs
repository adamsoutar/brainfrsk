use std::convert::TryInto;

pub struct Tape {
    values: Vec<u8>,
    pointer: usize
}

impl Tape {
    pub fn right(&mut self) {
        self.pointer += 1;
        if self.pointer == self.values.len() {
            self.values.push(0)
        }
    }

    pub fn left (&mut self) {
        if self.pointer == 0 {
            // Put a zero on the start
            let mut a = vec![0];
            a.append(&mut self.values);
            self.values = a;
            return
        }

        self.pointer -= 1;
    }

    pub fn get_value (&self) -> u8 {
        let idx: usize = self.pointer.try_into().unwrap();
        self.values[idx]
    }

    pub fn set_value(&mut self, val: u8) {
        let idx: usize = self.pointer.try_into().unwrap();
        self.values[idx] = val;
    }
}

pub fn new_tape () -> Tape {
    return Tape {
        values: vec![0],
        pointer: 0
    }
}
