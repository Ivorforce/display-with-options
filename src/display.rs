use std::fmt::{Display, Formatter};
use crate::with_options;

pub trait DisplayWithOptions<Options> {
    fn fmt(&self, f: &mut Formatter, options: &Options) -> std::fmt::Result;
}

pub fn format_with_options<Options>(object: &impl DisplayWithOptions<Options>, options: &Options) -> String {
    format!("{}", with_options(object, options))
}
