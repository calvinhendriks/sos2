use std::cmp::Ordering;

pub struct Data {
	pub name: String,
	pub age: i32,
}

impl Data {
	pub fn new(a: i32, n: String) -> Data {
		Data {
			name: n,
			age: a,
        }
	}
}

pub struct Node {
    pub left: Option<Box<Node>>,
    pub right: Option<Box<Node>>,
    pub data: Data,
}

impl Node {
	fn new(d: Data) -> Node {
		Node {
			data: d,
			left: None,
			right: None
		}
	}

	fn contains(&self, d: Data) -> Option<&Node> {
		match d.age.cmp(&self.data.age) {
			Ordering::Less => {
				if let Some(ref l) = self.left {
                    l.contains(d)
                } else {
                    return None
                }
			},
			Ordering::Greater => {
				if let Some(ref r) = self.right {
                    r.contains(d)
                } else {
                    return None
                }
			},
			Ordering::Equal => {
				match d.name.cmp(&self.data.name) {
					Ordering::Less => {
						if let Some(ref l) = self.left {
							l.contains(d)
						} else {
							None
						}
					},
					Ordering::Greater => {
						if let Some(ref r) = self.right {
							r.contains(d)
						} else {
							return None
						}
					},
					Ordering::Equal => {
						Some(&self)
					},
				}
			},
		}
	}

	fn insert(&mut self, n: Node) {
		match n.data.age.cmp(&self.data.age) {
			Ordering::Less => {
				if let Some(ref mut l) = self.left {
					l.insert(n);
				} else {
					self.left = Some(Box::new(n));
				}
			},
			Ordering::Greater => {
				if let Some(ref mut r) = self.right {
					r.insert(n);
				} else {
					self.right = Some(Box::new(n));
				}
			},
			Ordering::Equal => {
				match n.data.name.cmp(&self.data.name) {
					Ordering::Less => {
						if let Some(ref mut l) = self.left {
							l.insert(n);
						} else {
							self.left = Some(Box::new(n));
						}
					},
					Ordering::Greater => {
						if let Some(ref mut r) = self.right {
							r.insert(n);
						} else {
							self.right = Some(Box::new(n));
						}
					},
					Ordering::Equal => {}
				}
			}
		}
	}

	fn print_node(&self, w: usize) {
		println!("{} {} {}\n",(0..w).map(|_| "   ").collect::<String>(),self.data.age, self.data.name);
		if let Some(ref l) = self.left {
			l.print_node(w+1);
		} else {
			println!("{} {}\n",(0..w+1).map(|_| "   ").collect::<String>(), "nil");
		}
		if let Some(ref r) = self.right {
			r.print_node(w+1);
		} else {
			println!("{} {}\n",(0..w+1).map(|_| "   ").collect::<String>(), "nil");
		}	
	}
}

pub struct Tree {
	pub root: Option<Box<Node>>,
}

impl Tree {
	pub fn new() -> Tree {
		Tree { root: None ,
		}
	}

	pub fn insert(&mut self, d: Data) {
		match self.root {
			None => self.root = Some(Box::new(Node::new(d))),
			Some(ref mut r) => r.insert(Node::new(d)),
		}
	}

	pub fn contains(&self, d: Data) -> bool {
		match self.root {
			None => return false,
			Some(ref n) => {
				match n.contains(d) {
					None => return false,
					_ => return true,
				}
			},
		}
	}

	pub fn print_tree(&self) {
		match self.root {
			None => println!("nil\n"),
			Some(ref n) => {
				n.print_node(0)
			}
		}
	}
}