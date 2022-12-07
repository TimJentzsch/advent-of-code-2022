use crate::utils::Day;

trait FileLike {
    fn name(&self) -> &str;
    fn size(&self) -> usize;
    fn get_by_name(&self, name: &str) -> Option<&Entry>;
    fn get_by_path(&self, path: &[&str]) -> Option<&Entry>;
}

#[derive(Debug)]
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

    fn get_by_name(&self, name: &str) -> Option<&Entry> {
        None
    }

    fn get_by_path(&self, path: &[&str]) -> Option<&Entry> {
        None
    }
}

#[derive(Debug)]
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

    fn add(&mut self, entry: Entry) {
        self.entires.push(entry);
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
}

#[derive(Debug)]
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
}
