use crate::utils::{get_input, Lexer};

pub fn part1() -> i32 {
    let mut answer = 0i32;

    execute(|cycles, x_reg| {
        if ((cycles + 20) % 40) == 0 {
            answer += cycles as i32 * x_reg;
        }
    });

    answer
}

pub fn part2() -> String {
    let mut output = String::from("\n");

    execute(|cycles, x_reg| {
        let row_x = cycles % 40;

        if (row_x - x_reg).abs() <= 1 {
            output += "█";
        } else {
            output += " ";
        }

        if row_x == 39 {
            output += "\n";
        }
    });

    output
}

#[derive(Debug, PartialEq)]
enum Op {
    Addx,
    Noop,
}

#[derive(Debug)]
struct Instruction {
    op: Op,
    arg: Option<i32>,
}

fn execute<F: FnMut(i32, i32)>(mut f: F) {
    let mut x_register = 1i32;
    let mut cycles = 0i32;

    let mut inc_cycle = |curr_reg_value: i32| {
        f(cycles, curr_reg_value);
        cycles += 1;
    };

    for instruction in get_instructions() {
        inc_cycle(x_register);
        if instruction.op == Op::Addx {
            inc_cycle(x_register);
        }
        
        match instruction.arg {
            Some(v) => x_register += v,
            _ => {},
        };
    }
}

fn get_instructions() -> Vec<Instruction> {
    let mut instructions = vec![];
    let mut lexer = Lexer::new(get_input(10));

    while !lexer.done() {
        if lexer.consume_str("addx ") {
            instructions.push(Instruction { 
                op: Op::Addx, 
                arg: Some(lexer.consume_integer(10).unwrap()),
            })
        } else if lexer.consume_str("noop") {
            instructions.push(Instruction { op: Op::Noop, arg: None });
        } else {
            panic!();
        }

        lexer.skip_whitespace();
    }

    instructions
}
