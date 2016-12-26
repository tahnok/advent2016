use std::io;
use std::io::Read;
use std::str::FromStr;

fn main() {
    let mut input = String::new();
    let _ = io::stdin().read_to_string(&mut input);

    let mut machine = Machine::new();
    machine.load(&input);
    machine.run();
    println!("{}", machine.a);
}

pub struct Machine {
    a: isize,
    b: isize,
    c: isize,
    d: isize,
    instructions: Vec<Instruction>,
    ip: usize,
}

impl Machine {
    fn new() -> Machine {
        Machine{
            a: 0,
            b: 0,
            c: 0,
            d: 0,
            instructions: vec![],
            ip: 0,
        }
    }

    fn load(&mut self, input: &str) {
        self.instructions = input.trim().lines().map(|x| x.trim().parse().unwrap()).collect()
    }

    fn step(&mut self) {
        let instruction = self.instructions[self.ip];
        match instruction {
            Instruction::INC(reg) => self.increment(&reg),
            Instruction::DEC(reg) => self.decrement(&reg),
            Instruction::CPYL(val, reg) => self.copy_literal(val, &reg),
            Instruction::CPYR(src, dst) => self.copy_register(&src, &dst),
            Instruction::JNZR(reg, jmp) => self.jump_non_zero_register(&reg, jmp),
            Instruction::JNZL(val, jmp) => self.jump_non_zero_literal(val, jmp),
        }
    }

    fn increment(&mut self, register: &Register) {
        match register {
            &Register::A => self.a += 1,
            &Register::B => self.b += 1,
            &Register::C => self.c += 1,
            &Register::D => self.d += 1,
        }
        self.ip += 1;
    }

    fn decrement(&mut self, register: &Register) {
        match register {
            &Register::A => self.a -= 1,
            &Register::B => self.b -= 1,
            &Register::C => self.c -= 1,
            &Register::D => self.d -= 1,
        }
        self.ip += 1;
    }

    fn copy_literal(&mut self, value: isize, register: &Register) {
        match register {
            &Register::A => self.a = value,
            &Register::B => self.b = value,
            &Register::C => self.c = value,
            &Register::D => self.d = value,
        }
        self.ip += 1;
    }

    fn copy_register(&mut self, src: &Register, dst: &Register) {
        let value = self.fetch(src);
        match dst {
            &Register::A => self.a = value,
            &Register::B => self.b = value,
            &Register::C => self.c = value,
            &Register::D => self.d = value,
        }
        self.ip += 1;

    }

    fn jump_non_zero_register(&mut self, register: &Register, jump: isize) {
        if self.fetch(register) != 0 {
            let new = (self.ip as isize) + jump;
            if new < 0 {
                self.ip = 0;
            } else {
                self.ip = new as usize;
            }
        } else {
            self.ip += 1;
        }
    }

    fn jump_non_zero_literal(&mut self, value: isize, jump: isize) {
        if value != 0 {
            let new = (self.ip as isize) + jump;
            if new < 0 {
                self.ip = 0;
            } else {
                self.ip = new as usize;
            }
        } else {
            self.ip += 1;
        }
    }


    fn fetch(&self, register: &Register) -> isize {
        match register {
            &Register::A => self.a,
            &Register::B => self.b,
            &Register::C => self.c,
            &Register::D => self.d,
        }
    }

