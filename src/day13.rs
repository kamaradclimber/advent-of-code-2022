use std::cmp::Ordering;
use std::fs;

pub fn solve(input_file: String, part: u8) {
    let contents = fs::read_to_string(&input_file).expect("Could not read input_file");
    let mut packets = vec![];
    let lines = contents.lines();
    let mut result = 0;
    for line in lines {
        if line == "" {
            continue;
        }
        let p: Packet = line.parse().expect("We must provide valid inputs");
        packets.push(p);
    }
    if part == 1 {
        for (index, chunk) in packets.chunks(2).enumerate() {
            let (a, b) = if let [a, b] = chunk { (a, b) } else { panic!() };
            if a <= b {
                println!("pair {0}  {a:?} and {b:?} are in the right order", index + 1);
                result += index + 1;
            }
        }
        println!("Solution for part {0}, is {result}", part);
    } else {
        let p1: Packet = "[[2]]".parse().unwrap();
        let p1_copy = p1.clone();
        packets.push(p1);
        let p2: Packet = "[[6]]".parse().unwrap();
        let p2_copy = p2.clone();
        packets.push(p2);
        packets.sort();

        let n = packets.iter().position(|r| r == &p1_copy).unwrap() + 1;
        let m = packets.iter().position(|r| r == &p2_copy).unwrap() + 1;
        result = n * m;
        println!("Solution for part {0}, is {result}", part);
    }
}

#[derive(PartialEq, Eq, Clone)]
enum Packet {
    List(Vec<Packet>),
    Integer(u32),
}

#[derive(Debug)]
enum PacketParsingError {
    UnexpectedCharacter(char),
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).expect("Ordering is total")
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Packet) -> Option<Ordering> {
        match (self, other) {
            (Packet::Integer(e1), Packet::Integer(e2)) => e1.partial_cmp(e2),
            (Packet::Integer(e1), _) => Packet::List(vec![Packet::Integer(*e1)]).partial_cmp(other),
            (_, Packet::Integer(e1)) => self.partial_cmp(&Packet::List(vec![Packet::Integer(*e1)])),
            (Packet::List(l1), Packet::List(l2)) => l1.partial_cmp(&l2),
        }
    }
}

impl std::fmt::Debug for Packet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Packet::Integer(el) => write!(f, "{el}")?,
            Packet::List(list) => {
                write!(f, "[")?;
                for (i, el) in list.iter().enumerate() {
                    write!(f, "{:?}", el)?;
                    if i < list.len() - 1 {
                        write!(f, ",")?;
                    }
                }
                write!(f, "]")?
            }
        }
        Ok(())
    }
}

fn extract_one_entry(s: &str) -> &str {
    let mut opened_brackets = 0;
    for (i, c) in s.chars().enumerate() {
        match c {
            '[' => opened_brackets += 1,
            ']' => opened_brackets -= 1,
            _ => (),
        }
        if opened_brackets == 0 {
            return &s[0..=i];
        }
    }
    return s;
}

fn read_litteral(s: &str) -> Option<u32> {
    let mut i = 0;
    for c in s.chars() {
        if !c.is_digit(10) {
            break;
        }
        i += 1;
    }
    if i == 0 {
        return None;
    }
    let integer = s[0..i].parse().expect("We have checked it is a well formed integer");
    return Some(integer);
}

impl std::str::FromStr for Packet {
    type Err = PacketParsingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // we always assume well-formed entries
        if s == "[]" {
            return Ok(Packet::List(vec![]));
        }
        let next_char = s.chars().next().expect("There should be at least one char left in the string");
        if next_char.is_digit(10) {
            return Ok(Packet::Integer(read_litteral(s).unwrap()));
        }
        if s.starts_with('[') {
            assert!(s.ends_with(']'));
            let mut items = vec![];
            let mut cursor = 1;
            loop {
                match read_litteral(&s[cursor..s.len() - 1]) {
                    None => {
                        let next_entry = extract_one_entry(&s[cursor..s.len() - 1]);
                        cursor += next_entry.len();

                        let subpacket = Packet::from_str(next_entry).expect("Sub packet must be well-formed");
                        items.push(subpacket);
                        if &s[cursor..=cursor] == "," {
                            cursor += 1;
                        } else {
                            break;
                        }
                    }
                    Some(u) => {
                        items.push(Packet::Integer(u));
                        cursor += u.to_string().len();
                        if &s[cursor..=cursor] == "," {
                            cursor += 1;
                        } else {
                            break;
                        }
                    }
                }
            }
            Ok(Packet::List(items))
        } else {
            Err(PacketParsingError::UnexpectedCharacter(next_char))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn order_packets() {
        let p1 = Packet::from_str("[]").expect("Parsing should work");
        let p2 = Packet::from_str("[]").expect("Parsing should work");
        assert!(p1 <= p2);

        let p1 = Packet::from_str("[2,3,4]").expect("Parsing should work");
        let p2 = Packet::from_str("4").expect("Parsing should work");
        assert!(p1 <= p2);

        let p1 = Packet::from_str("[2,3,4]").expect("Parsing should work");
        let p2 = Packet::from_str("[2,3,4]").expect("Parsing should work");
        assert!(p1 <= p2);

        let p1 = Packet::from_str("[2,3,4]").expect("Parsing should work");
        let p2 = Packet::from_str("[2,2,4]").expect("Parsing should work");
        assert!(p1 > p2);

        let p1 = Packet::from_str("[2,3,4]").expect("Parsing should work");
        let p2 = Packet::from_str("[2,3,4,5]").expect("Parsing should work");
        assert!(p1 <= p2);

        let p1 = Packet::from_str("[2,3,4,5]").expect("Parsing should work");
        let p2 = Packet::from_str("[2,3,4]").expect("Parsing should work");
        assert!(p1 > p2);

        let p1 = Packet::from_str("[2,3,4,5]").expect("Parsing should work");
        let p2 = Packet::from_str("[]").expect("Parsing should work");
        assert!(p1 > p2);
    }

    #[test]
    fn parse_packet() {
        let s = "[]";
        let p = Packet::from_str(&s).expect("Parsing should work");
        assert_eq!(p, Packet::List(vec![]));

        let s = "139";
        let p = Packet::from_str(&s).expect("Parsing should work");
        assert_eq!(p, Packet::Integer(139));

        let s = "[139]";
        let p = Packet::from_str(&s).expect("Parsing should work");
        assert_eq!(p, Packet::List(vec![Packet::Integer(139)]));

        let s = "[139,42]";
        let p = Packet::from_str(&s).expect("Parsing should work");
        let expectation = Packet::List(vec![Packet::Integer(139), Packet::Integer(42)]);
        assert_eq!(p, expectation);

        let s = "[139,[],42]";
        let p = Packet::from_str(&s).expect("Parsing should work");
        let expectation = Packet::List(vec![Packet::Integer(139), Packet::List(vec![]), Packet::Integer(42)]);
        assert_eq!(p, expectation);
    }

    #[test]
    fn extract_one_entry_works() {
        let s = "[]";
        assert_eq!(extract_one_entry(&s), "[]");

        let s = "[A]";
        assert_eq!(extract_one_entry(&s), "[A]");

        let s = "[A, B, [C,D], [], [D, E, [F, []]]]";
        assert_eq!(extract_one_entry(&s), s);

        let s = "[A, []] a suffix that is not relevant []";
        assert_eq!(extract_one_entry(&s), "[A, []]");

        let s = "";
        assert_eq!(extract_one_entry(&s), "");

        let s = "A, B";
        assert_eq!(extract_one_entry(&s), "A");
    }
}
