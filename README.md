# try_map
**UPDATE 20th of Feb 2018**
The Rust standard library is getting this functionality with name `transpose`, so this crate is going to fade away. The tracking issue is here: https://github.com/rust-lang/rust/issues/47338

`try_map` method for `Option` and `flip` method for `Option` and `Vec`. These helper methods allow more ergonomic error handling when mapping
functions that return `Result`, over collections.

## How to use:

Add to `Cargo.toml`:

    [dependencies]
    try_map = "0.3"

Bring the extension traits to the scope in your code: (`FallibleMapExt` is for enabling `try_map` and `FlipResultExt` is for enabling `flip`.)

    use try_map::{FallibleMapExt, FlipResultExt};

Use the `try_map` and `flip` methods like a boss!

        fn try_map_example() -> Result<Option<i32>, &'static str> {
            let x = Some(42)
                .try_map(|x| Ok(x + 1))?
                .try_map(|x| Ok(x + 1))?
                .try_map(|x| if true { Err("oh noes") } else { Ok(x + 1) })?
                .try_map(|x| Ok(x + 1))?;
        
            Ok(x)
        }
        assert_eq!(try_map_example(), Err("oh noes"));

        fn flip_example() -> Result<Option<i32>, &'static str> {
            let x = Some(42)
                .map(|x| Ok(x + 1)).flip()?
                .map(|x| Ok(x + 1)).flip()?
                .map(|x| if true { Err("oh noes") } else { Ok(x + 1) }).flip()?
                .map(|x| Ok(x + 1)).flip()?;
        
            Ok(x)
        }
        assert_eq!(flip_example(), Err("oh noes"));

## What else?

There is an open issue in the Rust RFC repo suggesting bringing these helper methods to the standard library: https://github.com/rust-lang/rfcs/issues/1815
Thanks for @killercup for suggesting implementing these in a 3rd party crate. This way they are immediately useful. (My claim is that they would be still
useful in the standard library, though!)

It seems that to be able to abstract the trait providing `try_map` over different kinds of collections, we need a support for higher-kinded types or associated
type constructors. However, the `flip` method seems to be implementable for all kinds of things just using associated types.
