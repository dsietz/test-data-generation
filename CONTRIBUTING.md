### Contributing
We welcome your contributions!

There are many ways you can contribute to this project. Since this project is in its adolescenct stage, the simplest way to contribute is to start using `Test Data Generation`. (Please adhere to our adoption rules.)

### Adopter
You are welcome to download the product(s) and its source code for your use. By doing so, you are agreeing to the following responsibilities:

- Adhere to the [![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
- Report any bugs, enhancement requests, or product suggestions using our [Issues Tracking](https://github.com/dsietz/test-data-generation/issues)
- Submit any questions or comments for discussion about the this crate in the [Rust User Forum](https://users.rust-lang.org/)

### Developer
If you would like to get more involved and contribute to the code, (e.g.: fixing an issue or providing an enhancement), you are welcome to follow these steps:

>NOTE: In an effort to ensure compatibility, this project is restricted to the `STABLE RELEASE`

1. File a [request](https://github.com/dsietz/test-data-generation/issues), (or select an [existing one](https://github.com/dsietz/test-data-generation/issues)), and tag me with @dsietz on an issue. I'll then get you setup as a contributor. 
2. Fork our [repository](https://github.com/dsietz/test-data-generation)
3. Make the necessary code changes in your repo
4. Ensure that our [testing strategy and standards](./TESTING.md) are adhered as part of your development
5. Ensure that you have updated the [What's New](https://github.com/dsietz/test-data-generation/blob/development/README.md#whats-new) section of the README file to include your changes
6. Submit a properly formatted [pull request](./PULL_REQUESTS.md) to merge your changes to our [development](https://github.com/dsietz/test-data-generation/tree/development) branch

> As a note, all contributions are expected to follow [the Rust Code of Conduct](https://www.rust-lang.org/en-US/conduct.html).

#### Project
This [project](https://github.com/dsietz/test-data-generation/projects/1) attempts to be an idiomatic rust library and to maintain a sane structure. All source code is located in `src/`, and tests are in `tests/`.

The source is split into four modules:
- `lib.rs` contains top-level traits, module documentation, and helper functions
- `builders.rs` contains all the configuration code
- `errors.rs` contains error handling for finishing configuration