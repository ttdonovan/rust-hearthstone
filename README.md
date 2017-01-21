## rust-hearthstone

**CAUTION:** This project in still under active development and the API is
most likely to change. Use at your own risk.

Inspiration for this project comes from [python-hearthstone](https://github.com/HearthSim/python-hearthstone).

### Goals

A Hearthstone Rust library (CardDefs, DBF, enums, log parser) containing:

* A CardDefs.xml parser (`hearthstone::cardxml`)
* A Power.log parser (`hearthstone::hslog`)
* A DbfXml parser (`hearthstone::dbf`)
* Hearthstone enums as primitive integers (`hearthstone::enums`)

Further exploration for using the Rust library with other languages such as
Python, Ruby and/or C# by creating language specific wrappers.

### Requirements

* Rust Nightly

### Building

```
$ rustup default nightly
$ ./bootstrap
$ cargo build
```

### Testing

```
$ cargo test
```

### Examples

```
$ cargo run --example cardxml
```

### License

This project is licensed under the MIT license. The full license text is
available in the LICENSE file.

The CardDefs.xml file distributed contains Hearthstone data that
is copyright © Blizzard Entertainment.
