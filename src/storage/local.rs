use std::io::Error;

use super::generic::StorageDriver;

#[derive(Debug)]
pub struct LocalStorage<'a> {
    data_path: &'a str,
}

impl<'a> Default for LocalStorage<'a> {
    fn default() -> Self {
        LocalStorage {
            data_path: "/var/njord/data",
        }
    }
}

impl<'a> StorageDriver for LocalStorage<'a> {
    fn flush_data(self) -> Result<(), Error> {
        Ok(())
    }
    fn init_storage(self) -> Result<(), Error> {
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::LocalStorage;

    #[test]
    fn test_default_node() {
        let local_storage = LocalStorage::default();
        assert_eq!(local_storage.data_path, "/var/njord/data")
    }

    #[test]
    fn test_flush_data() {
        let local_storage = LocalStorage::default();
    }

    #[test]
    fn test_init_storage() {
        let local_storage = LocalStorage::default();
        println!("{:?}", local_storage)
    }
}
