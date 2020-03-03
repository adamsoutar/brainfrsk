mod tape;
mod instructions;

fn main() {
    let tape = tape::new_tape();
    let instrs = instructions::read_from_file("test.bf").unwrap();

    for i in instrs {
        println!("{:?}", i);
    }
}
