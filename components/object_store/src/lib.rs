// Copyright 2023 The CeresDB Authors
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Re-export of [object_store] crate.

use std::sync::Arc;

pub use upstream::{
    local::LocalFileSystem, path::Path, Error as ObjectStoreError, GetResult, ListResult,
    ObjectMeta, ObjectStore,
};

pub mod aliyun;
pub mod config;
pub mod disk_cache;
pub mod mem_cache;
pub mod metrics;
pub mod multipart;
pub mod obkv;
pub mod prefix;
pub mod s3;

pub type ObjectStoreRef = Arc<dyn ObjectStore>;
