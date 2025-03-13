# Social Package for Mys

This package contains social networking features for the Mys blockchain.

## Modules

### Profile

The Profile module provides user profile management functionality:

- Create and update user profiles
- Associate profiles with usernames
- Query profile information
- Events for profile changes

### Counter

The Counter module is a simple test module that provides a way to:

- Create counters with initial values
- Increment counters
- Reset counters
- Query counter values

## Usage

### Publishing the Package

```bash
myso client publish --gas-budget 100000000 ./crates/social/Move
```

### Counter Module Usage

After publishing, you'll get a package ID like `0xabcdef...`. Use that ID for these commands:

```bash
# Create a new counter (default value 0)
myso client call --package <PUBLISHED_PACKAGE_ID> --module counter --function create_counter --gas-budget 10000000

# Create a counter with a specific initial value
myso client call --package <PUBLISHED_PACKAGE_ID> --module counter --function create_counter_with_value --args 42 --gas-budget 10000000

# Increment a counter by 1
myso client call --package <PUBLISHED_PACKAGE_ID> --module counter --function increment --args <COUNTER_OBJECT_ID> --gas-budget 10000000

# Increment a counter by a specific amount
myso client call --package <PUBLISHED_PACKAGE_ID> --module counter --function increment_by --args <COUNTER_OBJECT_ID> 10 --gas-budget 10000000

# Reset a counter to 0
myso client call --package <PUBLISHED_PACKAGE_ID> --module counter --function reset --args <COUNTER_OBJECT_ID> --gas-budget 10000000
```

### Profile Module Usage

```bash
# Create a profile
myso client call --package <PUBLISHED_PACKAGE_ID> --module profile --function create_and_register_profile --args "My Display Name" "My Bio Text" "https://example.com/profile.jpg" --gas-budget 10000000

# Create a test profile
myso client call --package <PUBLISHED_PACKAGE_ID> --module profile --function create_test_profile --gas-budget 10000000

# Update a profile
myso client call --package <PUBLISHED_PACKAGE_ID> --module profile --function update_profile --args <PROFILE_OBJECT_ID> "New Display Name" "New Bio Text" "https://example.com/new-profile.jpg" --gas-budget 10000000
```