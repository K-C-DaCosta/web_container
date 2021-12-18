use std::{
    io::{Read, Seek, Write},
    mem,
};
pub mod header; 
use header::*;


pub struct Container<R> {
    header: Option<ContainerHeader>,
    data: R,
}
impl<R> Container<R>
where
    R: Read,
{
    pub fn load(mut res: R) -> Self {
        unimplemented!("asdasd")
    }
}
