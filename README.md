# math_util

a small math utility library written in rust.

mostly made this to learn rust and have somewhere to put random math functions i end up making.

## what's in it

* basic arithmetic
* min / max / clamp
* lerp
* geometry stuff
* constants
* more stuff eventually

## example

```rust
use math_util::arithmetic::lerp;

fn main() {
    let result = lerp(0.0, 10.0, 0.5);

    println!("{}", result);
}
```

## why?

i wanted to learn rust, so i decided making a little math library would be a good way to practice.

it's not meant to compete with anything, it's just a project i'm working on and adding to whenever i feel like it.

## status

still being worked on.

things will probably change a lot.

## license

this project is licensed under the **Apache License 2.0**.

see `LICENSE` for the full license text.
