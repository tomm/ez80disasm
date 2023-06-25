use ez80::*;

fn main() {
    let filename = match std::env::args().skip(1).skip_while(|a| a.starts_with("-")).next() {
        Some(f) => f,
        _ => {
            eprintln!("Usage: ez80disasm [-z80] <input file>");
            std::process::exit(0);
        }
    };

    let z80_mode = std::env::args().any(|a| a == "-z80");

    let mut machine = PlainMachine::new();
    let mut cpu = Cpu::new_ez80();

    let code = std::fs::read(filename).unwrap();

    for (i, e) in code.iter().enumerate() {
        machine.poke(i as u32, *e);
    }

    println!("\t.org 0");
    println!("\t.assume adl={}", if z80_mode { 0 } else { 1 });

    let dis = ez80::disassembler::disassemble(&mut machine, &mut cpu, Some(!z80_mode), 0, code.len() as u32);

    for line in &dis {
        // invalid opcode. convert to .db
        if line.asm == "NONINOP" {
            print!("\t.db ");
            for byte in &line.bytes {
                print!("{:02x}h,", byte);
            }
        } else {
            print!("\t{:20}", line.asm);
        }
        print!("; {:06x}h -", line.loc);
        for byte in &line.bytes {
            print!(" {:02x}", byte);
        }
        println!();
    }
}
