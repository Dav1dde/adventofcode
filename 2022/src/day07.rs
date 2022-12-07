use aoc2022::Input;
use std::{
    cell::RefCell,
    cmp::Reverse,
    collections::{BTreeMap, BinaryHeap},
    io::BufRead,
    rc::Rc,
};

#[derive(Default)]
struct Directory {
    children: RefCell<BTreeMap<String, Entry>>,
}

impl Directory {
    fn get_or_create_directory(&self, name: &str) -> Rc<Directory> {
        let mut children = self.children.borrow_mut();
        let entry = children
            .entry(name.to_owned())
            .or_insert_with(|| Entry::Directory(Default::default()));
        let Entry::Directory(directory) = entry else {
            panic!("tried to create a directory but a file with such name already exists") 
        };
        Rc::clone(directory)
    }

    fn add_file(&self, name: &str, size: usize) {
        let mut children = self.children.borrow_mut();
        children.insert(name.to_owned(), Entry::File { size });
    }
}

#[allow(dead_code)]
fn print_dir(directory: &Directory) {
    fn print(directory: &Directory, depth: usize) {
        for (name, c) in directory.children.borrow().iter() {
            print!("{}{name}", " ".repeat(depth * 2));
            match c {
                Entry::File { size } => println!(" {size}"),
                Entry::Directory(inner) => {
                    println!();
                    print(inner.as_ref(), depth + 1)
                }
            }
        }
    }

    println!("/");
    print(directory, 1);
}

enum Entry {
    File { size: usize },
    Directory(Rc<Directory>),
}

#[derive(Default)]
struct Filesystem {
    root: Rc<Directory>,
}

struct Cursor {
    cwd: Vec<Rc<Directory>>,
}

impl Cursor {
    fn new(fs: &Filesystem) -> Self {
        Cursor {
            cwd: vec![Rc::clone(&fs.root)],
        }
    }

    fn reset(&mut self) {
        self.cwd.truncate(1);
    }

    fn cd(&mut self, segment: &str) {
        match segment {
            ".." => {
                // Can't cd out of root
                if self.cwd.len() > 1 {
                    self.cwd.pop();
                }
            }
            "." => (),
            segment => {
                let current = self.cwd.last().unwrap();
                let sub = current.get_or_create_directory(segment);
                self.cwd.push(sub);
            }
        }
    }

    fn add_dir(&self, name: &str) {
        let current = self.cwd.last().unwrap();
        current.get_or_create_directory(name);
    }

    fn add_file(&self, name: &str, size: usize) {
        let current = self.cwd.last().unwrap();
        current.add_file(name, size);
    }
}

fn build_fs(fs: &Filesystem, reader: Input) {
    let mut cursor = Cursor::new(fs);

    let mut in_ls = false;
    for line in reader.lines() {
        let line = line.unwrap();

        if line.starts_with('$') {
            in_ls = false;
        }

        if let Some(path) = line.strip_prefix("$ cd ") {
            if path.starts_with('/') {
                cursor.reset();
            }

            for segment in path.split('/').filter(|s| !s.is_empty()) {
                cursor.cd(segment);
            }
        } else if line == "$ ls" {
            in_ls = true;
        } else if in_ls {
            let (dir_or_size, name) = line.split_once(' ').expect("invalid file listing in ls");
            if dir_or_size == "dir" {
                cursor.add_dir(name);
            } else {
                let size = dir_or_size.parse().unwrap();
                cursor.add_file(name, size);
            }
        }
    }
}

trait Visitor {
    type Acc: Default;

    fn visit(&mut self, dir: &Directory, acc: &Self::Acc);
    fn calculate(&self, entry: &Entry) -> Option<Self::Acc>;
    fn acc(&mut self, a: Self::Acc, b: Self::Acc) -> Self::Acc;
}

fn dfs<T: Visitor>(root: &Directory, visitor: &mut T) -> T::Acc {
    fn dfs_inner<T: Visitor>(dir: &Directory, visitor: &mut T) -> T::Acc {
        let mut result = T::Acc::default();

        for child in dir.children.borrow().values() {
            if let Entry::Directory(dir) = child {
                let a = dfs_inner(dir, visitor);
                result = visitor.acc(result, a);
            }
            if let Some(value) = visitor.calculate(child) {
                result = visitor.acc(result, value);
            }
        }

        visitor.visit(dir, &result);

        result
    }

    dfs_inner(root, visitor)
}

pub fn part1(reader: Input) -> anyhow::Result<usize> {
    let fs = Filesystem::default();
    build_fs(&fs, reader);

    struct SizeVisitor(usize);
    impl Visitor for SizeVisitor {
        type Acc = usize;

        fn visit(&mut self, _: &Directory, acc: &Self::Acc) {
            if *acc <= 100000 {
                self.0 += *acc;
            }
        }

        fn calculate(&self, entry: &Entry) -> Option<Self::Acc> {
            if let &Entry::File { size } = entry {
                Some(size)
            } else {
                None
            }
        }

        fn acc(&mut self, a: Self::Acc, b: Self::Acc) -> Self::Acc {
            a + b
        }
    }

    let mut visitor = SizeVisitor(0);
    dfs(&fs.root, &mut visitor);

    Ok(visitor.0)
}

pub fn part2(reader: Input) -> anyhow::Result<usize> {
    let fs = Filesystem::default();
    build_fs(&fs, reader);

    const MAX_SPACE: usize = 70000000;
    const REQUIRED_SPACE: usize = 30000000;

    #[derive(Default)]
    struct DeleteVisitor(BinaryHeap<Reverse<usize>>);
    impl Visitor for DeleteVisitor {
        type Acc = usize;

        fn visit(&mut self, _: &Directory, acc: &Self::Acc) {
            self.0.push(Reverse(*acc))
        }

        fn calculate(&self, entry: &Entry) -> Option<Self::Acc> {
            if let &Entry::File { size } = entry {
                Some(size)
            } else {
                None
            }
        }

        fn acc(&mut self, a: Self::Acc, b: Self::Acc) -> Self::Acc {
            a + b
        }
    }

    let mut visitor = DeleteVisitor::default();

    let total = dfs(&fs.root, &mut visitor);
    let current_free = MAX_SPACE - total;

    while let Some(Reverse(dir)) = visitor.0.pop() {
        if current_free + dir >= REQUIRED_SPACE {
            return Ok(dir);
        }
    }

    unreachable!()
}

pub fn main() {
    aoc2022::cli::run(part1, part2).unwrap();
}
