use std::{cell::RefCell, fmt, rc::Rc};
use crate::lexer::Lexer;
use crate::utils::get_input;

pub fn part1() -> usize {
    let fs = get_file_system();

    let mut dir_sizes_sum = 0;

    visit_dir_sizes(fs.root(), &mut |size| {
        if size <= 100000 { 
            dir_sizes_sum += size; 
        }
    });

    dir_sizes_sum
}

pub fn part2() -> usize {
    let fs = get_file_system();
    
    const TOTAL_DISK_SPACE: usize = 70000000;
    const REQUIRED_DISK_SPACE: usize = 30000000;
    let total_size = fs.root().borrow().size();
    let size_needed = REQUIRED_DISK_SPACE - (TOTAL_DISK_SPACE - total_size);
    let mut smallest_valid_size_found = usize::MAX;

    visit_dir_sizes(fs.root(), &mut |size| {
        if size > size_needed {
            smallest_valid_size_found = smallest_valid_size_found.min(size)
        }
    });
    
    smallest_valid_size_found
}
    
fn visit_dir_sizes<F: FnMut(usize)>(node: Rc<RefCell<Node>>, f: &mut F) {
    let node = node.borrow();
    if let Node::Dir(d) = &*node {
        f(node.size());
        d.children.iter().for_each(|c| visit_dir_sizes(c.clone(), f));
    }
}

#[derive(Debug)]
struct File {
    name: String, 
    size: usize,
}

#[derive(Debug)]
struct Dir {
    name: String,
    children: Vec<Rc<RefCell<Node>>>,
    parent: Option<Rc<RefCell<Node>>>,
}

#[derive(Debug)]
enum Node {
    File(File),
    Dir(Dir),
}

impl Node {
    fn as_dir(&mut self) -> &mut Dir {
        match self {
            Node::Dir(ref mut d) => d,
            _ => panic!(),
        }
    }

    fn size(&self) -> usize {
        match self {
            Node::File(f) => f.size,
            Node::Dir(d) => d.children.iter().map(|c| c.borrow().size()).sum()
        }
    }
}

struct FileSystem {
    root_node: Rc<RefCell<Node>>,
    active_directory: Rc<RefCell<Node>>,
}

impl FileSystem {
    fn new() -> Self {
        let root_node = Rc::new(RefCell::new(Node::Dir(Dir { 
            name: "/".into(), 
            children: vec![],
            parent: None,
        })));

        FileSystem { 
            root_node: root_node.clone(), 
            active_directory: root_node,
        }
    }

    fn root(&self) -> Rc<RefCell<Node>> {
        self.root_node.clone()
    }

    fn active_directory(&self) -> Rc<RefCell<Node>> {
        self.active_directory.clone()
    }

    fn insert(&mut self, node: Node) {
        self.active_directory.borrow_mut().as_dir().children.push(Rc::new(RefCell::new(node)));
    }

    fn cd(&mut self, name: Option<&str>) {
        match name {
            None => { // cd ..
                let active_dir_parent = self.active_directory.borrow_mut().as_dir().parent.clone();
                match active_dir_parent {
                    None => panic!("Tried to \"cd ..\" out of the root node!"),
                    Some(parent) => self.active_directory = parent,
                }
            }
            Some(name) => {
                let active_directory = self.active_directory.clone();
                let mut active_directory = active_directory.borrow_mut();
                let new_active_dir = active_directory.as_dir().children.iter().find(|c| {
                    match &*c.borrow() {
                        Node::File(..) => false,
                        Node::Dir(d) => d.name == name,
                    }
                });
                self.active_directory = new_active_dir.expect(&format!("Failed to find directory named {}", name)[..]).clone();
            }
        }
    }
}

fn get_file_system() -> FileSystem {
    let mut lexer = Lexer::new(get_input(7));

    //  Skip the initial "$ cd /" command
    lexer.skip_until('\n');
    lexer.advance();

    let mut file_system = FileSystem::new();

    while !lexer.done() {
        assert!(lexer.consume_str("$ "));

        if lexer.consume_str("ls") {
            lexer.skip_whitespace();
            while !lexer.done() && !lexer.matches_str("$ ") {
                if lexer.consume_str("dir ") {
                    file_system.insert(Node::Dir(Dir {
                        name: lexer.consume_word().unwrap(),
                        children: vec![],
                        parent: Some(file_system.active_directory()),
                    }));
                } else {
                    let size = lexer.consume_number::<usize>(10).unwrap();
                    lexer.skip_whitespace();
                    
                    file_system.insert(Node::File(File {
                        name: lexer.consume_word().unwrap(),
                        size,
                    }))
                }

                lexer.skip_whitespace();
            }
        } else if lexer.consume_str("cd ") {
            if lexer.consume_str("..") {
                file_system.cd(None);
            } else {
                file_system.cd(Some(&lexer.consume_word().unwrap()));
            }
            lexer.skip_whitespace();
        }
    }

    file_system
}

//////////////////
// DISPLAY IMPL //
//////////////////

impl fmt::Display for FileSystem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt_node(self.root_node.clone(), 0, f)
    }
}

fn fmt_node(node: Rc<RefCell<Node>>, indent: usize, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{:indent$}", "", indent=indent * 2)?;
    match &*node.borrow() {
        Node::File(file) => writeln!(f, "{} (size = {})", file.name, file.size)?,
        Node::Dir(dir) => {
            writeln!(f, "{}", dir.name)?;
            for child in &dir.children {
                fmt_node(child.clone(), indent + 1, f)?;
            }
        }
    }
    fmt::Result::Ok(())
}
