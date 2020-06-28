use super::{BYTE_SIZE_U8, SHIFT};

#[cfg(debug_assertions)]
use std::{fmt, collections::HashMap};

#[derive(Clone)]
pub struct Table {
    pub length: u8,
    pub binary: Vec<u8>,
}

#[cfg(debug_assertions)]
pub struct DebugHashMap(HashMap<u8, Table>);
#[cfg(debug_assertions)]
pub struct DebugVec(Vec<u8>);

impl Table {
    pub fn new() -> Self {
	Self { length: 0,
	       binary: Vec::new(), }
    }

    pub fn deeper(&self, bit: u8) -> Self {
	let mut vec = self.binary.clone();
	
	if self.length % BYTE_SIZE_U8 == 0 {
	    vec.push(bit);
	} else {
	    let last = vec.last_mut().unwrap();
	    *last = (*last << SHIFT) | bit;
	}
	
	Self { length: self.length + 1,
	       binary: vec }
    }
}

#[cfg(debug_assertions)]
impl fmt::Debug for Table {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
	fmt.debug_struct("Table")
	    .field("length", &self.length)
	    .field("binary", &DebugVec { 0: self.binary.clone() })
	    .finish()
    }
}

#[cfg(debug_assertions)]
impl fmt::Debug for DebugVec {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
	fmt.debug_list()
	    .entries(self.0.iter().map(|&val| format!("{:08b}", val)))
	    .finish()
    }
}

#[cfg(debug_assertions)]
impl fmt::Debug for DebugHashMap {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
	fmt.debug_map()
	    .entries(self.0.iter().map(|(key, val)|
				       (*key as char, val)))
	    .finish()
    }
}
