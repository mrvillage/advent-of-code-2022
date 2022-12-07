use std::{
    hash::Hash,
    sync::{Arc, Mutex},
};

#[derive(Debug)]
struct Dir<'a> {
    pub name: &'a str,
    pub files: Vec<Arc<Mutex<File<'a>>>>,
    pub parent: Option<Arc<Mutex<Dir<'a>>>>,
    pub children: Vec<Arc<Mutex<Dir<'a>>>>,
}

impl PartialEq for Dir<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Eq for Dir<'_> {}

impl Hash for Dir<'_> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

impl Dir<'_> {
    pub fn size(&self) -> u32 {
        self.files
            .iter()
            .map(|i| i.lock().unwrap().size)
            .sum::<u32>()
            + self
                .children
                .iter()
                .map(|i| i.lock().unwrap().size())
                .sum::<u32>()
    }

    pub fn size_under_100k(&self) -> u32 {
        let children_size = self
            .children
            .iter()
            .map(|i| i.lock().unwrap().size_under_100k())
            .sum::<u32>();
        let size = self.size();
        if size <= 100_000 {
            size + children_size
        } else {
            children_size
        }
    }

    pub fn sizes(&self, required: u32) -> Vec<u32> {
        let mut sizes = self
            .children
            .iter()
            .flat_map(|i| i.lock().unwrap().sizes(required))
            .collect::<Vec<_>>();
        sizes.push(self.size());
        sizes.into_iter().filter(|i| *i >= required).collect()
    }
}

#[derive(Debug)]
struct File<'a> {
    pub name: &'a str,
    pub size: u32,
}

impl PartialEq for File<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Eq for File<'_> {}

impl Hash for File<'_> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

fn main() {
    let input = include_str!("input.txt");
    let root = Arc::new(Mutex::new(Dir {
        name: "/",
        files: vec![],
        parent: None,
        children: vec![],
    }));
    let mut current_dir: Arc<Mutex<Dir<'_>>> = root.clone();
    input.split("$ ").for_each(|l| {
        if l.starts_with("cd") {
            let name = l.split(' ').nth(1).unwrap().strip_suffix("\r\n").unwrap();
            if name == "/" {
                current_dir = root.clone();
            } else if name == ".." {
                let lock = current_dir.lock().unwrap();
                if let Some(parent) = &lock.parent {
                    let parent = parent.clone();
                    drop(lock);
                    current_dir = parent;
                }
            } else {
                let dir = {
                    let lock = current_dir.lock().unwrap();
                    lock.children
                        .iter()
                        .find(|i| i.lock().unwrap().name == name)
                        .cloned()
                };
                if let Some(dir) = dir {
                    current_dir = dir.clone();
                } else {
                    let dir = Arc::new(Mutex::new(Dir {
                        name,
                        files: vec![],
                        parent: Some(current_dir.clone()),
                        children: vec![],
                    }));
                    current_dir.lock().unwrap().children.push(dir);
                }
            }
        } else if l.starts_with("ls") {
            for i in l.lines().skip(1) {
                let mut f = i.split(' ');
                if i.starts_with("dir") {
                    let name = f.nth(1).unwrap();
                    let dir = Arc::new(Mutex::new(Dir {
                        name,
                        files: vec![],
                        parent: Some(current_dir.clone()),
                        children: vec![],
                    }));
                    current_dir.lock().unwrap().children.push(dir);
                } else {
                    let size = f.next().unwrap().parse::<u32>().unwrap();
                    let name = f.next().unwrap();
                    let file = Arc::new(Mutex::new(File { name, size }));
                    current_dir.lock().unwrap().files.push(file);
                }
            }
        }
    });

    let lock = root.lock().unwrap();
    println!("Part One: {}", lock.size_under_100k());

    let unused = 70_000_000 - lock.size();
    println!(
        "Part Two: {}",
        lock.sizes(30_000_000 - unused)
            .iter()
            .min()
            .cloned()
            .unwrap_or(0)
    );
}
