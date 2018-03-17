use std::io::{stdin, Read};

/// This enum represents the eight valid instructions in brainf*ck
#[derive(PartialEq)]
#[derive(Debug)]
#[derive(Copy)]
#[derive(Clone)]
pub enum InsnKind {
    ADD,
    SUB,
    LEFT,
    RIGHT,
    READ,
    WRITE,
    OPEN,
    CLOSE
}

/// This struct represents a brainf*ck instruction with an additional usize operand
#[derive(PartialEq)]
#[derive(Debug)]
pub struct Insn {
    kind: InsnKind,
    operand: usize
}

impl Insn {
    pub fn new(kind: InsnKind, operand: usize) -> Insn {
        Insn {
            kind: kind,
            operand: operand
        }
    }
}

/// This function converts a string into a Vec<InsnType>
/// All non-brainf*ck characters are ignored
fn lex(code: &str) -> Vec<InsnKind> {
    code.chars()
        .map(|x| match x {
            '+' => Some(InsnKind::ADD),
            '-' => Some(InsnKind::SUB),
            '<' => Some(InsnKind::LEFT),
            '>' => Some(InsnKind::RIGHT),
            ',' => Some(InsnKind::READ),
            '.' => Some(InsnKind::WRITE),
            '[' => Some(InsnKind::OPEN),
            ']' => Some(InsnKind::CLOSE),
            _ => None
        })
        .filter(|x| x.is_some())
        .map(|x| x.unwrap())
        .collect()
}

/// Parses a Vec<InsnKind> into a Vec<Insn>
/// Insn is a struct that has a usize operand 
/// which can be a count in the case of ADD, SUB, 
/// LEFT, RIGHT, READ, or WRITE instructions
/// It can also be a location in the Vec<Insn> in
/// the case of OPEN or CLOSE instructions
///
/// This function compresses runs of multiple instructions
/// into a single instruction with a count
fn parse_tokens(tokens: &Vec<InsnKind>) -> Vec<Insn> {
    let len = tokens.len();

    let mut result = Vec::new();
    let mut index = 0;

    while index < len {
        let token = tokens[index];
        match token {
            InsnKind::ADD | InsnKind::SUB | InsnKind::LEFT | InsnKind::RIGHT | InsnKind::READ | InsnKind::WRITE => {
                let mut count = 1;
                
                while index + count < len && tokens[index + count] == token {
                    count += 1;
                }
                
                index += count;
                result.push(Insn::new(token, count));
            }   
            _ => {
                index += 1;
                result.push(Insn::new(token, 1));
            }
        }
    }

    result
}

/// This function calculates the locations of matching pairs of [ and ]
/// and sets the operands of each instruction to be the location
/// of the instruction it is matched with
fn compute_jumps(insns: &mut Vec<Insn>) {
    let mut loops = Vec::new();

    let mut stack = Vec::new();
    for (i, insn) in insns.iter().enumerate() {
        if insn.kind == InsnKind::OPEN {
            stack.push(i);
        } else if insn.kind == InsnKind::CLOSE {
            let j = stack.pop();
            
            if j.is_none() {
                panic!("] without matching [");
            }

            loops.push((i, j.unwrap()));
        }
    }

    for (a, b) in loops {
        insns[a].operand = b + 1;
        insns[b].operand = a + 1;
    }
}

/// This function parses brainf*ck source code into a Vec<Insn>
/// It compresses runs of instructions into single instructions for efficiency of execution
/// It also computes the jump locations of the loop instructions
pub fn parse(code: &str) -> Vec<Insn> {
    let tokens = lex(code);
    let mut ir = parse_tokens(&tokens);
    compute_jumps(&mut ir);
    ir
}

