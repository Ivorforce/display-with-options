use std::fmt::{Display, Formatter};

pub trait DisplayWithOptions<Options> {
    fn fmt(&self, f: &mut Formatter, options: &Options) -> std::fmt::Result;
}
