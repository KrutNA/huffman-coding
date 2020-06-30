use crate::types::ui::*;

use clap::{Arg, App};
use std::env;

pub fn get_cli_args<'a>() -> CliArgs {
    App::new("Huffman code")
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(Arg::with_name("type")
             .index(1)
	     .required(true)
	     .possible_values(&[ARCHIVE, EXTRACT]))
        .arg(Arg::with_name("input")
	     .short("i")
	     .required(true)
	     .value_name("FILE")
	     .validator(|file_name| {
		 if std::path::Path::new(&file_name).is_file() {
		     Ok(())
		 } else {
		     Err(format!("Can't open file {}", file_name))
		 }
	     }))
        .arg(Arg::with_name("output")
             .short("o")
             .takes_value(true)
             .value_name("FILE"))
        .get_matches()
	.into()
}
