// struct Parameter {
//     entity: String,
//     name: String,
//     value: String,
//     ns: String
// }

// struct Conf {
//     parameters: Vec<Parameter>
// }

#[derive(Debug, PartialEq)]
pub struct Node<T> {
    val: Option<T>,
    ns: String,
    childs: Vec<Box<Node<T>>>,
}

impl<T> Node<T>
where
    T: std::fmt::Debug,
{
    pub const CAPACITY: usize = 32;

    pub fn new(val: T, ns: &str) -> Self {
        Self {
            val: Some(val),
            ns: ns.to_owned(),
            childs: Vec::with_capacity(Self::CAPACITY),
        }
    }
    fn create_node(ns: &str) -> Self {
        Self {
            val: None,
            ns: ns.to_owned(),
            childs: Vec::with_capacity(Self::CAPACITY),
        }
    }
    fn _insert<'a>(&mut self, path: &mut dyn Iterator<Item = &str>) -> Option<&mut Self> {
        let ns = path.next();
        if let None = ns {
            return Some(self);
        }
        ns.and_then(
            |ns| match self.childs.iter().position(|node| node.ns == ns) {
                Some(i) => Self::_insert(&mut self.childs[i], path),
                None => {
                    self.childs.push(Box::new(Self::create_node(ns)));
                    self.childs.last_mut().and_then(|node| node._insert(path))
                }
            },
        )
    }
    pub fn insert(&mut self, val: T, path: &str) -> Option<&mut Self> {
        let mut path = path.split("::");
        let node = path
            .next()
            .and_then(|ns| {
                if self.ns == ns {
                    self._insert(&mut path)
                } else {
                    None
                }
            })
            .or_else(|| None);
        node.and_then(|n| {
            n.val = Some(val);
            Some(n)
        })
    }
    fn _get_mut(&mut self, path: &mut dyn Iterator<Item = &str>) -> Option<&mut Self> {
        let ns = path.next();
        if let None = ns {
            return Some(self);
        }
        ns.and_then(
            |ns| match self.childs.iter().position(|node| node.ns == ns) {
                Some(i) => Self::_get_mut(&mut self.childs[i], path),
                None => None,
            },
        )
    }
    pub fn get_mut(&mut self, path: &str) -> Option<&mut Self> {
        let mut path = path.split("::");
        path.next()
            .and_then(|ns| {
                if self.ns == ns {
                    self._get_mut(&mut path)
                } else {
                    None
                }
            })
            .or_else(|| None)
    }
    pub fn delete(&mut self, path: &str) -> Option<Box<Self>> {
        let mut path = path.split("::").collect::<Vec<&str>>();
        path.split_last()
            .and_then(|(ns, path)| {
                let path = path.join("::");
                self.get_mut(&path)
                    .and_then(|parent| {
                        parent
                            .childs
                            .iter()
                            .position(|node| node.ns == *ns)
                            .and_then(|i| Some(parent.childs.remove(i)))
                            .or_else(|| None)
                    })
                    .or_else(|| None)
            })
            .or_else(|| None)
    }
}

// export RUSTFLAGS="-A warnings"
// export RUSTFLAGS="--cap-lints allow"
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn init_and_get_root() {
        let path = "root";
        let mut root = Node::new(10, path);
        let root_ptr = &mut root as *mut Node<i32>;
        let root_get = unsafe { (&mut *root_ptr).get_mut(path) };
        assert_eq!(root_get, root.get_mut(path))
    }

    #[test]
    fn init_and_get_node() {
        let root = String::from("root");
        let tag = String::from("a");
        let path = format!("{}::{}", root, tag);
        let mut root = Node {
            val: Some(10),
            ns: root,
            childs: vec![Box::new(Node::create_node(&tag))],
        };
        assert_eq!(Some(&mut Node::create_node(&tag)), root.get_mut(&path))
    }

    #[test]
    fn init_insert_and_get_node() {
        let path = "root::a::b::c";
        let mut root = Node::new(10, "root");
        let root_ref = unsafe { (&mut *(&mut root as *mut Node<i32>)) };
        let insert = root.insert(100, path).unwrap();
        let get = root_ref.get_mut(path).unwrap();
        assert_eq!(insert, get);
        println!("Address of root.insert() = {:p}", insert);
        println!("Address of root.get() = {:p}", get);
    }

    #[test]
    fn init_insert_and_delete_node() {
        let path = "root::a::b::c";
        let mut root = Node::new(10, "root");
        let root_ref = unsafe { (&mut *(&mut root as *mut Node<i32>)) };
        let insert = root.insert(100, path).unwrap();
        let delete = root_ref.delete(path).unwrap();
        let delete = unsafe { &mut *Box::into_raw(delete) };
        assert_eq!(insert, delete);
        println!("Address of root.insert() = {:p}", insert);
        println!("Address of root.delete() = {:p}", delete);
    }
}

// Namespaces:
// ns r {
//     ns a {
//         ns b {
//             ns c {
//             }
//         }
//         ns w {
//             nw x {
//                 ns y {
//                     ns z {
//                     }
//                 }
//             }
//         }
//     }
//     ns b {
//         ns c {
//             ns d {
//                 ns e {
//                     ns f {
//                     }
//                 }
//             }
//             ns x {
//
//             }
//         }
//         ns q {
//             ns w {
//
//             }
//         }
//     }
// }
//
// r
// r::a
// r::a::b
// r::a::b::c
// r::a::w
// r::a::w::x
// r::a::w::x::y
// r::a::w::x::y::z
// r::b
// r::b::c
// r::b::c::d
// r::b::c::d::e
// r::b::c::d::e::f
// r::b::c::x
// r::b::q
// r::b::q::w
