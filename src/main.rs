struct CPU {
    pc: u32
}

fn step(cpu: &mut CPU) {
    cpu.pc += 4;
    println!("Current PC: {0}", cpu.pc);
}

fn main() {
    let mut cpu: CPU = CPU { pc: 0 };
    loop {
        step(&mut cpu);
    }
}
