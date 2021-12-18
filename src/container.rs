
#[repr(C)]
pub struct HeaderEntry{
    filename:[u8;128],
    /// number of bytes in the file
    file_size:u64,
    /// start of the file, relative to the start of the container
    offset:u64,
}

pub struct ContainerHeader{

}

pub struct Container{
 
    
}
