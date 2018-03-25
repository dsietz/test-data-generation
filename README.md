## Test Data Generation
[![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)

Linux: [![Build Status](https://travis-ci.org/dsietz/test-data-generation.svg?branch=master)](https://travis-ci.org/dsietz/test-data-generation)
Windows: [![Build status](https://ci.appveyor.com/api/projects/status/ejg8c33dn31nhv36/branch/master?svg=true)](https://ci.appveyor.com/project/kbknapp/clap-rs/branch/master)

### Description
For software development teams who need realistic test data for testing their software, this Test Data Generation library is a light-weight module 
that implements Markov Decision Process machine learning to quickly and easily profile sample data, create an algorithm, and produce representative test data without the need for 
persistent data sources, data cleaning, or remote services. Unlike other solutions, this open source solution can be integrated into your test source code, 
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

