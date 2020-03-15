//  Copyright 2020 Palade Ionut-Victor
//
//  Licensed under the Apache License, Version 2.0 (the "License");
//  you may not use this file except in compliance with the License.
//  You may obtain a copy of the License at
//  http://www.apache.org/licenses/LICENSE-2.0

use raft::prelude::*;
use raft::storage::{RaftState, Storage};

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

impl<'a> Storage for LocalStorage<'a> {
    fn initial_state(&self) -> raft::Result<RaftState> {
        self.initial_state()
    }

    fn entries(
        &self,
        low: u64,
        high: u64,
        max_size: impl Into<Option<u64>>,
    ) -> raft::Result<Vec<Entry>> {
        self.entries(low, high, max_size.into().unwrap_or(u64::MAX))
    }

    fn term(&self, idx: u64) -> Result<u64, raft::Error> {
        Ok(1)
    }

    fn first_index(&self) -> Result<u64, raft::Error> {
        Ok(1)
    }

    fn last_index(&self) -> Result<u64, raft::Error> {
        Ok(1)
    }

    fn snapshot(&self, request_index: u64) -> Result<Snapshot, raft::Error> {
        Ok(Snapshot::default())
    }
}
