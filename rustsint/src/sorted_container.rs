struct Data<'a> {
    name: &'a str,
    age: u8,
}

struct Node<'a> {
	data: &'a Data,
	left: Option<Box<Node<'a>>>,
	right: Option<Box<Node<'a>>>,
}

struct Tree {
	root: Option<Box<Node<'a>>>,
}


fn compare(d1: &Data, d2: &Data) -> {
	if d1.age > 
}

