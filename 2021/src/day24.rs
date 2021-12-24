use anyhow::Context;
use aoc2021::Input;
use itertools::Itertools;
use std::io::BufRead;

#[derive(Debug, Copy, Clone)]
enum Register {
    W,
    X,
    Y,
    Z,
}

impl TryFrom<&str> for Register {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "w" => Ok(Self::W),
            "x" => Ok(Self::X),
            "y" => Ok(Self::Y),
            "z" => Ok(Self::Z),
            _ => anyhow::bail!("invalid register '{}'", value),
        }
    }
}

trait EqualOp {
    type Output;
    fn equal(self, rhs: Self) -> Self::Output;
}

impl EqualOp for i32 {
    type Output = Self;
    fn equal(self, rhs: Self) -> Self::Output {
        (self == rhs) as i32
    }
}

type Registers<T> = [T; 4];

#[derive(Debug, Copy, Clone)]
enum Operation {
    Add,
    Mul,
    Div,
    Mod,
    Eql,
}

impl std::fmt::Display for Operation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Add => write!(f, "+"),
            Self::Mul => write!(f, "*"),
            Self::Div => write!(f, "/"),
            Self::Mod => write!(f, "%"),
            Self::Eql => write!(f, "=="),
        }
    }
}

#[derive(Debug)]
enum Instruction {
    Inp(Register),
    Add(Register, Value),
    Mul(Register, Value),
    Div(Register, Value),
    Mod(Register, Value),
    Eql(Register, Value),
}

impl Instruction {
    fn evaluate<T>(&self, alu: &mut Alu<T>)
    where
        T: std::ops::Add<Output = T>,
        T: std::ops::Mul<Output = T>,
        T: std::ops::Div<Output = T>,
        T: std::ops::Div<Output = T>,
        T: std::ops::Rem<Output = T>,
        T: EqualOp<Output = T>,
        T: Default,
        T: Clone,
        T: From<i32>,
    {
        macro_rules! op {
            ($alu:ident, $register:ident, $value:ident, $map:expr) => {{
                let value = $value.resolve(&$alu);
                alu.map(*$register, |a| $map(a, value))
            }};
        }
        match self {
            Self::Inp(register) => alu.write_input_to(*register),
            Self::Add(register, value) => op!(alu, register, value, |a, b| a + b),
            Self::Mul(register, value) => op!(alu, register, value, |a, b| a * b),
            Self::Div(register, value) => op!(alu, register, value, |a, b| a / b),
            Self::Mod(register, value) => op!(alu, register, value, |a, b| a % b),
            Self::Eql(register, value) => op!(alu, register, value, |a: T, b| a.equal(b)),
        }
    }
}

impl TryFrom<&str> for Instruction {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let (ins, args) = value
            .split_once(' ')
            .ok_or_else(|| anyhow::anyhow!("lol"))?;

        let res = match ins {
            "inp" => Self::Inp(args.try_into()?),
            ins => {
                let (r, v) = args.split_once(' ').ok_or_else(|| {
                    anyhow::anyhow!("expected 2 arguments for instruction '{}'", ins)
                })?;
                let (r, v) = (r.try_into()?, v.try_into()?);
                match ins {
                    "add" => Self::Add(r, v),
                    "mul" => Self::Mul(r, v),
                    "div" => Self::Div(r, v),
                    "mod" => Self::Mod(r, v),
                    "eql" => Self::Eql(r, v),
                    _ => anyhow::bail!("invalid instruction '{}'", ins),
                }
            }
        };
        Ok(res)
    }
}

#[derive(Debug)]
enum Value {
    Register(Register),
    Constant(i32),
}

impl Value {
    fn resolve<T>(&self, alu: &Alu<T>) -> T
    where
        T: Clone,
        T: From<i32>,
    {
        match self {
            Self::Register(register) => alu.read(*register).clone(),
            Self::Constant(constant) => (*constant).into(),
        }
    }
}

impl TryFrom<&str> for Value {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if let Ok(register) = value.try_into() {
            Ok(Self::Register(register))
        } else {
            Ok(Self::Constant(value.parse().context(format!(
                "invalid value, neither a register nor a number '{}'",
                value
            ))?))
        }
    }
}

type AluInput<T> = Box<dyn Iterator<Item = T>>;

struct Alu<T> {
    registers: Registers<T>,
    input: AluInput<T>,
}

impl<T> Alu<T> {
    fn new<I>(input: I) -> Self
    where
        T: Default,
        I: IntoIterator<Item = T>,
        <I as IntoIterator>::IntoIter: 'static,
    {
        let input = Box::new(input.into_iter());
        Self {
            registers: Registers::default(),
            input,
        }
    }

    fn map<F>(&mut self, register: Register, f: F)
    where
        F: FnOnce(T) -> T,
        T: Default,
    {
        let value = std::mem::take(&mut self.registers[register as usize]);
        self.registers[register as usize] = f(value);
    }

    fn read(&self, register: Register) -> &T {
        &self.registers[register as usize]
    }

    #[allow(unused)]
    fn read_mut(&mut self, register: Register) -> &mut T {
        &mut self.registers[register as usize]
    }

    #[allow(unused)]
    fn write(&mut self, register: Register, value: T) {
        self.registers[register as usize] = value;
    }

    fn write_input_to(&mut self, register: Register) {
        self.registers[register as usize] = self.input.next().expect("need more input");
    }
}

impl<T: std::fmt::Debug> std::fmt::Debug for Alu<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.registers)
    }
}

#[derive(Debug, Clone)]
enum Symbolic {
    #[allow(unused)]
    Input(u8),
    Value(i32),
    Term(Box<(Operation, Symbolic, Symbolic)>),
}

