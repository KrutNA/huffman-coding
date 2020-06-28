use crate::types::{ table::Table,
		    data::Data, };

use std::collections::HashMap;

pub fn encode_by_map(
    data: &Vec<u8>,
    table: &HashMap<u8, Table>
)
    -> Data
{
    let mut new_data = Data::default();
    for byte in data.iter() {
	let table = table.get(byte).unwrap();
	new_data += table;
    }
    new_data
    
}
