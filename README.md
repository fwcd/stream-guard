# Stream Guard

A small RAII wrapper around a [`Stream`](https://docs.rs/futures/latest/futures/stream/trait.Stream.html) (asynchronous iterator) that automatically invokes a user-defined action upon being dropped.
