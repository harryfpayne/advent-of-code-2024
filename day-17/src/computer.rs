use std::ops::BitXor;

#[derive(Debug)]
pub struct ComputerState {
    pub registers: [i64; 3],
    pub instructions: Vec<u8>,
    pub instruction_pointer: usize,
    pub output: Vec<u8>,
}

const A: usize = 0;
const B: usize = 1;
const C: usize = 2;

impl ComputerState {
    pub fn new(registers: [i64; 3], instructions: Vec<u8>) -> Self {
        ComputerState{
            registers,
            instructions,
            instruction_pointer: 0,
            output: vec![],
        }
    }

    pub fn step(&mut self) -> bool {
        if self.instruction_pointer >= self.instructions.len() {
            return false;
        }
        let [op, operand] = self.instructions[self.instruction_pointer..=self.instruction_pointer+1] else {
            return false;
        };

        let mut should_inc = true;
        match op {
            0 => self.adv(operand), // A = A / 2^$o
            1 => self.bxl(operand), // B = B xor o
            2 => self.bst(operand), // B = $o % 8
            3 => { // jump to o if A is 0
                if self.jnz(operand) {
                    should_inc = false;
                }
            },
            4 => self.bxc(operand), // B = B xor C
            5 => self.out(operand), // out <- $o % 8
            6 => self.bdv(operand), // B = A / 2^$o
            7 => self.cdv(operand), // C = A / 2^$o
            _ => panic!("Unknown op: {}", op),
        }

        if should_inc {
            self.instruction_pointer += 2
        }
        if self.instruction_pointer >= self.instructions.len() {
            return false;
        }
        true
    }

    fn adv(&mut self, operand: u8) {
        let num = self.registers[A];
        let c = self.get_combo_operand(operand);
        let den = 2i64.pow(c as u32);
        let res = num / den;
        self.registers[A] = res;
    }

    fn bxl(&mut self, operand: u8) {
        let b = self.registers[B];
        self.registers[B] = b.bitxor(operand as i64);
    }

    fn bst(&mut self, operand: u8) {
        let c = self.get_combo_operand(operand);
        self.registers[B] = (c % 8) as i64
    }

    fn jnz(&mut self, operand: u8) -> bool {
        if self.registers[A] != 0 {
            self.instruction_pointer = operand as usize;
            return true;
        }
        false
    }

    fn bxc(&mut self, operand: u8) {
        self.registers[B] = self.registers[B].bitxor(self.registers[C]);
    }

    fn out(&mut self, operand: u8) {
        let o = self.get_combo_operand(operand) % 8;
        self.output.push(o);
    }

    fn bdv(&mut self, operand: u8) {
        let num = self.registers[A];
        let c = self.get_combo_operand(operand);
        let den = 2i64.pow(c as u32);
        let res = num / den;
        self.registers[B] = res;
    }

    fn cdv(&mut self, operand: u8) {
        let num = self.registers[A];
        let c = self.get_combo_operand(operand);
        let den = 2i64.pow(c as u32);
        let res = num / den;
        self.registers[C] = res;
    }

    fn get_combo_operand(&self, operand: u8) -> u8 {
        match operand {
            0..=3 => operand,
            4..=6 => self.registers[(operand - 4) as usize] as u8,
            _ => panic!("invalid combo operand: {}", operand),
        }
    }

    pub fn get_output(&self) -> &[u8] {
        &self.output
    }
}