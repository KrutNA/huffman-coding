use crate::types::node::*;

use std::collections::BinaryHeap;

pub fn update_with_data(heap_buffer: &mut [u32], data: &[u8]) {
    for &byte in data.iter() {
	heap_buffer[byte as usize] += 1;
    }
}

pub fn convert_to_heap(
    heap_buffer: &mut [u32]
)
    -> BinaryHeap<Element>
{
    heap_buffer.iter().enumerate().filter_map(|(data, &priority)| {
	if priority > 0 { Some(Element::new_with_priority(data as u8, priority)) }
	else { None }
    }).collect()
}
