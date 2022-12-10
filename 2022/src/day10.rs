#[derive(Debug)]
struct Cpu<T> where T: Clocked {
    register: i32,
    cycles: usize,
    clocked: T
}

trait Clocked {
    fn run_cycle(&mut self, cycles: usize, register: i32);
}

#[derive(Default)]
struct SignalAdder {
    signal: i32
}

impl Clocked for SignalAdder {
    fn run_cycle(&mut self, cycles: usize, register: i32) {
        if cycles == 20 || cycles == 60 || cycles == 100 || cycles == 140 || cycles == 180 || cycles == 220 {
            self.signal += register * cycles as i32;
        }
    }
}

struct Lcd {
    sprite_size: i32,
    row_size: usize
}

impl Clocked for Lcd {
    fn run_cycle(&mut self, cycles: usize, register: i32) {
        let position = ((cycles - 1) % (self.row_size)) as i32;
        let sprite_start = register - 1;
        let sprite_end = register - 1 + self.sprite_size;

        if position == 0 {
            println!();
        }

        if sprite_start <= position && sprite_end > position {
            print!("#");
        } else {
            print!(".");
        }
    }
}

impl<T> Cpu<T> where T: Clocked {
    fn new(clocked: T) -> Cpu<T> {
        Cpu {
            register: 1,
            cycles: 0,
            clocked
        }
    }

    fn parse(&mut self, source: &str) {
        source
            .lines()
            .for_each(|line| {
                if line.starts_with("noop") {
                    self.noop();
                } else if line.starts_with("addx ") {
                    let x = line.strip_prefix("addx ").unwrap().parse::<i32>().expect("Invalid number.");
                    self.addx(x);
                } else {
                    panic!("Invalid instruction: {}", line);
                }
            });
    }

    fn noop(&mut self) {
        self.cycles += 1;
        self.clocked.run_cycle(self.cycles, self.register);
    }

    fn addx(&mut self, x: i32) {
        self.cycles += 1;
        self.clocked.run_cycle(self.cycles, self.register);

        self.cycles += 1;
        self.clocked.run_cycle(self.cycles, self.register);
        self.register += x;
    }
}

fn main() {
    let input = include_str!("../input/day10.input");

    // Part one
    let signal_sum = first_puzzle(input);
    println!("The sum of the six signal strengths is {}.", signal_sum);

    // Part two
    second_puzzle(input);
}

#[test]
fn sample() {
    let input = include_str!("../sample/day10.input");

    assert_eq!(first_puzzle(input), 13140);
    second_puzzle(input);
}

fn first_puzzle(source: &str) -> i32 {
    let mut cpu = Cpu::new(SignalAdder::default());
    cpu.parse(source);

    cpu.clocked.signal
}

fn second_puzzle(source: &str) {
    let mut cpu = Cpu::new(Lcd { sprite_size: 3, row_size: 40 });
    cpu.parse(source);

    println!();
    println!();
}