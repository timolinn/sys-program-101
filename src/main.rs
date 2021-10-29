/// IMPLEMENTING A CPU
/// CPU related terms:
/// - An intricsic operation (op) refers to procedures supported natively by the system
/// - The registers are containers of data the CPU directly accesses
/// - an opcode is a number that maps to an operation

#[allow(dead_code)]
mod lib;

use lib::sysproglib;

fn main() {
    // BOOTING UP ///

    // 1. Initialize cpu
    let mut cpu = sysproglib::CPU {
        position_in_memory: 0,
        registers: [0; 16],
        memory: [0; 4096],
        stack: [0; 16],
        stack_pointer: 0,
    };

    cpu.registers[0] = 5;
    cpu.registers[1] = 10;

    cpu.memory[0x000] = 0x21;
    cpu.memory[0x001] = 0x00;
    cpu.memory[0x002] = 0x21;
    cpu.memory[0x003] = 0x00;
    cpu.memory[0x100] = 0x80;
    cpu.memory[0x101] = 0x14;
    cpu.memory[0x102] = 0x80;
    cpu.memory[0x103] = 0x14;
    cpu.memory[0x104] = 0x00;
    cpu.memory[0x105] = 0xEE;

    cpu.run();

    assert_eq!(cpu.registers[0], 45);
    println!("5 + (10 * 2) + (10 * 2) = {}", cpu.registers[0]);
}

#[cfg(test)]
mod tests {
    #[test]
    fn extract_chip_variables_from_opcode() {
        let opcode: u16 = 0x71E4; // [74] = high byte, [7] = high nibble, [4] = low nibble. [E4] = low byte, [E] = high nibble, [4] = low nibble

        // 0x71E4 = 111000111100100
        // 0xF000 = 1111000000000000
        let c = (opcode & 0xF000) >> 12; // high byte, high nibble - opcode group
        let x = (opcode & 0x0F00) >> 8; // high byte, low nibble - register
        let y = (opcode & 0x00F0) >> 4; // low byte, high nibble - register
        let d = (opcode & 0x000F) >> 0; // low byte, low nibble - sub-group opcode

        assert_eq!(c, 0x7);
        assert_eq!(x, 0x1);
        assert_eq!(y, 0xE);
        assert_eq!(d, 0x4);

        let nnn = opcode & 0x0FFF; // high byte, low nibble and low byte, both nibbles - memory address
        let kk = opcode & 0x00FF; // low byte, both nibbles - integer
        assert_eq!(nnn, 0x1E4);
        assert_eq!(kk, 0xE4);
    }
}
