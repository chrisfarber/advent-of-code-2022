use std::{cell::RefCell, collections::HashMap, fs::File, io::Read, rc::Rc};

use aoc::shell_parse::{cd_cmd, commands, ls_cmd, ls_out_files};

fn main() {
    println!("day 7");

    let mut f = File::open("inputs/day7.txt").expect("couldn't open file");
    let mut text = String::new();
    f.read_to_string(&mut text)
        .expect("couldn't read all data into memory");

    let mut track = FileTracker::new();
    for run in commands(&text) {
        match Command::from(run.command) {
            Command::Cd(dir) => {
                track.cd(dir);
            }
            Command::Ls => {
                for (fname, size) in ls_out_files(run.output) {
                    track.record_file(fname, size);
                }
            }
            Command::Unknown => panic!("Unknown command encountered: {}", run.command),
        }
    }

    let sizes = track.collect_sizes();

    let solution: u64 = sizes.iter().filter(|s| **s <= 100000).sum();
    println!("the answer? {}", solution);

    let need_to_free = 30000000 - (70000000 - sizes[0]);
    println!("need to free {}", need_to_free);

    let mut sizes = sizes;
    sizes.sort();
    for size in sizes {
        if size >= need_to_free {
            println!("the second answer? {}", size);
            break;
        }
    }
}

enum Command<'a> {
    Cd(&'a str),
    Ls,
    Unknown,
}

impl<'a> Command<'a> {
    fn from(cmd_str: &'a str) -> Self {
        if let Some(dir) = cd_cmd(cmd_str) {
            Self::Cd(dir)
        } else if ls_cmd(cmd_str) {
            Self::Ls
        } else {
            Self::Unknown
        }
    }
}

#[derive(Debug)]
struct Dir<'a> {
    name: &'a str,
    subdirs: HashMap<&'a str, Rc<RefCell<Dir<'a>>>>,
    own_file_size: u64,
}

impl<'a> Dir<'a> {
    fn new(name: &'a str) -> Dir<'a> {
        Dir {
            name,
            subdirs: HashMap::new(),
            own_file_size: 0,
        }
    }

    fn find_or_create_dir(&mut self, subdir_name: &'a str) -> Rc<RefCell<Dir<'a>>> {
        if let Some(dir) = self.subdirs.get(subdir_name) {
            Rc::clone(dir)
        } else {
            let new_dir = Self::new(subdir_name);
            let r = Rc::new(RefCell::new(new_dir));
            self.subdirs.insert(subdir_name, Rc::clone(&r));
            r
        }
    }

    fn sizes(&self) -> Vec<u64> {
        let sizes: Vec<Vec<u64>> = self
            .subdirs
            .values()
            .map(|sd| sd.borrow().sizes())
            .collect();
        let direct_children_total: u64 = sizes.iter().map(|v| v[0]).sum();
        let mut out = vec![self.own_file_size + direct_children_total];
        for size in sizes.iter().flatten() {
            out.push(*size)
        }
        out
    }
}

#[derive(Debug)]
struct FileTracker<'a> {
    root: Rc<RefCell<Dir<'a>>>,
    dir_stack: Vec<Rc<RefCell<Dir<'a>>>>,
}

impl<'a> FileTracker<'a> {
    fn new() -> Self {
        let dir = Rc::new(RefCell::new(Dir::new("/")));
        Self {
            root: Rc::clone(&dir),
            dir_stack: Vec::new(),
        }
    }

    fn cd(&mut self, to: &'a str) {
        match to {
            "/" => {
                self.dir_stack.clear();
            }
            ".." => {
                self.dir_stack.pop();
            }
            child => {
                // let mut current_dir = (*self.root).borrow_mut();
                let mut current_dir = self.current_mut();
                let subdir = current_dir.find_or_create_dir(child);
                drop(current_dir);
                self.dir_stack.push(Rc::clone(&subdir));
            }
        }
    }

    fn current_mut(&mut self) -> std::cell::RefMut<Dir<'a>> {
        let current = self.dir_stack.last().unwrap_or(&self.root);
        let r = (**current).borrow_mut();
        r
    }

    fn record_file(&mut self, _name: &str, size: u64) {
        let mut dir = self.current_mut();
        dir.own_file_size += size;
    }

    fn collect_sizes(&self) -> Vec<u64> {
        self.root.borrow().sizes()
    }
}
