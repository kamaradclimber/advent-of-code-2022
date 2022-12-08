use std::collections::HashSet;
use std::fs;

pub fn solve(input_file: String, part: u8) {
    let contents = fs::read_to_string(&input_file).expect("Could not read input_file");
    let lines = contents.lines();
    let mut forest: Vec<Vec<u32>> = vec![];
    for (y, line) in lines.enumerate() {
        forest.push(vec![]);
        let chars: Vec<char> = line.chars().collect();
        for (x, tree) in chars.iter().enumerate() {
            let tree_height = tree.to_digit(10).unwrap();
            forest[y].push(tree_height);
        }
    }
    let mut visibles: HashSet<(usize, usize)> = HashSet::new();
    for y in 0..forest.len() {
        visibles.insert((0, y));
        let mut heighest_visible_left = forest[y][0];
        for x in 0..forest[y].len() {
            if forest[y][x] > heighest_visible_left {
                visibles.insert((x, y));
                heighest_visible_left = forest[y][x];
            }
        }
        visibles.insert((forest.len() - 1, y));
        let mut heighest_visible_right = forest[y][forest[y].len() - 1];
        for x in (0..forest[y].len()).rev() {
            if forest[y][x] > heighest_visible_right {
                visibles.insert((x, y));
                heighest_visible_right = forest[y][x];
            }
        }
    }
    for x in 0..forest[0].len() {
        visibles.insert((x, 0));
        let mut heighest_visible_up = forest[0][x];
        for y in 0..forest.len() {
            if forest[y][x] > heighest_visible_up {
                visibles.insert((x, y));
                heighest_visible_up = forest[y][x];
            }
        }
        visibles.insert((x, forest.len() - 1));
        let mut heighest_visible_bottom = forest[forest.len() - 1][x];
        for y in (0..forest.len()).rev() {
            if forest[y][x] > heighest_visible_bottom {
                visibles.insert((x, y));
                heighest_visible_bottom = forest[y][x];
            }
        }
    }

    println!("Visible trees for part {0}, is {1}", part, visibles.len());
}
