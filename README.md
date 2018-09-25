# Clock

Implement a clock that handles times without dates.

You should be able to add and subtract minutes to it.

Two clocks that represent the same time should be equal to each other.

## Rust Traits for .to_string()

Did you implement .to_string() for the Clock struct?

If so, try implementing the
[Display trait](https://doc.rust-lang.org/std/fmt/trait.Display.html) for Clock instead.

Traits allow for a common way to implement functionality for various types.

For additional learning, how would you implement String::from for the Clock type?

## Writing the Code

Execute the tests with:

```bash
$ cargo test
```

All but the first test have been ignored. After you get the first test to
pass, open the tests source file which is located in the `tests` directory
and remove the `#[ignore]` flag from the next test and get the tests to pass
again. Each separate test is a function with `#[test]` flag above it.
Continue, until you pass every test.

If you wish to run all tests without editing the tests source file, use:

```bash
$ cargo test -- --ignored
```

To run a specific test, for example `some_test`, you can use:

```bash
$ cargo test some_test
```

If the specific test is ignored use:

```bash
$ cargo test some_test -- --ignored
```

## Source

Pairing session with Erin Drummond [https://twitter.com/ebdrummond](https://twitter.com/ebdrummond)
