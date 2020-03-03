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

    // These functions simulate integer over/underflow
    pub fn increment (&mut self) {
        let val = self.values[self.pointer];
        if val == 255 {
            self.values[self.pointer] = 0;
        } else {
            self.values[self.pointer] += 1;
        }
    }
    pub fn decrement (&mut self) {
        let val = self.values[self.pointer];
        if val == 0 {
            self.values[self.pointer] = 255;
        } else {
            self.values[self.pointer] -= 1;
        }
    }

    pub fn get_value (&self) -> u8 {
        self.values[self.pointer]
    }

    pub fn set_value(&mut self, val: u8) {
        self.values[self.pointer] = val;
    }
}

pub fn new_tape () -> Tape {
    return Tape {
        values: vec![0],
        pointer: 0
    }
}
