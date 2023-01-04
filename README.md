# should-be

Postfix assertions of equality
```rust
use should_be::ShouldBe; // Import the trait
let yaks_shaved = 0;
yaks_shaved.should_be(0); // passes
```
```rust
yaks_shaved.should_be(1); // panics
// you can provide a panic message using a tuple.
yaks_shaved.should_be((1, "why haven't you shaved?"))
```
