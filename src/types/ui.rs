use clap::ArgMatches;

pub const ARCHIVE: &'static str = "a";
pub const EXTRACT: &'static str = "x";

pub enum Type {
    Archive,
    Extract,
}

pub struct CliArgs {
    pub input:     String,
    pub output:    String,
    pub code_type: Type,
}

impl<'a> From<ArgMatches<'a>> for CliArgs {
    fn from(matches: ArgMatches<'a>) -> Self {	
	Self {
            input:     matches.value_of("input").unwrap().to_string(),
	    output:    matches.value_of("output").unwrap().to_string(),
            code_type: match matches.value_of("type").unwrap() {
		ARCHIVE => Type::Archive,
		_       => Type::Extract,
	    },
	}
    }
}
