use std::fs;

pub fn solve(input_file: String, part: u8) {
    let contents = fs::read_to_string(&input_file).expect("Could not read input_file");
    let lines = contents.lines();
    let mut result = 0;
    for line in lines {
        println!("Line is {0}", &line);
        result += snafu2normal(string2snafu(line));
    }
    println!("Score for part {0}, is {1}", part, snafu2string(normal2snafu(result as i64)));
}


fn snafu2normal(snafu: Vec<i32>) -> i64 {
    let base : i64 = 5;
    let mut sum = 0;
    for (i, value) in snafu.iter().rev().enumerate() {
        let intermediate = (value.abs() as i64) * base.pow(i as u32);
        if *value < 0 {
            sum -= intermediate;
        } else {
            sum += intermediate;
        }
    }
    sum
}

fn string2snafu(s: &str) -> Vec<i32> {
        s.chars().map(|c| match c {
            '=' => -2,
            '-' => -1,
            '0' => 0,
            '1' => 1,
            '2' => 2,
            _ => panic!()
        }).collect()
}

fn normal2snafu(mut n: i64) -> Vec<i32> {
    let mut rev_res = vec![];
    loop {
        let m = n % 5;
        n = n / 5;
        rev_res.push(m as i32);
        if n == 0 {
            break;
        }
    }
    for i in 0..rev_res.len() {
        if rev_res[i] > 2 {
            // first let's add one to the (i+1)th digit
            for j in i+1..=rev_res.len() {
                if j == rev_res.len() {
                    rev_res.push(1);
                    break;
                }
                if rev_res[j] < 4 {
                    rev_res[j] += 1;
                    break;
                }
                rev_res[j] = 0;
            }
            // then replace i-th digit by corresponding entry
            rev_res[i] = rev_res[i] - 5;

        }
    }
    rev_res.iter().map(|digit| *digit).rev().collect()

}

fn snafu2string(snafu: Vec<i32>) -> String {
    snafu.iter().map(|s|
              match s {
                  i if *i >= 0 => std::char::from_digit(*i as u32, 10).unwrap(),
                  -1 => '-',
                  -2 => '=',
                  _ => panic!()
              }).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_normal() {
        assert_eq!(snafu2normal(string2snafu("1=-0-2")), 1747);
        assert_eq!(snafu2normal(string2snafu("12111")), 906);
        assert_eq!(snafu2normal(string2snafu("2=0=")), 198);
        assert_eq!(snafu2normal(string2snafu("21")), 11);
        assert_eq!(snafu2normal(string2snafu("2=01")), 201);
        assert_eq!(snafu2normal(string2snafu("111")), 31);
        assert_eq!(snafu2normal(string2snafu("20012")), 1257);
    }

    #[test]
    fn normal_to_snafu() {
        assert_eq!(snafu2string(normal2snafu(0)), "0"); 
        assert_eq!(snafu2string(normal2snafu(1)), "1");
        assert_eq!(snafu2string(normal2snafu(2)), "2");
        assert_eq!(snafu2string(normal2snafu(3)), "1=");
        assert_eq!(snafu2string(normal2snafu(4)), "1-");
        assert_eq!(snafu2string(normal2snafu(5)), "10");
        assert_eq!(snafu2string(normal2snafu(6)), "11");
        assert_eq!(snafu2string(normal2snafu(7)), "12");
        assert_eq!(snafu2string(normal2snafu(8)), "2=");
        assert_eq!(snafu2string(normal2snafu(9)), "2-");
        assert_eq!(snafu2string(normal2snafu(10)), "20");
        assert_eq!(snafu2string(normal2snafu(15)), "1=0");
        assert_eq!(snafu2string(normal2snafu(20)), "1-0");
        assert_eq!(snafu2string(normal2snafu(2022)), "1=11-2");
        assert_eq!(snafu2string(normal2snafu(314159265)), "1121-1110-1=0");
        assert_eq!(snafu2string(normal2snafu(12345)), "1-0---0");
    }

    #[test]
    fn reverse() {
        for i in 0..100 {
            let snafu = normal2snafu(i);
            assert_eq!(snafu2normal(snafu), i);
        }
    }

}