/// This function runs brainf*ck code
pub fn run(ir: &Vec<Insn>) {
    let mut ip = 0;
    let mut dp = 0;
    let mut data : [u8; 30000] = [0; 30000];

    let mut new_ip : usize;

    while ip < ir.len() {
        new_ip = ip + 1;

        let insn = &ir[ip];
        match insn.kind {
            InsnKind::ADD => data[dp] = (((data[dp] as usize) + insn.operand) % 256) as u8,
            InsnKind::SUB => data[dp] = (((data[dp] as usize) - insn.operand) % 256) as u8,
            InsnKind::LEFT => dp -= insn.operand,
            InsnKind::RIGHT => dp += insn.operand,
            InsnKind::READ => { let stdin = stdin(); for _ in 0..insn.operand { data[dp] = stdin.lock().bytes().next().unwrap_or(Ok(0)).unwrap_or(0); } },
            InsnKind::WRITE => for _ in 0..insn.operand { print!("{}", data[dp] as char); },
            InsnKind::OPEN => if data[dp] == 0 { new_ip = insn.operand; },
            InsnKind::CLOSE => if data[dp] != 0 { new_ip = insn.operand; },
        }

        ip = new_ip;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lex() {
        let test = "a+z-b>y<_[1]#,5.&";
        let result = lex(test);
        assert_eq!(result, vec![InsnKind::ADD, InsnKind::SUB, InsnKind::RIGHT, InsnKind::LEFT, 
                            InsnKind::OPEN, InsnKind::CLOSE, InsnKind::READ, InsnKind::WRITE]);
    }   

    #[test]
    fn test_parse_tokens() {
        let test = vec![InsnKind::ADD, InsnKind::ADD, InsnKind::ADD, InsnKind::OPEN, InsnKind::SUB, InsnKind::CLOSE, 
                        InsnKind::RIGHT, InsnKind::RIGHT, InsnKind::RIGHT, InsnKind::LEFT, InsnKind::LEFT, InsnKind::LEFT,
                        InsnKind::READ, InsnKind::READ, InsnKind::WRITE, InsnKind::WRITE];
        let result = parse_tokens(&test);
        assert_eq!(result, vec![Insn::new(InsnKind::ADD, 3), Insn::new(InsnKind::OPEN, 1), Insn::new(InsnKind::SUB, 1), Insn::new(InsnKind::CLOSE, 1),
                                Insn::new(InsnKind::RIGHT, 3), Insn::new(InsnKind::LEFT, 3), Insn::new(InsnKind::READ, 2), Insn::new(InsnKind::WRITE, 2)]);
    }

    #[test]
    fn test_compute_jumps() {
        let mut test = vec![Insn::new(InsnKind::ADD, 3), Insn::new(InsnKind::OPEN, 1), Insn::new(InsnKind::SUB, 1), 
                            Insn::new(InsnKind::OPEN, 1), Insn::new(InsnKind::SUB, 1), Insn::new(InsnKind::CLOSE, 1), Insn::new(InsnKind::CLOSE, 1),
                            Insn::new(InsnKind::RIGHT, 3), Insn::new(InsnKind::LEFT, 3), Insn::new(InsnKind::READ, 2), Insn::new(InsnKind::WRITE, 2)];
        compute_jumps(&mut test);
        assert_eq!(test[1].operand, 7);
        assert_eq!(test[6].operand, 2);
        assert_eq!(test[3].operand, 6);
        assert_eq!(test[5].operand, 4);
    }

    #[test]
    fn test_parse() {
        let test = "+++[-[-]]>>><<<,,..";
        let result = parse(test);
        assert_eq!(result, vec![Insn::new(InsnKind::ADD, 3), Insn::new(InsnKind::OPEN, 7), Insn::new(InsnKind::SUB, 1), 
                                Insn::new(InsnKind::OPEN, 6), Insn::new(InsnKind::SUB, 1), Insn::new(InsnKind::CLOSE, 4), Insn::new(InsnKind::CLOSE, 2),
                                Insn::new(InsnKind::RIGHT, 3), Insn::new(InsnKind::LEFT, 3), Insn::new(InsnKind::READ, 2), Insn::new(InsnKind::WRITE, 2)]);   
    }
}