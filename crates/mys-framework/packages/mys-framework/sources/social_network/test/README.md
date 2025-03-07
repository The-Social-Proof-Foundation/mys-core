# Social Network Tests

This directory contains test modules for the Mysten Social Network features.

## Test Modules

- `profile_tests.move`: Tests for profile creation and management
- `post_tests.move`: Tests for posts, comments, likes, and other content interactions
- `social_graph_tests.move`: Tests for follow/unfollow functionality and social relationships
- `test_runner.move`: Helper module for running all tests together

## Running Tests

You can run the tests using the MySocial Move testing framework. From the root of the repository, use the following commands:

### Run All Tests

To run all tests in the social network module:

```bash
mys move test --path ./crates/mys-framework/packages/mys-framework
```

### Run Specific Test Modules

To run tests for a specific module:

```bash
# Run profile tests
sui move test --path ./crates/mys-framework/packages/mys-framework/sources/social_network --filter profile_tests

# Run post tests
sui move test --path ./crates/mys-framework/packages/mys-framework --filter post_tests

# Run social graph tests
sui move test --path ./crates/mys-framework/packages/mys-framework --filter social_graph_tests
```

### Run Individual Tests

To run a single test function:

```bash
# Example: Run just the profile creation test
sui move test --path ./crates/mys-framework/packages/mys-framework --filter test_create_profile
```

## Test Coverage

The test suite covers the following key functionality:

1. **Profile Management**
   - Profile creation
   - Profile updating
   - Ownership verification

2. **Content Interactions**
   - Post creation
   - Comment creation
   - Like/unlike functionality

3. **Social Graph**
   - Follow/unfollow functionality
   - Relationship tracking
   - Validation rules (e.g., preventing self-follows)

## Adding New Tests

When adding new features to the social network modules, please add corresponding tests that:

1. Test the happy path (expected successful behavior)
2. Test edge cases and failure scenarios
3. Use the test_scenario framework to simulate multi-transaction sequences

Follow the existing patterns in the test modules for consistency.
