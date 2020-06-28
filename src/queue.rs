use crate::types::node::*;
use std::collections::BinaryHeap;

pub fn generate_from_data(data: &Vec<u8>) -> BinaryHeap<Element> {
    let mut heap = Vec::new();
    for &byte in data.iter() {
	if let Some(i) = (0..heap.len()).into_iter()
	    .find(|&x| if let Element::Data(x) = heap[x] { x.data == byte } else { false }) {
	    if let Some(Element::Data(x)) = heap.get_mut(i) {
		x.inc();
	    }
	} else {
	    heap.push(Element::Data(Data::new(byte)));
	}
    }

    heap.into()
}
