use std::cmp::Ordering;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Node {
    pub left:     Option<Element>,
    pub right:    Option<Element>,
    pub priority: u64,
}


#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Element {
    Node(Box<Node>),
    Data(Data),
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Data {
    pub data:     u8,
    pub priority: u64,
}

pub trait Increment {
    fn inc(&mut self);
}

impl Data {
    pub fn new(data: u8) -> Self {
	Data { data,
	       priority: u64::default() }
    }
}

impl Increment for Data {
    fn inc(&mut self) {
	self.priority += 1;
    }
}

impl Node {
    pub fn new() -> Self {
	Self { priority: u64::default(),
	       left: None,
	       right: None }
    }

    pub fn left(&mut self, element: &mut Element) -> &mut Self {
	self.left = Some(element.to_owned());
	self
    }

    pub fn right(&mut self, element: &mut Element) -> &mut Self {
	self.right = Some(element.to_owned());
	self
    }
}

impl Increment for Node {
    fn inc(&mut self) {
	self.priority += 1;
    }
}

impl Ord for Element {
    fn cmp(&self, other: &Self) -> Ordering {
	let self_data = match self {
	    Self::Node(x) => x.priority,
	    Self::Data(x) => x.priority,
	};
	let other_data = match other {
	    Self::Node(x) => x.priority,
	    Self::Data(x) => x.priority,
	};
	self_data.cmp(&other_data)
    }
}

impl PartialOrd for Element {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
	Some(self.cmp(other))
    }
}
