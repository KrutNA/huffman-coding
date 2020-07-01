use std::cmp::Ordering;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Node {
    pub left:     Element,
    pub right:    Element,
    pub priority: u32,
}


#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Element {
    Node(Box<Node>),
    Data(Data),
    None,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Data {
    pub data:     u8,
    pub priority: u32,
}

pub trait Increment {
    fn inc(&mut self);
}

impl Data {
    pub fn new(data: u8) -> Data {
	Data { data,
	       priority: 0 }
    }
}

impl Increment for Data {
    fn inc(&mut self) {
	self.priority += 1;
    }
}

impl Node {
    pub fn new() -> Self {
	Self { priority: u32::default(),
	       left: Element::None,
	       right: Element::None }
    }

    pub fn left(&mut self, element: &mut Element) -> &mut Self {
	self.priority += element.priority();
	self.left = element.to_owned();
	self
    }

    pub fn right(&mut self, element: &mut Element) -> &mut Self {
	self.priority += element.priority();
	self.right = element.to_owned();
	self
    }
}

impl Increment for Node {
    fn inc(&mut self) {
	self.priority += 1;
    }
}

impl Element {
    pub fn new_with_priority(data: u8, priority: u32) -> Self {
	Self::Data(Data { data, priority })
    }

    pub fn priority(&mut self) -> u32 {
	match self {
	    Self::Data(x) => x.priority,
	    Self::Node(x) => x.priority,
	    Self::None    => 0,
	}
    }
}

impl Ord for Element {
    fn cmp(&self, other: &Self) -> Ordering {
	let self_data = match self {
	    Self::Node(x) => x.priority,
	    Self::Data(x) => x.priority,
	    Self::None    => 0,
	};
	let other_data = match other {
	    Self::Node(x) => x.priority,
	    Self::Data(x) => x.priority,
	    Self::None    => 0,
	};
	other_data.cmp(&self_data)
    }
}

impl PartialOrd for Element {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
	Some(self.cmp(other))
    }
}

#[cfg(feature = "ptree")]
impl ptree::TreeItem for Element {
    type Child = Self;

    fn write_self<W: std::io::Write>(
	&self, f: &mut W,
	style: &ptree::Style
    )
	-> std::io::Result<()>
    {
	match self {
	    Element::Data(x) => write!(f, "[{}: {}]",
				       style.paint(x.data as char),
				       style.paint(x.priority),),
	    Element::Node(x) => write!(f, "{}",
				       style.paint(x.priority)),
	    _ => Ok(()),
	}
    }

    fn children(&self) -> std::borrow::Cow<[Self::Child]> {
	let v = if let Element::Node(x) = self {
	    vec![x.left.clone(), x.right.clone()]
	} else {
	    Vec::new()
	};
	std::borrow::Cow::from(v)
    }
}
