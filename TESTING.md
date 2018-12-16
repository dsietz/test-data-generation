### Testing Strategy and Standards

Our project follows the [Test Driven Development](https://en.wikipedia.org/wiki/Test-driven_development) approach. This means that all tests are written __prior__ to the development of the working code. Our goal is to have a 90% or high code coverage whenever released to the `Master` branch.

#### Standards
- All tests are located in the `tests` directory in their aligned test file (e.g.: .tests/facts.rs are thge tests for the profile::fact::Fact)
- All tests should have names that describe what they are testing (e.g.: new_fact_from_serialized)
- Tests should include both the positive and negative scenarios
- Test should cover exceptions and how they are handled
- There should be tests that represent how the users will use the crate's functionalitiy  