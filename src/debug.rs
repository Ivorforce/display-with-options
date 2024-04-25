use std::fmt::{Debug, Formatter};

pub trait DebugWithOptions<Options> {
    fn fmt(&self, f: &mut Formatter, options: &Options) -> std::fmt::Result;
}
