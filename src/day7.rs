use regex::Regex;
use std::fs;

pub fn solve(input_file: String, part: u8) {
    let contents = fs::read_to_string(&input_file).expect("Could not read input_file");
    let lines = contents.lines();

    let cd = Regex::new(r"^\$ cd (.+)$").unwrap();
    let ls = Regex::new(r"^\$ ls$").unwrap();
    let dir_line = Regex::new(r"^dir (.+)$").unwrap();
    let file_line = Regex::new(r"^(\d+) (.+)$").unwrap();

    let mut tree: Vec<FilesystemEntry> = vec![];

    let root = FilesystemEntry::Directory("/".to_string(), vec![], None);
    tree.push(root);

    let mut cwd_id = 0;

    for line in lines {
        if line == "$ cd /" {
            // println!("This is the initial line of the input, ignoring");
        } else if cd.is_match(line) {
            let dir_name = &cd.captures(line).unwrap()[1];
            // println!("Changing directory to {0}", dir_name);
            if dir_name == ".." {
                cwd_id = tree[cwd_id].parent_id();
            } else {
                cwd_id = tree[cwd_id].find_child_id(dir_name, &tree);
            }
        } else if ls.is_match(line) {
            // println!("Displaying content of current working directory");
        } else if file_line.is_match(line) || dir_line.is_match(line) {
            let new_entry;
            if file_line.is_match(line) {
                let size = file_line.captures(line).unwrap()[1].parse::<u32>().unwrap();
                let filename = &file_line.captures(line).unwrap()[2];
                let file = FilesystemEntry::File(filename.to_string(), size);
                // println!("We discover (new) file");
                new_entry = file;
            } else {
                let dirname = &dir_line.captures(line).unwrap()[1];
                let dir = FilesystemEntry::Directory(dirname.to_string(), vec![], Some(cwd_id));
                // println!("We discover (new) dir");
                new_entry = dir;
            };
            let new_id = tree.len();
            tree.push(new_entry);
            let cwd = &mut tree[cwd_id];
            cwd.add_child(new_id);
        } else {
            panic!("We don't know how to parse line {0}", &line);
        }
    }
    if part == 1 {
        let mut sizes = vec![None; tree.len()];

        let mut relevant = 0;
        for (index, _) in tree.iter().enumerate() {
            let size = compute_size(&tree, &mut sizes, index);
            // println!("Computed size is {size}");
            if matches!(tree[index], FilesystemEntry::Directory(..)) && size <= 100000 {
                relevant += size;
            }
        }
        println!("Solution is {relevant}");
    } else {
        let mut sizes = vec![None; tree.len()];

        let root_size = compute_size(&tree, &mut sizes, 0);
        let free_space = 70000000 - root_size;
        let mut relevant_to_delete = root_size;
        let space_to_clean = 30000000 - free_space; // we should check there is indeed something to do
        for (index, _) in tree.iter().enumerate() {
            if matches!(tree[index], FilesystemEntry::Directory(..)) {
                // technically we know we've already computed everything
                let size = compute_size(&tree, &mut sizes, index);
                if size >= space_to_clean && size < relevant_to_delete {
                    relevant_to_delete = size;
                }
            }
        }
        println!("Solution is {relevant_to_delete}");
    }
}

// dynamic programming size computation
fn compute_size(tree: &Vec<FilesystemEntry>, sizes: &mut Vec<Option<u32>>, index: usize) -> u32 {
    match sizes[index] {
        Some(size) => size,
        None => {
            let size = match &tree[index] {
                FilesystemEntry::File(_, size) => *size,
                FilesystemEntry::Directory(_, children, _) => {
                    let mut sum = 0;
                    for child_id in children {
                        sum += compute_size(tree, sizes, *child_id);
                    }
                    sum
                }
            };
            sizes[index] = Some(size);
            size
        }
    }
}

#[derive(Hash, PartialEq, Eq, Debug)]
enum FilesystemEntry {
    // name, size
    File(String, u32),
    // name, child ids, parent id
    Directory(String, Vec<usize>, Option<usize>),
}

impl FilesystemEntry {
    fn name(&self) -> &str {
        match self {
            FilesystemEntry::File(n, _) => n,
            FilesystemEntry::Directory(n, _, _) => n,
        }
    }

    fn parent_id(&self) -> usize {
        match self {
            FilesystemEntry::File(..) => {
                panic!("Getting parent of a file is not implemented (although it could be)")
            }
            FilesystemEntry::Directory(_, _, None) => {
                panic!("This directory has no parent. Is it the rood directory?")
            }
            FilesystemEntry::Directory(_, _, Some(id)) => *id,
        }
    }

    fn find_child_id(&self, name: &str, tree: &Vec<FilesystemEntry>) -> usize {
        match self {
            FilesystemEntry::File(..) => panic!("Cannot add child on file"),
            FilesystemEntry::Directory(dir_name, children, _) => {
                let error_message =
                    format!("No child directory of {0} is named {1}", dir_name, name);
                *children
                    .iter()
                    .filter(|&child_id| matches!(tree[*child_id], FilesystemEntry::Directory(..)))
                    .find(|&child_id| tree[*child_id].name() == name)
                    .expect(&error_message)
            }
        }
    }

    // see https://stackoverflow.com/questions/57437256/cannot-borrow-as-mutable-as-it-is-behind-a-reference for explaination
    fn add_child(&mut self, child_id: usize) {
        match self {
            FilesystemEntry::File(..) => panic!("Cannot add child on file"),
            FilesystemEntry::Directory(_, children, _) => children.push(child_id),
        }
    }
}
