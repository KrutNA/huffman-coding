use crate::types::{ table::Table, data::Data, };

use std::collections::HashMap;

pub fn encode_by_map(
    new_data: &mut Data,
    data: &[u8],
    table: &HashMap<u8, Table>
)
{
    for byte in data.iter() {
	let table = table.get(byte).expect(&format!("{}", *byte));
	(*new_data) += table;
    }
}
