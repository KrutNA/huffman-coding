mod queue;
mod tree;
mod convert;
mod types;
mod ui;
mod encode;

use types::ui::Type;

fn main() {
    // let buffer = &mut [0; types::BUFFER_SIZE];
    let buffer = &mut vec![0; types::BUFFER_SIZE];
    let args   = ui::get_cli_args();
    
    match args.code_type {
	Type::Archive => encode::execute(buffer, &args),
	Type::Extract => (),
    }
}
