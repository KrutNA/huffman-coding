#[cfg(debug_assertions)]
use crate::tree;
use crate::types::{
    ui::CliArgs,
    node::{Element, Node, Data},
    BUFFER_SIZE,
};

use std::{
    fs::File,
    io::{Read, Write},
    slice::Iter,
    convert::TryInto,
};

#[cfg(feature = "time")]
use std::time::SystemTime;

pub fn decode(buffer: &mut Vec<u8>, args: &CliArgs) {    
    let mut ifile = File::open(&args.input).unwrap();
    let mut ofile = File::create(&args.output).unwrap();

    #[cfg(feature = "time")]
    let time = SystemTime::now();

    if let Ok(len) = ifile.read(buffer) {
	if len == 0 { return; }
    }

    let mut iter   = buffer.iter();
    let root       = read_node(&mut iter);
    #[cfg(debug_assertions)]
    let table      = tree::convert_to_table(&root);
    
    #[cfg(feature = "time")]
    println!("{}", SystemTime::now().duration_since(time).unwrap().as_secs_f32());

    let mut vec    = Vec::new();
    for _ in 0..8 {
	let &val   = iter.next().expect("Unexpected EOF");
	vec.push(val);
    }
    let count      = u64::from_le_bytes(vec.iter().map(|&x| x)
					.collect::<Vec<_>>().as_slice()
					.try_into().unwrap());

    #[cfg(debug_assertions)]
    {
	#[cfg(feature = "ptree")]
	ptree::print_tree(&root).unwrap();
	println!("{}", count);
	println!("{:#?}", table);
    }
    
    #[cfg(feature = "time")]
    let time = SystemTime::now();

    let (mut last, mut last_len) = (0, 0);
    let mut vec = Vec::new();
    
    for _ in 0..count {
	let mut element = &root;

	'find: loop {
	    if last_len == 0 {
		if let Some(&tmp) = iter.next() {
		    last = tmp;
		} else {
		    let len = ifile.read(buffer).unwrap();
		    if len == 0 {
			panic!("Unexpected EOF");
		    }
		    iter = buffer.iter();
		    last = *iter.next().unwrap();
		}
		last_len = 8;
	    }
	    
	    let bit = last & 0x80 != 0;
	    
	    match element {
		Element::Data(x) => { vec.push(x.data);
				      break 'find },
		Element::Node(x)
		    if bit  => element = &x.left,
		Element::Node(x) => element = &x.right,
		Element::None    => panic!("Unexpected None"),
	    }
	    last <<= 1;
	    last_len -= 1;
	};
	
	if vec.len() == BUFFER_SIZE {
	    let _ =  ofile.write(&vec.as_slice());
	    vec.clear();
	}
    }
    let _ =  ofile.write(&vec.as_slice());
    
    #[cfg(feature = "time")]
    println!("{}", SystemTime::now().duration_since(time).unwrap().as_secs_f32());
}

fn read_node(buffer_iter: &mut Iter<u8>) -> Element {
    if let Some(&data) = buffer_iter.next() {
	if data == b'\0' {
	    let mut iter = buffer_iter.clone();
	    
	    if iter.next() == Some(&b'\0') {
		buffer_iter.next().unwrap();
		
		let mut node = Node::new();
		node.left(&mut read_node(buffer_iter));
		node.right(&mut read_node(buffer_iter));

		return Element::Node(Box::new(node));
	    }
	}
	Element::Data(Data::new(data))
    } else {
	panic!("Unexpected EOF");
    }
}
