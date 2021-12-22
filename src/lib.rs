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
impl ErrorKind{
    fn text_error<T>(e:T)->Self{
        Self::GenericTextError("asdas")
    }
}

pub type ContainerError<T> = Result<T, ErrorKind>;


