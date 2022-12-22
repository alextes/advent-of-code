//! You browse around the filesystem to assess the situation and save the resulting terminal output (your puzzle input). For example:
//!
//! ```
//! $ cd /
//! $ ls
//! dir a
//! 14848514 b.txt
//! 8504156 c.dat
//! dir d
//! $ cd a
//! $ ls
//! dir e
//! 29116 f
//! 2557 g
//! 62596 h.lst
//! $ cd e
//! $ ls
//! 584 i
//! $ cd ..
//! $ cd ..
//! $ cd d
//! $ ls
//! 4060174 j
//! 8033020 d.log
//! 5626152 d.ext
//! 7214296 k
//! ```
//! The filesystem consists of a tree of files (plain data) and directories (which can contain other directories or files). The outermost directory is called /. You can navigate around the filesystem, moving into or out of directories and listing the contents of the directory you're currently in.
//!
//! Within the terminal output, lines that begin with $ are commands you executed, very much like some modern computers:
//!
//! cd means change directory. This changes which directory is the current directory, but the specific result depends on the argument:
//! cd x moves in one level: it looks in the current directory for the directory named x and makes it the current directory.
//! cd .. moves out one level: it finds the directory that contains the current directory, then makes that directory the current directory.
//! cd / switches the current directory to the outermost directory, /.
//! ls means list. It prints out all of the files and directories immediately contained by the current directory:
//! 123 abc means that the current directory contains a file named abc with size 123.
//! dir xyz means that the current directory contains a directory named xyz.
//! Given the commands and output in the example above, you can determine that the filesystem looks visually like this:
//!
//! ```
//! - / (dir)
//!   - a (dir)
//!     - e (dir)
//!       - i (file, size=584)
//!     - f (file, size=29116)
//!     - g (file, size=2557)
//!     - h.lst (file, size=62596)
//!   - b.txt (file, size=14848514)
//!   - c.dat (file, size=8504156)
//!   - d (dir)
//!     - j (file, size=4060174)
//!     - d.log (file, size=8033020)
//!     - d.ext (file, size=5626152)
//!     - k (file, size=7214296)
//! ```
//! Here, there are four directories: / (the outermost directory), a and d (which are in /), and e (which is in a). These directories also contain files of various sizes.
//!
//! Since the disk is full, your first step should probably be to find directories that are good candidates for deletion. To do this, you need to determine the total size of each directory. The total size of a directory is the sum of the sizes of the files it contains, directly or indirectly. (Directories themselves do not count as having any intrinsic size.)
//!
//! The total sizes of the directories above can be found as follows:
//!
//! The total size of directory e is 584 because it contains a single file i of size 584 and no other directories.
//! The directory a has total size 94853 because it contains files f (size 29116), g (size 2557), and h.lst (size 62596), plus file i indirectly (a contains e which contains i).
//! Directory d has total size 24933642.
//! As the outermost directory, / contains every file. Its total size is 48381165, the sum of the size of every file.
//! To begin, find all of the directories with a total size of at most 100000, then calculate the sum of their total sizes. In the example above, these directories are a and e; the sum of their total sizes is 95437 (94853 + 584). (As in this example, this process can count files more than once!)
//!
//! Find all of the directories with a total size of at most 100000. What is the sum of the total sizes of those directories?

use std::{
    cell::RefCell,
    collections::HashMap,
    fmt::{Debug, Formatter},
    rc::Rc,
};

type DirectoryHandle = Rc<RefCell<Directory>>;

struct Directory {
    directories: HashMap<String, DirectoryHandle>,
    files: HashMap<String, u32>,
    name: String,
    parent: Option<DirectoryHandle>,
    size: u32,
    // Used purely for pretty debug logging
    depth: u32,
}

impl Directory {
    fn new(name: &str, parent: Option<DirectoryHandle>, depth: u32) -> Self {
        Directory {
            depth,
            directories: HashMap::new(),
            files: HashMap::new(),
            name: name.to_string(),
            parent,
            size: 0,
        }
    }

    fn add_file(&mut self, name: &str, size: u32) {
        self.files.insert(name.to_string(), size);
    }

    fn add_size(&mut self, size: u32) {
        // Add to our size, and all parents.
        self.size += size;
        if let Some(parent) = &self.parent {
            parent.borrow_mut().add_size(size);
        }
    }

    fn all_child_directories(&self) -> Vec<DirectoryHandle> {
        let mut directories = vec![];
        for (_, directory) in &self.directories {
            directories.push(directory.clone());
            directories.append(&mut directory.borrow().all_child_directories());
        }
        directories
    }
}

impl Debug for Directory {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "{}dir {} {} {}",
            " ".repeat((self.depth * 2).try_into().unwrap()),
            self.name,
            self.size,
            self.depth,
        )?;
        for (name, size) in &self.files {
            writeln!(
                f,
                "{}- {} {}",
                " ".repeat(((self.depth + 1) * 2).try_into().unwrap()),
                name,
                size
            )?;
        }

        for (_, dir) in &self.directories {
            writeln!(f, "{:?}", dir.borrow())?;
        }

        Ok(())
    }
}

