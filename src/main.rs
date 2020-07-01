mod queue;
mod tree;
mod convert;
mod types;
mod ui;
mod encoder;
mod decoder;

use types::ui::Type;

fn main() {
    let buffer = &mut vec![0; types::BUFFER_SIZE];
    let args   = ui::get_cli_args();
    
    match args.code_type {
	Type::Archive => encoder::encode(buffer, &args),
	Type::Extract => decoder::decode(buffer, &args),
    }
}
