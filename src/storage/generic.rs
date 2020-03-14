use std::io::Error;

pub trait StorageDriver {
    fn flush_data(&self) -> Result<(), Error>;
    fn init_storage(&self) -> Result<(), Error>;
}
