## Test Data Generation
[![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)

Linux: [![Build Status](https://travis-ci.org/dsietz/test-data-generation.svg?branch=master)](https://travis-ci.org/dsietz/test-data-generation)

### Description
**PROBLEM**
</br>
In order to make test data represent production, (a.k.a. realistic) you need to perform one of the following:
+ load data from a production environment into the non-production environment, which requires ETL (e.g.: masking, obfuscation, etc.)
+ stand up a pre-loaded "profile" database that is randomly sampled, which requires preparing sample data from either another test data source 
or production environment (option #1 above)

**SOLUTION**
</br>
 By analyzing a sample data set (e.g.: 2016 Census of top 200 male first names), we are able to create an algorithm (profile) based on that data sample. 
 This algorithm can be easily stored (as a data file) and used to generate "realistic" test data as desired. 

---

### Creating an Algorithm from Sample Data
The following utilities are part of the Test Data Generation project:
1. Data Algorithm Creator
2. Data Generator

#### Configuration
|  file name  | default location | description |
| ----------- | :--------------- | :---------- |
| log4rs.yaml | ./config         | setting for logging |
| tdg.yaml    | ./config         | general setting for Test Data generation |

---

### Support
Visit our [wiki](https://github.com/dsietz/test-data-generation/wiki) to help you get started with the project and its utilities

---
## Continuous Integration
See [travis](./.travis.yml) for detailed script

## Contributing

See [CONTRIBUTING](./CONTRIBUTING.md) for more information on technical details.

## License

test-data-generation is primarily distributed under the terms of the Apache License (Version 2.0).

See ![LICENSE-APACHE](.:LICENSE-APACHE "Apache License") for details.