    fn run(&mut self) {
        while self.ip < self.instructions.len() {
            self.step();
        }
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Register {
    A,
    B,
    C,
    D
}

#[derive(Debug)]
pub struct RegisterErr;

impl FromStr for Register {
    type Err = RegisterErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "a" => Ok(Register::A),
            "b" => Ok(Register::B),
            "c" => Ok(Register::C),
            "d" => Ok(Register::D),
            _ => Err(RegisterErr),
        }
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Instruction {
    INC(Register),
    DEC(Register),
    JNZR(Register, isize),
    JNZL(isize, isize),
    CPYL(isize, Register),
    CPYR(Register, Register),
}


#[derive(Debug)]
pub struct InstructionError;

impl FromStr for Instruction {
    type Err = InstructionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split_whitespace();
        match parts.next().unwrap() {
            "inc" => Ok(Instruction::INC(parts.next().unwrap().parse().unwrap())),
            "dec" => Ok(Instruction::DEC(parts.next().unwrap().parse().unwrap())),
            "jnz" => {
                let arg1 = parts.next().unwrap();
                let jump: isize = parts.next().unwrap().parse().unwrap();
                let maybe_reg: Result<Register, RegisterErr> = arg1.parse();
                match maybe_reg {
                    Ok(reg) => Ok(Instruction::JNZR(reg, jump)),
                    Err(_) => Ok(Instruction::JNZL(arg1.parse().unwrap(), jump)),
                }
            },
            "cpy" => {
                let arg1 = parts.next().unwrap();
                let dest: Register = parts.next().unwrap().parse().unwrap();
                let maybe_reg: Result<Register, RegisterErr> = arg1.parse();
                match maybe_reg {
                    Ok(reg) => Ok(Instruction::CPYR(reg, dest)),
                    Err(_) => Ok(Instruction::CPYL(arg1.parse().unwrap(), dest)),
                }
            },
            _ => Err(InstructionError)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_parses_increments() {
        assert_eq!(Instruction::INC(Register::A), "inc a".parse().unwrap());
    }

    #[test]
    fn it_parses_decrement() {
        assert_eq!(Instruction::DEC(Register::A), "dec a".parse().unwrap());
    }

    #[test]
    fn it_parses_jnz_register() {
        assert_eq!(Instruction::JNZR(Register::B, -1), "jnz b -1".parse().unwrap());
    }

    #[test]
    fn it_parses_jnz_literal() {
        assert_eq!(Instruction::JNZL(10, -1), "jnz 10 -1".parse().unwrap());
    }

    #[test]
    fn it_parses_cpy_literal() {
        assert_eq!(Instruction::CPYL(41, Register::C), "cpy 41 c".parse().unwrap());
    }

    #[test]
    fn it_parses_cpy_register() {
        assert_eq!(Instruction::CPYR(Register::A, Register::D), "cpy a d".parse().unwrap());
    }

    #[test]
    fn machine_evals_increment_instruction() {
        let mut machine = Machine::new();
        machine.load("inc a");
        machine.step();
        assert_eq!(machine.a, 1);
    }

    #[test]
    fn machine_evals_decrement_instruction() {
        let mut machine = Machine::new();
        machine.load("dec b");
        machine.step();
        assert_eq!(machine.b, -1);
    }

    #[test]
    fn machine_evals_cpyl() {
        let mut machine = Machine::new();
        machine.load("cpy -2 c");
        machine.step();
        assert_eq!(machine.c, -2);
    }

    #[test]
    fn machine_evals_cpyr() {
        let mut machine = Machine::new();
        machine.load("cpy -2 c\ncpy c d");
        machine.step();
        machine.step();
        assert_eq!(machine.d, -2);
    }

    #[test]
    fn machine_evals_jnz_zero_register() {
        let mut machine = Machine::new();
        machine.load("jnz a 4");
        machine.step();
        assert_eq!(machine.ip, 1);
    }

    #[test]
    fn machine_evals_jnz_zero_literal() {
        let mut machine = Machine::new();
        machine.load("jnz 0 4");
        machine.step();
        assert_eq!(machine.ip, 1);
    }

    #[test]
    fn machine_evals_jnz_non_zero_forward() {
        let mut machine = Machine::new();
        machine.load("inc a\njnz a 4");
        machine.step();
        machine.step();
        assert_eq!(machine.ip, 5);
    }

    #[test]
    fn machine_evals_jnz_non_zero_backwards() {
        let mut machine = Machine::new();
        machine.load("inc a\njnz a -4");
        machine.step();
        machine.step();
        assert_eq!(machine.ip, 0);
    }

    #[test]
    fn machine_runs_until_stop() {
        let mut machine = Machine::new();
        machine.load("cpy 41 a
        inc a
        inc a
        dec a
        jnz a 2
        dec a");
        machine.run();
        assert_eq!(machine.a, 42);
    }
}
