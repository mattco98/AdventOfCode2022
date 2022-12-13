use std::cmp::{PartialOrd, Ordering};
use crate::utils::{Lexer, get_input};

pub fn part1() -> usize {
    get_data().iter()
        .enumerate()
        .filter(|(_, group)| group[0].partial_cmp(&group[1]).unwrap() == Ordering::Less)
        .map(|(i, _)| i + 1)
        .sum()
}

pub fn part2() -> usize {
    let data = get_data();
    let mut data = data.iter().flatten().collect::<Vec<_>>();

    let marker1 = parse_data(&mut Lexer::new("[[2]]"));
    let marker2 = parse_data(&mut Lexer::new("[[6]]"));

    data.push(&marker1);
    data.push(&marker2);

    data.sort();

    data.iter()
        .enumerate()
        .filter(|(_, packet)| ***packet == marker1 || ***packet == marker2)
        .map(|(i, _)| i + 1)
        .product()
}

#[derive(Debug)]
enum Data {
    Int(u32),
    List(Vec<Data>),
}

impl PartialEq for Data {
    fn eq(&self, other: &Data) -> bool {
        if let Data::Int(l) = self && let Data::Int(r) = other {
            return l == r;
        }

        if let Data::List(l) = self && let Data::List(r) = other {
            if l.len() != r.len() {
                return false;
            }

            for i in 0..l.len() {
                if l[i] != r[i] {
                    return false;
                }
            }

            return true;
        }

        false
    }
}

impl PartialOrd for Data {
    fn partial_cmp(&self, other: &Data) -> Option<Ordering> {
        match self {
            Data::Int(l) => match other {
                Data::Int(r) => l.partial_cmp(r),
                Data::List(_) => Data::List(vec![Data::Int(*l)]).partial_cmp(other),
            },
            Data::List(l) => match other {
                Data::Int(r) => self.partial_cmp(&Data::List(vec![Data::Int(*r)])),
                Data::List(r) => {
                    let mut i = 0;

                    loop {
                        if i >= l.len() {
                            if i >= r.len() {
                                return Some(Ordering::Equal);
                            }

                            return Some(Ordering::Less);
                        }

                        if i >= r.len() {
                            return Some(Ordering::Greater);
                        }

                        let result = l[i].partial_cmp(&r[i]);
                        if !matches!(result, Some(Ordering::Equal)) {
                            return result;
                        }

                        i += 1;
                    }
                }
            }
        }
    }
}

impl Eq for Data {}

impl Ord for Data {
    fn cmp(&self, other: &Data) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

fn get_data() -> Vec<Vec<Data>> {
    let mut all_data = vec![];
    let mut curr_data = vec![];

    for line in get_input(13).lines() {
        if line.is_empty() {
            all_data.push(curr_data);
            curr_data = vec![];
        } else {
            curr_data.push(parse_data(&mut Lexer::new(line)));
        }
    }

    if !curr_data.is_empty() {
        all_data.push(curr_data);
    }

    all_data
}

fn parse_data(lexer: &mut Lexer) -> Data {
    if let Some(num) = lexer.consume_unsigned_integer::<u32>(10) {
        Data::Int(num)
    } else if lexer.consume_char('[') {
        let mut items = vec![];
        
        while !lexer.matches(']') {
            items.push(parse_data(lexer));
            if !lexer.consume_char(',') {
                break;
            }
        }
        
        assert!(lexer.consume_char(']'));

        Data::List(items)
    } else {
        panic!("unexpected char '{}'", lexer.ch());
    }
}