mod vm;
mod instructions;

fn main() {
    let instrs = instructions::read_from_file("test.bf");

    if let Ok(i) = instrs {
        vm::run_for_instructions(i);
    } else {
        println!("Failed to lex 'test.bf'
{:?}", instrs.err().unwrap())
    }
}
