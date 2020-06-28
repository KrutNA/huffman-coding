use crate::types::{node::*,
		   table::*,};

use std::collections::{BinaryHeap, HashMap};

const LEFT_BIT:  u8 = 0b1;
const RIGHT_BIT: u8 = 0b0;

pub fn generate_from_heap(heap: &mut BinaryHeap<Element>) -> Element {
    while heap.len() > 1 {
	let mut node  = Node::new();
	node.left(&mut heap.pop().unwrap());
	node.right(&mut heap.pop().unwrap());
	heap.push(Element::Node(Box::new(node)));
    }
    heap.pop().unwrap()
}

pub fn convert_to_table(element: &Element) -> HashMap<u8, Table> {
    let mut map = HashMap::new();

    to_table(element, &mut map, Table::new());

    map
}

pub fn to_table(
    element: &Element,
    map:     &mut HashMap<u8, Table>,
    table:   Table
) {
    match element {
	Element::Data(x) => {
	    map.insert(x.data,
		       if table.length == 0 { table.deeper(RIGHT_BIT) }
		       else { table }
	    );
	}
	Element::Node(x) => {
	    to_table(&x.left,  map, table.deeper(LEFT_BIT));
	    to_table(&x.right, map, table.deeper(RIGHT_BIT));
	}
	_ => (),
    }
}

