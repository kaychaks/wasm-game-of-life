## WASM Game of Life

This is standard Game of Life implementation as showcased in the [Rust WASM Book Tutorial](https://rustwasm.github.io/docs/book/game-of-life/introduction.html). However, there are minor changes in implementation

- I hate continous `for..in` loops. So I have used the `cartesian_product()` from `itertools` crate for when walking down rows & columns. And then good old `fold` for further creation. 
- Also I used the `From` implementation for `Cell` type for the dead and alive checks (rather than using native bool or numeric types)

The JS code is pretty much the same as shown in the tutorial. _That could have been better_. However, my focus was (is) to understand WASM more than JS. 

### Setup

[Follow the setup instructions here](https://rustwasm.github.io/docs/book/game-of-life/setup.html)

## Build

```sh
wasm-pack build
```
