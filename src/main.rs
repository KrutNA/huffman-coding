mod queue;
mod tree;
mod convert;
mod types;


// const DATA = include_str!("files/lorem_ipsum");
// const DATA = "mississippi river";
// const DATA = include_str!("files/hard_data");
const DATA: &'static [u8] = include_bytes!("files/random_data");

fn main() {

    // #[cfg(debug_assertions)]
    // println!("{}", data);
    
    let     data   = Vec::from(DATA);
    let mut heap   = queue::generate_from_data(&data);
    let     root   = tree::generate_from_heap(&mut heap);
    let     table  = tree::convert_to_table(&root);
    let     result = convert::encode_by_map(&data, &table);
    
    #[cfg(feature = "ptree")]
    ptree::print_tree(&root).unwrap();
    
    // #[cfg(debug_assertions)]
    // println!("{:#?}", types::table::DebugHashMap::new(&table));

    println!("{}", result.data.len());
}
