use std::error::Error;



mod container;
pub use container::*;

mod packing;
pub use packing::*;

#[derive(Debug)]
pub enum ErrorKind {
    PackFailed(&'static str),
    GenericTextError(&'static str),
    GenericError(Box<dyn Error>),
}
pub type ContainerError<T> = Result<T, ErrorKind>;


