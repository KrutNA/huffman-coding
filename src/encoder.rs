use crate::convert;
use crate::tree;
use crate::queue;
use crate::types::{
    ui::CliArgs,
    data::Data,
    node::Element,
    BYTE_SIZE_U8
};

#[cfg(feature = "time")]
use std::time::SystemTime;

use std::{fs::File,
	  io::{Read, Write}};

const BYTE_COUNT: usize = 256;

pub fn encode(buffer: &mut Vec<u8>, args: &CliArgs) {
    let heap_buffer = &mut [0; BYTE_COUNT];
    
    let mut file   = File::open(&args.input).unwrap();
    let mut readed = 0u64;
    
    #[cfg(feature = "time")]
    let time = SystemTime::now();
    
    while let Ok(len) = file.read(buffer) {
	if len == 0 { break; }
	readed += len as u64;
	queue::update_with_data(heap_buffer, &buffer[..len]);
    }
    
    #[cfg(feature = "time")]
    println!("{}", SystemTime::now().duration_since(time).unwrap().as_secs_f32());
    
    #[cfg(feature = "time")]
    let time = SystemTime::now();
    
    let mut heap   = queue::convert_to_heap(heap_buffer);
    let     root   = tree::generate_from_heap(&mut heap);
    let     table  = tree::convert_to_table(&root);
    
    #[cfg(feature = "time")]
    println!("{}", SystemTime::now().duration_since(time).unwrap().as_secs_f32());
    
    #[cfg(feature = "time")]
    let time = SystemTime::now();
    
    let mut ifile  = File::open(&args.input).unwrap();
    let mut ofile  = File::create(&args.output).unwrap();
    let mut _writed = 0;

    let mut last   = 0;
    let mut length = 0;
    let mut data   = Data::default();
    
    let _ = ofile.write(encode_tree(&root).as_slice());
    let _ = ofile.write(&readed.to_le_bytes());

    while let Ok(len) = ifile.read(buffer) {
	if len == 0 { break; }
	
	data = Data {
	    data: if length == 0 {
		Vec::new()
	    } else {
		vec![last]
	    }, last: length
	};

	convert::encode_by_map(&mut data, &buffer[..len], &table);

	let count = if data.last % BYTE_SIZE_U8 == 0 {
	    last = 0;
	    length = 0;
	    data.data.len()
	} else {
	    last = *data.data.last().unwrap();
	    length = data.last;
	    data.data.len() - 1
	};

	let count = ofile.write(&data.data.as_slice()[..count]).unwrap();
	_writed += count;
    }
    if length % BYTE_SIZE_U8 != 0 {
	_writed += ofile.write(
	    &[*data.data.last().unwrap()]
	).unwrap()
    }
    
    #[cfg(feature = "time")]
    println!("{}", SystemTime::now().duration_since(time).unwrap().as_secs_f32());

    #[cfg(debug_assertions)]
    {
	#[cfg(feature = "ptree")]
	ptree::print_tree(&root).unwrap();
	println!("{:#?}", table);
	println!("r: {} / w: {}", readed, _writed);
    }
}

fn encode_tree(tree: &Element) -> Vec<u8> {
    let mut vec = Vec::new();
    encode_node(&mut vec, &tree);
    vec
}

fn encode_node(vec: &mut Vec<u8>, node: &Element) {
    match node {
	Element::Data(x) => vec.push(x.data),
	Element::Node(x) => {
	    vec.append(&mut vec![b'\0', b'\0']);
	    encode_node(vec, &x.left);
	    encode_node(vec, &x.right);
	},
	Element::None    => (),
    }
}