impl std::fmt::Display for Symbolic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Input(i) => write!(f, "i{}", i),
            Self::Value(value) => write!(f, "{}", value),
            Self::Term(term) => write!(f, "({}{}{})", term.1, term.0, term.2),
        }
    }
}

impl Default for Symbolic {
    fn default() -> Self {
        Self::Value(0)
    }
}

impl From<i32> for Symbolic {
    fn from(value: i32) -> Self {
        Symbolic::Value(value)
    }
}

impl std::ops::Add for Symbolic {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::Value(0), x) => x,
            (x, Self::Value(0)) => x,
            (Self::Value(a), Self::Value(b)) => Self::Value(a + b),
            (a, b) => Self::Term(Box::new((Operation::Add, a, b))),
        }
    }
}

impl std::ops::Mul for Symbolic {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::Value(0), _) => Self::Value(0),
            (_, Self::Value(0)) => Self::Value(0),
            (Self::Value(a), Self::Value(b)) => Self::Value(a * b),
            (a, b) => Self::Term(Box::new((Operation::Mul, a, b))),
        }
    }
}

impl std::ops::Div for Symbolic {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::Value(1), x) => x,
            (x, Self::Value(1)) => x,
            (Self::Value(a), Self::Value(b)) => Self::Value(a / b),
            (a, b) => Self::Term(Box::new((Operation::Div, a, b))),
        }
    }
}

impl std::ops::Rem for Symbolic {
    type Output = Self;

    fn rem(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::Value(a), Self::Value(b)) => Self::Value(a % b),
            (a, b) => Self::Term(Box::new((Operation::Mod, a, b))),
        }
    }
}

impl EqualOp for Symbolic {
    type Output = Self;

    fn equal(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::Value(a), Self::Value(b)) => Self::Value((a == b) as i32),
            (a, b) => Self::Term(Box::new((Operation::Eql, a, b))),
        }
    }
}

fn eval2(instructions: Vec<Instruction>) -> anyhow::Result<()> {
    // let inputs = (0..14).map(|_| (1..=9)).multi_cartesian_product();

    // let input = vec![1, 1, 7, 8, 1, 2, 8, 7, 2, 3, 2, 3, 3, 9];
    // let input = vec![9; 14];
    //              [9, 1, 9, 6, 1, 1, 7, 6, 1, 2, 1, 3, 1, 9]
    //                  X     X  X
    let input = vec![5, 1, 6, 7, 1, 2, 8, 7, 2, 3, 2, 3, 9, 1];
    // let mut input = vec![1, 1, 1, 1, 1, 3, 9];

    let mut alu = Alu::new(input);
    for (i, ins) in instructions.iter().enumerate() {
        ins.evaluate(&mut alu);

        if i == 59 || i == 113 || i == 149 || i == 185 || i == 203 || i == 221 || i == 239 {
            println!("{:>3} {:?} \t{:?}", i + 1, ins, alu.registers);
        }
        if i == 60 || i == 114 || i == 150 || i == 186 || i == 204 || i == 222 || i == 240 {
            println!("{:>3} {:?} \t{:?}", i + 1, ins, alu.registers);
        }
    }
    println!("{:?}", alu.registers);
    Ok(())
}

fn eval(instructions: Vec<Instruction>) -> anyhow::Result<()> {
    // [9, 1, 8, 9, 7, 3, 9, 9, 4, 9, 8, 9, 9, 5]
    // [5, 1, 1, 2, 1, 1, 7, 6, 1, 2, 1, 3, 9, 1]

    //  5 <----------------------------------> 1
    //     1 <----------------------------> 9
    //              1 <----------------> 3
    //        1<>2     1<>7  6<>1  2<>1
    //                       ?  ?        ?
    //  0  1  2  3  4  5  6  7  8  9 10 11 12 13
    // [1, 1, 1, 1, 1, 3, 9, 9, 4, 9, 8, 3, 2, 9]
    // [X, X, X, ?, ?, 1, 7, 8, 3, 9, 8, ?, 2, 9]
    //                 2, 8  7, 2  2, 1     1, 8
    //                 9, 9  6, 1  3, 2
    //                             4, 3
    //                             5, 4
    //                             6, 5
    //                             7, 6
    //                             8, 7

    // let inputs = vec![vec![9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9]].into_iter();
    // [9, 9, 9, 9, 9, 3, 9]
    // PART 1
    let inputs = (0..7).map(|_| (1..=9).rev()).multi_cartesian_product(); // input
    let mut result = Vec::<i32>::new();
    'part1: for input in inputs {
        let r = input.clone();
        let mut alu = Alu::new(r);
        for (i, ins) in instructions.iter().enumerate() {
            ins.evaluate(&mut alu);

            if i == 114 {
                // println!("{:?}", input);
                // println!("{:?} {:?} {}", ins, alu.registers, alu.registers[3] % 26);
                if *alu.read(Register::X) == 1 && *alu.read(Register::Z) % 26 > 20 {
                    result.extend(&input[0..7]);
                    break 'part1;
                }
                break;
            }
        }
    }

    println!("P1: {:?}", result);

    Ok(())
}

pub fn part1(reader: Input) -> anyhow::Result<i32> {
    let instructions = reader
        .lines()
        .map(|line| line?.as_str().try_into())
        .collect::<Result<Vec<Instruction>, _>>()?;

    eval(instructions).unwrap();

    Ok(0)
}

pub fn part2(reader: Input) -> anyhow::Result<i32> {
    let instructions = reader
        .lines()
        .map(|line| line?.as_str().try_into())
        .collect::<Result<Vec<Instruction>, _>>()?;

    eval2(instructions).unwrap();

    Ok(0)
}

pub fn main() {
    aoc2021::cli::run(part1, part2).unwrap();
}
