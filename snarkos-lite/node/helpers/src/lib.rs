// Copyright 2024 Aleo Network Foundation
// This file is part of the snarkOS library.

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at:

// http://www.apache.org/licenses/LICENSE-2.0

// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

// AlexZ: This is just a temp crate with content that might be based 
// on multiple snarkos crates. The aim is to see which parts of the BFT 
// code we will need to keep...

#![forbid(unsafe_code)]
#![recursion_limit = "256"]

pub mod helpers;
pub use helpers::*;