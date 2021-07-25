pub fn mock_rand(n: u8) -> f32 {
    let base: u32 = 0b0_01111110_00000000000000000000000;
    let large_n = (n as u32) << 15;
    let f32_bits = base | large_n;
    let m = f32::from_bits(f32_bits);
    2.0 * (m - 0.5)
}

// chip-8 manual => Rust in Action, page 164
// this manual defines the structure of an opcode in a chip-8 architecture
pub struct CPU {
    pub current_operation: u16,
    pub registers: [u8; 2],
}

impl CPU {
    fn read_opcode(&self) -> u16 {
        self.current_operation
    }

    fn add_xy(&mut self, x: u8, y: u8) {
        self.registers[x as usize] += self.registers[y as usize];
    }

    pub fn run(&mut self) {
        // loop {
        let opcode = self.read_opcode();

        let c = ((opcode & 0xF000) >> 12) as u8; // extract first nibble. assuming opcode is 0x8234, c = (0x8)
        let x = ((opcode & 0x0F00) >> 8) as u8; // x = 0x2
        let y = ((opcode & 0x00F0) >> 4) as u8;
        let d = ((opcode & 0x000F) >> 0) as u8;

        match (c, x, y, d) {
            (0x8, _, _, 0x4) => self.add_xy(x, y),
            _ => todo!("opcode {:04x}", opcode),
        }
    }
    // }
}
