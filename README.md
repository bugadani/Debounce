Debounce
=========

Debounce is a Rust library used to process noisy binary state transitions, like button presses in embedded systems.

Creating a debouncer
--------------------

_**Note**: Debounce works by counting consecutive samples, so you'll need to figure out how many samples you need to consider the input **stable**._

As an example we want to process button presses. We determine, that 2 consecutive samples are enough to consider our input stable.

```rust
use debounce::{Debounce, State, Change};
use debounce::consts::U2; // 2 samples for stable signal

let debounce: Debounce<U2> = Debounce::new(State::Released);
```

Feeding data
------------

You can feed data into debouncer using the `update` method. Update takes the current state of your input as a `State`, which can be either `State::Touched` or `State::Released`.

You may also use `.into()` on a `bool` to convert it into `State`. `true` will be converted to `State::Touched` and false to `State::Released`.

Observing state
---------------

`update` returns with a `Change` value that indicates what happened. A `Change` can have three values:

 * `Change::Touched` or `Change::Released` when the internal state changes.
 * `Change::NoChange(state)` when input does not change the internal state. `state` will contain the current stable signal state, which can be diferent from the input state.

_Note: if you only care about the signal state and not the change, you can use `.into()` to convert the `Change` value to a corresponding `State` value._

You can also use the `state` method that returns a `State` value that indicated the current stable signal state.
