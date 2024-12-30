pub mod template;

// Use this file to add helper functions and additional modules.

pub struct Intcode {
    program: Vec<usize>,
    memory: Vec<usize>,
    pointer: usize,
    noun: Option<usize>,
    verb: Option<usize>,
}

impl Intcode {
    pub fn new(state: &str) -> Intcode {
        Intcode {
            program: state
                .split(",")
                .map(|int| int.trim().parse::<usize>().unwrap())
                .collect(),
            memory: vec![0],
            pointer: 0,
            noun: None,
            verb: None,
        }
    }

    pub fn set_noun(&mut self, noun: usize) {
        self.noun = Some(noun);
    }

    pub fn set_verb(&mut self, verb: usize) {
        self.verb = Some(verb);
    }

    pub fn run(&mut self) -> usize {
        // Copy program into memory.
        self.memory.clone_from(&self.program);

        // set noun and verb if set.
        if self.noun.is_some() {
            self.memory[1] = self.noun.unwrap();
        }
        if self.verb.is_some() {
            self.memory[2] = self.verb.unwrap();
        }

        let mut running: bool = true;
        self.pointer = 0;

        while running {
            let opcode: usize = self.memory[self.pointer];

            match opcode {
                1 => self.add(),
                2 => self.multiply(),
                99 => {
                    running = false;
                }
                other => {
                    println!("Unknown opcode {}", other)
                }
            }
        }
        self.memory[0]
    }

    fn add(&mut self) {
        let a: usize = self.memory[self.memory[self.pointer + 1]];
        let b: usize = self.memory[self.memory[self.pointer + 2]];
        let c: usize = self.memory[self.pointer + 3];
        self.memory[c] = a + b;
        self.pointer += 4;
    }

    fn multiply(&mut self) {
        let a: usize = self.memory[self.memory[self.pointer + 1]];
        let b: usize = self.memory[self.memory[self.pointer + 2]];
        let c: usize = self.memory[self.pointer + 3];
        self.memory[c] = a * b;
        self.pointer += 4;
    }
}