enum Listing {
    File(String, u64),
    Directory(String),
}

enum Command {
    ChangeDirectory(String),
    ChangeDirectoryUp,
    List,
}

enum Instruction {
    Command(Command),
    Listing(Listing),
}

fn parse_line(line: &str) -> Instruction {
    let mut parts = line.split_whitespace();
    let first_part = parts.next().unwrap();
    match first_part {
        "$" => {
            let command = parts.next().unwrap();
            match command {
                "cd" => {
                    let argument = parts.next().unwrap();
                    if argument == ".." {
                        Instruction::Command(Command::ChangeDirectoryUp)
                    } else {
                        Instruction::Command(Command::ChangeDirectory(argument.to_string()))
                    }
                }
                "ls" => Instruction::Command(Command::List),
                _ => panic!("Unknown command: {}", command),
            }
        }
        _ => match first_part {
            "dir" => {
                let name = parts.next().unwrap();
                Instruction::Listing(Listing::Directory(name.to_string()))
            }
            _ => {
                let size = first_part.parse::<u64>().unwrap();
                let name = parts.next().unwrap();
                Instruction::Listing(Listing::File(name.to_string(), size))
            }
        },
    }
}

fn build_tree(input: &str) -> DirectoryHandle {
    let root = Directory::new("/", None, 0);
    let root_handle = Rc::new(RefCell::new(root));

    let mut current_directory: DirectoryHandle = root_handle.clone();

    // Step through each line in the input.
    // Skips the first line as it changes to the root directory which is already the current directory.
    for line in input.lines().skip(1) {
        // Parse each line into an instruction or a listing
        let instruction = parse_line(line);
        match instruction {
            Instruction::Command(command) => {
                match command {
                    Command::ChangeDirectoryUp => {
                        let parent = current_directory
                            .borrow()
                            .parent
                            .clone()
                            .expect("expect directory we're calling 'up' on to have a parent");
                        current_directory = parent;
                    }
                    Command::ChangeDirectory(directory_to_change_to) => {
                        // Find the directory in the current directory entries.
                        let directory = current_directory
                            .borrow()
                            .directories
                            .get(&directory_to_change_to)
                            .cloned()
                            .expect("expect directory to change to to exist");

                        // Set the current directory to the one we just found or created
                        current_directory = directory.clone();
                    }
                    Command::List => {
                        // We do nothing here. We only care about the listings.
                    }
                }
            }
            Instruction::Listing(listing) => match listing {
                Listing::File(name, size) => {
                    current_directory.borrow_mut().add_file(&name, size as u32);
                    current_directory.borrow_mut().add_size(size as u32);
                }
                Listing::Directory(name) => {
                    let new_directory = Directory::new(
                        &name,
                        Some(current_directory.clone()),
                        current_directory.borrow().depth + 1,
                    );
                    let new_directory_handle = Rc::new(RefCell::new(new_directory));
                    current_directory
                        .borrow_mut()
                        .directories
                        .insert(name, new_directory_handle.clone());
                }
            },
        }
    }

    root_handle
}

fn main() {
    let input = include_str!("../../input/day7.txt");

    let root_handle = build_tree(input);

    let mut total_size_under_one_hundred_thousand = 0;
    // Find all directories with a size of at most 100000
    let mut dirs = vec![];
    dirs.push(root_handle.clone());
    let child_dirs = root_handle.borrow().all_child_directories();
    dirs.append(&mut child_dirs.clone());
    for dir in dirs {
        if dir.borrow().size <= 100000 {
            total_size_under_one_hundred_thousand += dir.borrow().size;
        }
    }
    println!(
        "Total size under 100,000: {}",
        total_size_under_one_hundred_thousand
    );

    let total_size = root_handle.borrow().size;
    let total_space = 70000000;
    let unused_space = total_space - total_size;
    println!("Unused space: {}", unused_space);

    let needed_space = 30000000;
    let missing_space = needed_space - unused_space;
    println!("Missing space: {}", missing_space);

    // Find the smallest directory larger than mising_space.
    let mut smallest_directory: Option<DirectoryHandle> = None;
    for dir in child_dirs {
        if dir.borrow().size >= missing_space {
            if let Some(smallest) = &smallest_directory {
                if dir.borrow().size < smallest.borrow().size {
                    smallest_directory = Some(dir.clone());
                }
            } else {
                smallest_directory = Some(dir.clone());
            }
        }
    }

    println!(
        "Smallest directory size: {:?}",
        smallest_directory.unwrap().borrow().size
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one_test() {
        let input = include_str!("../../input/day7-example.txt");
        let root_handle = build_tree(input);

        // Assert total size is 48381165
        assert_eq!(root_handle.borrow().size, 48381165);
    }
}
