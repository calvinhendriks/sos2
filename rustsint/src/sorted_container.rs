use std::cmp::Ordering;
use std::option::Option;

pub struct Data<> {
    pub name: String,
    pub age: i32,
}

pub struct Node<> {
	pub data: Box<Data<>>,
	pub left: Option<Box<Node<>>>,
	pub right: Option<Box<Node<>>>
}

pub struct Tree<> {
	pub root: Option<Box<Node<>>>
}


fn compare(d1: &Data, d2: &Data) -> Ordering {
	if d1.age < d2.age {return Ordering::Less;}
	if d1.age > d2.age {return Ordering::Greater;}
	return d1.name.cmp(&d2.name);

}

pub fn data_new(age: i32, name: String) -> Box<Data<>>{
	let mut d: Box<Data> = Box::new(Data {
		name: name,
		age: age
	});
	return d;
}

fn node_new(d1: Box<Data<>>) -> Box<Node<>>{
	let mut n: Box<Node> = Box::new(Node {
		data: d1,
		left: None,
		right: None
	});
	return n;
}

pub fn tree_new<>() -> Box<Tree<>>{
	let mut t: Box<Tree> = Box::new(Tree {
		root: None
	});
	return t;
}

pub fn node_insert(n1: Box<Node<>>, n2: Box<Node<>>){
    match (compare(&n1.data, &n2.data)) {
        Ordering::Less => {
            match n1.left{
                None => n1.left = Some(n2),
                Some(mut l) => node_insert(l,n2),
            }
        },
        Ordering::Greater => {
            match n1.right{
                None => n1.right = Some(n2),
                Some(mut r) => node_insert(r,n2),
            }
        },
        Ordering::Equal => {},
    }


}

pub fn tree_insert(t: &Box<Tree<>>, d: Box<Data<>>){
    let mut n = node_new(d);
    //let mut x = t.root.unwrap();
    node_insert(t.root.unwrap(), n);
}

fn print_node(n: Option<Box<Node<>>>, w: usize){
	match(n){
		Some(n) => {
			println!("{:width$} {}\n",n.data.name, n.data.age, width = w);
			print_node(n.left, w+5);
			print_node(n.right, w+5);
		},
		None => {
			println!("{:width$}nil\n", width = w);
		}
	}


}

pub fn print_tree(t: &Box<Tree<>>){
	print_node(t.root, 0);

}




