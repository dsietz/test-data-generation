// Copyright 2018 David Sietz and [`test-data-generator` contributors](https://github.com/dsietz/test-data-generator/blob/master/CONTRIBUTORS.md).
// Licensed under the MIT license
// (see LICENSE or <https://opensource.org/licenses/Apache-2.0>)

#![crate_type= "lib"]
#![crate_name = "test_data_generation"]

#[macro_use]
extern crate log;

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate serde_yaml;
extern crate yaml_rust;
extern crate regex;
extern crate rand;
extern crate crossbeam;
extern crate csv;

#[macro_use]
pub mod macros;
pub mod data_sample_parser;
pub mod configs;
pub mod profile;
pub mod data;