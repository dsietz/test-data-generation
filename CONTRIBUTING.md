### Contributing

#### Getting started (section in README)

We welcome your contributions!

There are many ways you can contribute to this project. Since this project is in its infancy stage, the simplest way to contribute is to start using `Test Data Generation`. 
Provide us with constructive feedback, (e.g.: bugs, enhancement requests, product roadmap direction).

If you'd like to get more involved and contribute to the code, (e.g.: fixing an issue or providing an enhancement), just file a [request](https://github.com/dsietz/test-data-generation/issues), or you want to work on an [existing one](https://github.com/dsietz/test-data-generation/issues), tag me with @dsietz on an issue, and get you setup as a contributor.

As a note, all contributions are expected to follow [the Rust Code of Conduct](https://www.rust-lang.org/en-US/conduct.html).

#### Project Structure

This project attempts to be an idiomatic rust library and to maintain a sane structure. All source code is located in `src/`, and tests are in `tests/`.

The source is split into four modules:
- `lib.rs` contains top-level traits, module documentation, and helper functions
- `builders.rs` contains all the configuration code
- `errors.rs` contains error handling for finishing configuration

#### Pull requests

Pull requests are _the_ way to change code using git. If you aren't familiar with them in general, GitHub has some [excellent documentation](https://help.github.com/articles/about-pull-requests/).

There aren't many hard guidelines in this repository on how specifically to format your request. Main points:

- Please include a descriptive title for your pull request, and elaborate on what's changed in the description.
- Feel free to open a PR before the feature is completely ready, and commit directly to the PR branch.
  - This is also great for review of PRs before merging
  - All commits will be squashed together on merge, so don't worry about force pushing yourself.
- Please include at least a short description in each commit, and more of one in the "main" feature commit. Doesn't
  have to be much, but someone reading the history should easily tell what's different now from before.
- If you have `rustfmt-nightly` installed, using it is recommended. I can also format the code after merging the code,
  but formatting it consistently will make reviewing nicer.

### Testing

Building fern is as easy as is expected, `cargo build`.

Testing is somewhat more convoluted - mostly because using fern requires initializing a global logger. To test the two "integration" tests separately, you'll need to invoke two commands:

```sh
cargo test -- --skip test2
cargo test test2
```

Feel free to add tests and examples demonstrating new features as you see fit. Pull requests which solely add new/interesting example programs are also welcome.