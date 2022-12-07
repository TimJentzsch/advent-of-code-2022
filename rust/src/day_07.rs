use std::fmt::Debug;

use crate::utils::Day;

trait FileLike {
    fn name(&self) -> &str;
    fn size(&self) -> usize;
    fn get_by_name(&self, name: &str) -> Option<&Entry>;
    fn get_by_path(&self, path: &[&str]) -> Option<&Entry>;
    fn get_mut_by_name(&mut self, name: &str) -> Option<&mut Entry>;
    fn get_mut_by_path(&mut self, path: &[&str]) -> Option<&mut Entry>;
    fn pretty_print(&self, indent: usize) -> String;
}

#[derive(Clone, PartialEq, Eq)]
struct File {
    name: String,
    size: usize,
}

impl File {
    fn new(name: &str, size: usize) -> Self {
        Self {
            name: name.to_string(),
            size,
        }
    }
}

impl FileLike for File {
    fn name(&self) -> &str {
        &self.name
    }

    fn size(&self) -> usize {
        self.size
    }

    fn get_by_name(&self, _name: &str) -> Option<&Entry> {
        None
    }

    fn get_by_path(&self, _path: &[&str]) -> Option<&Entry> {
        None
    }

    fn get_mut_by_name(&mut self, _name: &str) -> Option<&mut Entry> {
        None
    }

    fn get_mut_by_path(&mut self, _path: &[&str]) -> Option<&mut Entry> {
        None
    }

    fn pretty_print(&self, indent: usize) -> String {
        format!(
            "{}- {} (file, size={})",
            "  ".repeat(indent),
            self.name(),
            self.size()
        )
    }
}

impl Debug for File {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.pretty_print(0))
    }
}

#[derive(PartialEq, Eq)]
struct Dir {
    name: String,
    entires: Vec<Entry>,
}

impl Dir {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            entires: Vec::new(),
        }
    }

    fn with_entries(mut self, entries: Vec<Entry>) -> Self {
        self.entires = entries;
        self
    }

    fn create(&mut self, entry: Entry) {
        self.entires.push(entry);
    }

    fn create_if_not_exists(&mut self, entry: Entry) {
        if self.get_by_name(entry.name()).is_none() {
            self.create(entry);
        }
    }
}

impl FileLike for Dir {
    fn name(&self) -> &str {
        &self.name
    }

    fn size(&self) -> usize {
        self.entires.iter().map(|entry| entry.size()).sum()
    }

    fn get_by_name(&self, name: &str) -> Option<&Entry> {
        self.entires.iter().find(|entry| entry.name() == name)
    }

    fn get_by_path(&self, path: &[&str]) -> Option<&Entry> {
        if let Some(first) = path.first() && let Some(entry) = self.get_by_name(first) {
            if path.len() == 1 {
                Some(entry)
            } else {
                entry.get_by_path(&path[1..])
            }
        } else {
            None
        }
    }

    fn get_mut_by_name(&mut self, name: &str) -> Option<&mut Entry> {
        self.entires.iter_mut().find(|entry| entry.name() == name)
    }

    fn get_mut_by_path(&mut self, path: &[&str]) -> Option<&mut Entry> {
        if let Some(first) = path.first() && let Some(entry) = self.get_mut_by_name(first) {
            if path.len() == 1 {
                Some(entry)
            } else {
                entry.get_mut_by_path(&path[1..])
            }
        } else {
            None
        }
    }

    fn pretty_print(&self, indent: usize) -> String {
        let mut result = format!("{}- {} (dir)", "  ".repeat(indent), self.name());

        if !self.entires.is_empty() {
            result += "\n";
            result += &self
                .entires
                .iter()
                .map(|entry| entry.pretty_print(indent + 1))
                .intersperse("\n".to_string())
                .collect::<String>();
        }

        result
    }
}

impl Debug for Dir {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.pretty_print(0))
    }
}

#[derive(PartialEq, Eq)]
enum Entry {
    File(File),
    Dir(Dir),
}

impl FileLike for Entry {
    fn name(&self) -> &str {
        match self {
            Entry::File(file) => file.name(),
            Entry::Dir(dir) => dir.name(),
        }
    }

    fn size(&self) -> usize {
        match self {
            Entry::File(file) => file.size(),
            Entry::Dir(dir) => dir.size(),
        }
    }

    fn get_by_name(&self, name: &str) -> Option<&Entry> {
        match self {
            Entry::File(file) => file.get_by_name(name),
            Entry::Dir(dir) => dir.get_by_name(name),
        }
    }

