#[macro_use]
extern crate enum_display_derive;

use failure::Fail;
use std::fmt::{self, Display};

#[derive(Display, Debug)]
pub enum ErrorSource {
    Inputs,
    Outputs,
}

pub struct ErrorLocation {
    source: ErrorSource,
    index: usize,
}

impl Display for ErrorLocation {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}[{}]", self.source, self.index)
    }
}

impl ErrorLocation {
    fn inputs(index: usize) -> Self {
        Self {
            source: ErrorSource::Inputs,
            index,
        }
    }
}

#[derive(Fail, Debug)]
pub enum Error {
    #[fail(display = "Foo")]
    Foo,
    #[fail(display = "Immature({}, {})", _0, _1)]
    Immature(usize, usize),
}

fn main() {
    println!("{}", Error::Foo);
    println!("{}", Error::Immature(5, 6));
}
