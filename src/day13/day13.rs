use std::cmp::Ordering;
use std::fmt::{Debug, Formatter};
use std::str::FromStr;
use std::string::ParseError;
use crate::Element::{I, L};

fn main() {
    let input = include_str!("day13.txt");

    let pairs: Vec<(Element, Element)> = input.split("\n\n").map(|pair| {
        let parts = pair.split_once('\n').unwrap();
        let a: Element = parts.0.trim().parse().unwrap();
        let b: Element = parts.1.trim().parse().unwrap();
        (a, b)
    }).collect();

    task1(&pairs);
    task2(&pairs);
}

fn task1(pairs: &Vec<(Element, Element)>) {
    let mut sum = 0;
    for (i, pair) in pairs.iter().enumerate() {
        if pair.0.partial_cmp(&pair.1) == Some(Ordering::Less) {
            sum += i + 1;
        }
    }
    println!("{}", sum);
}

fn task2(pairs: &Vec<(Element, Element)>) {
    let div = (L(vec![L(vec![I(2)])]), L(vec![L(vec![I(6)])]));
    let mut packets = Vec::<Element>::with_capacity(pairs.len() * 2 + 2);
    packets.push(div.0.clone());
    packets.push(div.1.clone());
    for pair in pairs {
        packets.push(pair.0.clone());
        packets.push(pair.1.clone());
    }

    packets.sort();

    let a = packets.binary_search(&div.0.clone()).unwrap() + 1;
    let b = packets.binary_search(&div.1.clone()).unwrap() + 1;

    println!("{}", a * b);
}

#[derive(Clone, PartialEq)]
enum Element {
    I(u32),
    L(Vec<Element>),
}

impl Element {
    const fn is_integer(&self) -> bool {
        matches!(*self, I(_))
    }
    const fn is_list(&self) -> bool {
        matches!(*self, L(_))
    }

    fn integer(self) -> u32 {
        match self {
            I(val) => val,
            L(_) => panic!("Isn't an Integer!"),
        }
    }
    fn list(self) -> Vec<Element> {
        match self {
            L(val) => val,
            I(_) => panic!("Isn't a List!"),
        }
    }

    fn to_list(self) -> Element {
        match self {
            L(_) => self,
            I(_) => L(vec![self]),
        }
    }
}

impl Eq for Element {}

impl PartialOrd<Self> for Element {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.is_integer() && other.is_integer() {
            let aa = self.clone().integer();
            let bb = other.clone().integer();
            if aa < bb {
                Some(Ordering::Less)
            } else if bb < aa {
                Some(Ordering::Greater)
            } else {
                Some(Ordering::Equal)
            }
        } else if self.is_list() && other.is_list() {
            let aa = self.clone().list();
            let bb = other.clone().list();
            let len = aa.len().min(bb.len());
            for i in 0..len {
                let tmp = aa[i].partial_cmp(&bb[i]);
                match tmp {
                    Some(Ordering::Greater) => return tmp,
                    Some(Ordering::Less) => return tmp,
                    _ => {}
                }
            }
            if aa.len() < bb.len() {
                Some(Ordering::Less)
            } else if bb.len() < aa.len() {
                Some(Ordering::Greater)
            } else {
                Some(Ordering::Equal)
            }
        } else {
            self.clone().to_list().partial_cmp(&other.clone().to_list())
        }
    }
}

impl Ord for Element {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl FromStr for Element {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let try_int = s.parse::<u32>();
        if try_int.is_ok() {
            Ok(I(try_int.unwrap()))
        } else {
            let mut val: Vec<Element> = Vec::new();
            let mut open: u32 = 0;
            let mut start: usize = 0;
            let mut end: usize = 0;
            for (i, c) in s.chars().enumerate() {
                match c {
                    ' ' => continue,
                    '[' => {
                        if open == 1 && end >= start { start = i }
                        open += 1;
                    }
                    ']' => open -= 1,
                    ',' => {
                        if open == 1 {
                            end = i;
                            val.push(s.get(start..end).unwrap().parse().unwrap())
                        }
                    }
                    _ => if open == 1 && end >= start { start = i },
                }
            }
            if start > end {
                val.push(s.get(start..s.len() - 1).unwrap().parse().unwrap());
            }
            Ok(L(val))
        }
    }
}

impl Debug for Element {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            I(val) => write!(f, "{}", val),
            L(val) => f.debug_list().entries(val).finish(),
        }
    }
}