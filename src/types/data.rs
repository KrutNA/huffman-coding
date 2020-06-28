use super::table::Table;
use super::BYTE_SIZE_U8;

use std::ops::AddAssign;

#[cfg(debug_assertions)]
use std::fmt;

#[derive(Default)]
pub struct Data {
    pub last: u8,
    pub data: Vec<u8>,
}

type Byte = (u8, u8);
impl AddAssign<Byte> for Data {
    fn add_assign(&mut self, byte: Byte) {
	if self.last % BYTE_SIZE_U8 == 0 {
	    self.data.push(0);
	    self.last = 0;
	}
	let last = self.data.last_mut().unwrap();
	if byte.1 <= BYTE_SIZE_U8 - self.last {
	    *last |= byte.0 << (8 - byte.1 - self.last);
	    self.last = self.last + byte.1;
	}
	else {
	    let diff = BYTE_SIZE_U8 - self.last;
	    let (head, tail) = (byte.0 >> (byte.1 - diff),
				byte.0 & ((1 << (byte.1 - diff)) - 1));
	    *last |= head;
	    self.last = 8;
	    self.add_assign((tail, byte.1 - diff));
	}
    }
}

impl <'a> AddAssign<&'a Table> for Data {
    fn add_assign(&mut self, data: &'a Table) {
	let mut length = data.length;
	for &byte in data.binary.iter() {
	    if length > 8 {
		*self += (byte, 8);
		length -= 8;
	    } else {
		*self += (byte, length);
	    }
	}
    }
}


#[cfg(debug_assertions)]
impl fmt::Debug for Data {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
	fmt.debug_list()
	    .entries(self.data.iter().map(|&val| format!("{:08b}", val)))
	    .finish()
    }
}
