use std::collections::HashSet;
use std::fs;

pub fn solve(input_file: String, part: u8) {
    let contents = fs::read_to_string(&input_file).expect("Could not read input_file");
    let lines = contents.lines();
    let mut forest: Vec<Vec<u32>> = vec![];
    for (y, line) in lines.enumerate() {
        forest.push(vec![]);
        let chars: Vec<char> = line.chars().collect();
        for tree in chars.iter() {
            let tree_height = tree.to_digit(10).unwrap();
            forest[y].push(tree_height);
        }
    }
    let mut visibles_from_left :  Vec<HashSet<(usize,usize)>> = vec![];
    let mut visibles_from_right : Vec<HashSet<(usize,usize)>> = vec![];
    let mut visibles_from_upper : Vec<HashSet<(usize,usize)>> = vec![];
    let mut visibles_from_lower : Vec<HashSet<(usize,usize)>> = vec![];
    let width = forest[0].len();
    for y in 0..forest.len() {
        visibles_from_left.push(HashSet::new());
        visibles_from_left[y].insert((0,y));
        let mut heighest_visible_left = forest[y][0];
        for x in 0..forest[y].len() {
            if forest[y][x] > heighest_visible_left {
                visibles_from_left[y].insert((x,y));
                heighest_visible_left = forest[y][x];
            }
        }
        visibles_from_right.push(HashSet::new());
        visibles_from_right[y].insert((forest[y].len() - 1, y));
        let mut heighest_visible_right = forest[y][forest[y].len() - 1];
        for x in (0..forest[y].len()).rev() {
            if forest[y][x] > heighest_visible_right {
                visibles_from_right[y].insert((x,y));
                heighest_visible_right = forest[y][x];
            }
        }
    }
    for x in 0..forest[0].len() {
        visibles_from_upper.push(HashSet::new());
        visibles_from_upper[x].insert((x,0));
        let mut heighest_visible_up = forest[0][x];
        for y in 0..forest.len() {
            if forest[y][x] > heighest_visible_up {
                visibles_from_upper[x].insert((x,y));
                heighest_visible_up = forest[y][x];
            }
        }
        visibles_from_lower.push(HashSet::new());
        visibles_from_lower[x].insert((x,forest.len() - 1));
        let mut heighest_visible_bottom = forest[forest.len() - 1][x];
        for y in (0..forest.len()).rev() {
            if forest[y][x] > heighest_visible_bottom {
                visibles_from_lower[x].insert((x,y));
                heighest_visible_bottom = forest[y][x];
            }
        }
    }
    if part == 1 {
        let a = [visibles_from_lower, visibles_from_left, visibles_from_right, visibles_from_upper].concat();
        let visibles = a.iter().fold(HashSet::new(), |total, el| total.union(el).cloned().collect());

        println!("Visible trees for part {0}, is {1}", part, visibles.len());
    } else {

        let my_forest = &forest;
        let best_scenic_view : usize = (0..forest.len()).flat_map(|y| (0..width).map(move |x| scenic_view(x,y, my_forest)))
            .max()
            .expect("Forest should not be empty");
        
        println!("Best scenic view is {best_scenic_view}");
    }
}

fn scenic_view(x: usize, y: usize, forest: &Vec<Vec<u32>>) -> usize {
    // to the right
    let mut view_length_right = 0;
    for xx in x+1..forest.len() {
        view_length_right += 1;
        if forest[y][xx] >= forest[y][x] {
            break
        }
    }
    // to the left
    let mut view_length_left = 0;
    for xx in (0..x).rev() {
        view_length_left += 1;
        if forest[y][xx] >= forest[y][x] {
            break
        }
    }
    // to upper
    let mut view_length_up = 0;
    for yy in (0..y).rev() {
        view_length_up += 1;
        if forest[yy][x] >= forest[y][x] {
            break
        }
    }
    // to lower
    let mut view_length_lower = 0;
    for yy in y+1..forest.len() {
        view_length_lower += 1;
        if forest[yy][x] >= forest[y][x] {
            break
        }
    }

    let result = view_length_right * view_length_left * view_length_up * view_length_lower;
    // println!("({x},{y}) U/L/D/R: {view_length_up}/{view_length_left}/{view_length_lower}/{view_length_right} => {result}");
    result
}
