## Test Data Generation
[![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
[![Coverage Status](https://coveralls.io/repos/github/dsietz/test-data-generation/badge.svg?branch=master)](https://coveralls.io/github/dsietz/test-data-generation?branch=master)

Linux: [![Build Status](https://travis-ci.org/dsietz/test-data-generation.svg?branch=master)](https://travis-ci.org/dsietz/test-data-generation)
Windows: [![Build status](https://ci.appveyor.com/api/projects/status/uw58v5t8ynwj8s8o/branch/master?svg=true)](https://ci.appveyor.com/project/dsietz/test-data-generation/branch/master)

### Description
For software development teams who need realistic test data for testing their software, this Test Data Generation library is a light-weight module 
that implements Markov decision process machine learning to quickly and easily profile sample data, create an algorithm, and produce representative test data without the need for 
persistent data sources, data cleaning, or remote services. Unlike other solutions, this open source solution can be integrated into your test source code, or 
wrapped into a web service or stand-alone utility.   

**PROBLEM**
</br>
In order to make test data represent production, (a.k.a. realistic) you need to perform one of the following:
+ load data from a production environment into the non-production environment, which requires ETL (e.g.: masking, obfuscation, etc.)
+ stand up a pre-loaded "profile" database that is randomly sampled, which requires preparing sample data from either another test data source 
or production environment (option #1 above)

**SOLUTION**
</br>
 Incorporate this library in your software's testing source code by loading an algorithm from a previously analyzed data sample and generating 
 test data during your tests runtime.
 
---

### Table of Contents
* [What's New](#whats-new)
* [About](#about)
* [Configuration](#configuration)
* [How to Contribute](#how-to-contribute)
* [License](#license)

## What's New

Here's whats new in 0.0.1:

* **Demo Mode:**  demonstrate the functionality of generating dates and peopl's names
* **Analyze CSV file**: parse a csv file (mutli fields) and create an algorithm
* **Save Data Sample Parser**: save the data sample parser (with its algorithm, but not sampel data) as a json file
* **load Data Sample Parser**: load a json file of a previoulsy saved the data sample parser
* **generate record**: generate a test data record
* **genrate by field name**: generate test data by the record's field name
* Updated the `README.md`

## About

`test data generation` uses [Markov decision process](https://en.wikipedia.org/wiki/Markov_decision_process) machine learning to create algorithms that enable test data generation on the fly without the overhead 
of test data databases, security data provisioning (e.g.: masking, obfuscation), or standing up remote services.

The algorithm is built on the bases of:
1. character patterns
2. frequency of patterns
3. character locations
4. beginning and ending characters
5. length of entity (string, date, number) 

**Example**
```
extern crate test_data_generation;
use test_data_generation::data_sample_parser::DataSampleParser;

fn main() {
    	let mut dsp = DataSampleParser::new();
    	dsp.analyze_csv_file("./tests/samples/sample-01.csv").unwrap();
    	
    	println!("My new name is {} {}", dsp.generate_record()[0], dsp.generate_record()[1]);
    	// My new name is Abbon Aady
}
```

## Configuration
|  file name  | default location | description |
| ----------- | :--------------- | :---------- |
| tdg.yaml    | ./config         | general setting for Test Data generation |

## How to Contribute

Details on how to contribute can be found in the [CONTRIBUTING](./CONTRIBUTING.md) file.

## License

test-data-generation is primarily distributed under the terms of the Apache License (Version 2.0).

See ![LICENSE-APACHE](.:LICENSE-APACHE "Apache License") for details.

### Documentation
Documentation for this crate is built without dependencies. 
```
cargo doc --no-deps
```

