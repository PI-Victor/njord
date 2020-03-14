//  Copyright 2020 Palade Ionut-Victor
//
//  Licensed under the Apache License, Version 2.0 (the "License");
//  you may not use this file except in compliance with the License.
//  You may obtain a copy of the License at
//  http://www.apache.org/licenses/LICENSE-2.0

use std::io::{Error, ErrorKind};

use super::generic::StorageDriver;
use super::local::LocalStorage;

pub fn new_storage_driver(driver: &str) -> Result<Box<dyn StorageDriver>, Error> {
    match driver {
        "local" => Ok(Box::new(LocalStorage::default())),
        driver => {
            let err_msg = format!("Storage driver: {:}, not found", driver);
            Err(Error::new(ErrorKind::Other, err_msg))
        }
    }
}

#[cfg(test)]
mod test {
    use super::new_storage_driver;

    #[test]
    fn test_storage_driver() {
        new_storage_driver("local").unwrap();
    }

    #[test]
    #[should_panic(expected = "Storage driver: unavailable, not found")]
    fn test_storage_driver_not_found() {
        new_storage_driver("unavailable").unwrap();
    }
}