    fn get_by_path(&self, path: &[&str]) -> Option<&Entry> {
        match self {
            Entry::File(file) => file.get_by_path(path),
            Entry::Dir(dir) => dir.get_by_path(path),
        }
    }

    fn get_mut_by_name(&mut self, name: &str) -> Option<&mut Entry> {
        match self {
            Entry::File(file) => file.get_mut_by_name(name),
            Entry::Dir(dir) => dir.get_mut_by_name(name),
        }
    }

    fn get_mut_by_path(&mut self, path: &[&str]) -> Option<&mut Entry> {
        match self {
            Entry::File(file) => file.get_mut_by_path(path),
            Entry::Dir(dir) => dir.get_mut_by_path(path),
        }
    }

    fn pretty_print(&self, indent: usize) -> String {
        match self {
            Entry::File(file) => file.pretty_print(indent),
            Entry::Dir(dir) => dir.pretty_print(indent),
        }
    }
}

impl Debug for Entry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.pretty_print(0))
    }
}

pub struct Day07;

impl Day for Day07 {
    fn identifier(&self) -> &'static str {
        "07"
    }

    fn run(&self) {
        let input = self.get_input();

        println!("Part 1: {}", part_1(&input));
        println!("Part 2: {}", part_2(&input));
    }
}

fn parse_file_tree(input: &str) -> Dir {
    let mut cur_path: Vec<&str> = Vec::new();
    let mut root = Dir::new("/");
    let mut cwd = &mut root;

    let mut lines = input.lines().peekable();

    while let Some(line) = lines.next() {
        let mut tokens = line.split_ascii_whitespace();

        match tokens.next().unwrap() {
            // A command by the user
            "$" => match tokens.next().unwrap() {
                // Changing the directory
                "cd" => {
                    match tokens.next().unwrap() {
                        "/" => {
                            cur_path = Vec::new();
                            cwd = &mut root;
                        }
                        ".." => {
                            cur_path.pop();

                            cwd = if cur_path.is_empty() {
                                &mut root
                            } else if let Entry::Dir(dir) = root
                                .get_mut_by_path(&cur_path)
                                .unwrap_or_else(|| panic!("Cannot navigate to path {cur_path:?}"))
                            {
                                dir
                            } else {
                                panic!("Expected directory, found file");
                            };
                        }
                        name => {
                            cur_path.push(name);
                            cwd = if let Entry::Dir(dir) = cwd
                                .get_mut_by_name(name)
                                .unwrap_or_else(|| panic!("Entry {name} not found in cwd"))
                            {
                                dir
                            } else {
                                panic!("Expected directory, found file");
                            };
                        }
                    };
                }
                // List the entries in the current directory
                "ls" => {
                    // Create all entries in the directory if they don't exist yet
                    while let Some(next_line) = lines.peek() {
                        // If the next command starts resume parsing
                        if next_line.starts_with('$') {
                            break;
                        }

                        let mut ls_tokens = next_line.split_ascii_whitespace();

                        let entry = match ls_tokens.next().expect("Expected dir or file") {
                            "dir" => {
                                let name = ls_tokens.next().expect("Expected dir name");
                                Entry::Dir(Dir::new(name))
                            }
                            size_str => {
                                let size = size_str.parse().expect("Invalid file size");
                                let name = ls_tokens.next().expect("Expected file name");
                                Entry::File(File::new(name, size))
                            }
                        };

                        cwd.create_if_not_exists(entry);
                        lines.next();
                    }
                }
                cmd => panic!("Invalid command '{cmd}'"),
            },
            token => panic!("Invalid token '{token}'"),
        }
    }

    root
}

fn part_1(_input: &str) -> usize {
    0
}

fn part_2(_input: &str) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &'static str = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";

    #[test]
    fn should_parse_file_tree() {
        let actual = parse_file_tree(EXAMPLE_INPUT);

        let expected = Dir::new("/").with_entries(vec![
            Entry::Dir(Dir::new("a").with_entries(vec![
                Entry::Dir(Dir::new("e").with_entries(vec![Entry::File(File::new("i", 584))])),
                Entry::File(File::new("f", 29116)),
                Entry::File(File::new("g", 2557)),
                Entry::File(File::new("h.lst", 62596)),
            ])),
            Entry::File(File::new("b.txt", 14848514)),
            Entry::File(File::new("c.dat", 8504156)),
            Entry::Dir(Dir::new("d").with_entries(vec![
                Entry::File(File::new("j", 4060174)),
                Entry::File(File::new("d.log", 8033020)),
                Entry::File(File::new("d.ext", 5626152)),
                Entry::File(File::new("k", 7214296)),
            ])),
        ]);

        assert_eq!(actual, expected);
    }
}
