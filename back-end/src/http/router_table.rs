/// router_table mod implements the router feature of HTTP.
use crate::http;
use std::collections::HashMap;
use std::{cell::RefCell, rc::Rc};

/// The sub-path of router, for example, router "/path/to/x" will be spilt into sub-paths "path", "to" and "x".
#[derive(Debug)]
pub struct Node {
    value: String,
    /// If the node's value is the last sub-path of a router's path,
    /// router_path will be the router's path.
    /// Otherwise, router_path will be None.
    router_path: Option<String>,
    children: RefCell<Vec<Rc<Node>>>,
    method_table: RefCell<HashMap<String, http::HandlerFn>>,
}

pub struct RouterTable {
    root: Rc<Node>,
}

pub struct Handler {
    pub router_path: String,
    pub handler: http::HandlerFn,
}

impl Node {
    pub fn new(value: &str, router_path: Option<String>, children: RefCell<Vec<Rc<Node>>>) -> Self {
        Node {
            value: String::from(value),
            router_path,
            children,
            method_table: RefCell::new(HashMap::new()),
        }
    }

    /// get_handler gets the handler of the current node.
    pub fn get_handler(node: Rc<Node>, method: &str) -> Option<Handler> {
        if let Some(h) = node.method_table.borrow_mut().get(method) {
            if let Some(path) = &node.router_path {
                Some(Handler {
                    router_path: path.clone(),
                    handler: h.clone(),
                })
            } else {
                None
            }
        } else {
            None
        }
    }
}

impl RouterTable {
    pub fn new() -> Self {
        let root = Node::new("/", Some(String::from("/")), RefCell::new(Vec::new()));

        RouterTable {
            root: Rc::new(root),
        }
    }

    /// register registers the handler into router table.
    pub fn register(&mut self, path: &str, method: &str, handler: http::HandlerFn) {
        if path.len() == 0 || path == "/" {
            self.root
                .method_table
                .borrow_mut()
                .insert(String::from(method), handler);
        }

        let strs_ = path.split("/");
        let mut strs: Vec<String> = Vec::new();
        for s in strs_ {
            if s.len() > 0 {
                strs.push(s.to_string());
            }
        }

        let mut node = self.root.clone();
        let end = strs.len() - 1;
        let mut i = 0;

        for s in strs {
            let mut exist = false;
            let mut node_: Option<Rc<Node>> = None;

            for child in node.children.borrow_mut().iter() {
                if child.value == s {
                    exist = true;
                    node_ = Some(child.clone());
                    break;
                }
            }

            if exist {
                if let Some(n) = node_ {
                    node = n;
                }
            } else {
                let mut router_path = None;
                if i == end {
                    router_path = Some(String::from(path));
                }
                let new_child = Node::new(&s, router_path, RefCell::new(Vec::new()));
                let new_child = Rc::new(new_child);
                node.children.borrow_mut().push(new_child.clone());
                node = new_child.clone();
            }

            i += 1;
        }

        node.method_table
            .borrow_mut()
            .insert(String::from(method), handler);
    }

    /// get_handler gets the handler of the specified path and method.
    pub fn get_handler(&mut self, path: &str, method: &str) -> Option<Handler> {
        if path.len() == 0 || path == "/" {
            return Node::get_handler(self.root.clone(), method);
        }

        let strs_ = path.split("/");
        let mut strs: Vec<String> = Vec::new();
        for s in strs_ {
            if s.len() > 0 {
                strs.push(s.to_string());
            }
        }

        let mut node = self.root.clone();
        for s in strs {
            let mut exist = false;
            let mut node_: Option<Rc<Node>> = None; // node_ may be not initialized.

            for child in node.children.borrow_mut().iter() {
                if child.value == s {
                    exist = true;
                    node_ = Some(child.clone());
                    break;
                }
            }

            if !exist {
                return None;
            }

            if let Some(n) = node_ {
                node = n;
                if let Some(h) = Node::get_handler(node.clone(), method) {
                    return Some(h);
                }
            } else {
                return None;
            }
        }

        None
    }

    /// print the content of a Node.
    #[allow(dead_code)]
    pub fn print(&mut self, node: Option<Rc<Node>>) {
        match node {
            None => {}
            Some(n) => {
                println!("{}", n.value);
                for child in n.children.borrow_mut().iter() {
                    self.print(Some(child.clone()));
                }
            }
        }
    }
}
