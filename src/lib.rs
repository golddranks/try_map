/// Extend `Option` with a fallible map method
///
/// This is useful for mapping fallible operations (i.e. operations that)
/// return `Result`, over an optional type. The result will be 
/// `Result<Option<U>>`, which makes it easy to handle the errors that
/// originate from inside the closure that's being mapped.
///
/// # Type parameters
///
/// - `T`: The input `Option`'s value type
/// - `U`: The outputs `Option`'s value type
/// - `E`: The possible error during the mapping
pub trait FallibleMapExt<T, U, E> {
    /// Try to apply a fallible map function to the option
    fn try_map<F>(self, f: F) -> Result<Option<U>, E> where
        F: FnOnce(T) -> Result<U, E>;
}

impl<T, U, E> FallibleMapExt<T, U, E> for Option<T> {
    fn try_map<F>(self, f: F) -> Result<Option<U>, E> where
        F: FnOnce(T) -> Result<U, E>
    {
        match self {
            Some(x) => f(x).map(Some),
            None => Ok(None),
        }
    }
}

/// Extend `Option<Result<T>>` with a `flip` method that scavenges the inner `Result`
/// type and brings it to the outernmost type for easy error handling.
///
/// This makes easy to `map`, `and_then` etc. with fallible (`Result`-returning)
/// functions over `Option` and then call `flip` to "surface" the `Result` for error handling.
///
/// # Type parameters
///
/// - `T`: The inner value type
/// - `E`: The error type of `Result`
pub trait FlipResultExt<T, E> {
    fn flip(self) -> Result<Option<T>, E>;
}

impl<T, E> FlipResultExt<T, E> for Option<Result<T, E>> {
    fn flip(self) -> Result<Option<T>, E>
    {
        match self {
            Some(r) => r.map(Some),
            None => Ok(None),
        }
    }
}

#[cfg(test)]
mod tests {
    use FallibleMapExt;
    use FlipResultExt;

    #[test]
    fn test_try_map_1() {
        fn inner() -> Result<Option<i32>, &'static str> {
            let x = Some(42)
                .try_map(|x| Ok(x + 1))?
                .try_map(|x| Ok(x + 1))?
                .try_map(|x| if true { Err("oh noes") } else { Ok(x + 1) })?
                .try_map(|x| Ok(x + 1))?;
        
            Ok(x)
        }
        assert_eq!(inner(), Err("oh noes"));
    }

    #[test]
    fn test_try_map_2() {
        fn inner() -> Result<Option<i32>, &'static str> {
            let x = Some(42)
                .try_map(|x| Ok(x + 1))?
                .try_map(|x| Ok(x + 1))?
                .try_map(|x| Ok(x + 1))?;
        
            Ok(x)
        }
        assert_eq!(inner(), Ok(Some(45)));
    }

    #[test]
    fn test_try_map_3() {
        fn inner() -> Result<Option<i32>, &'static str> {
            let x = Some(42)
                .try_map(|x| Ok(x + 1))?
                .try_map(|x| Ok(x + 1))?
                .try_map(|x| if true { Err("oh noes") } else { Ok(x + 1) })?
                .try_map(|x| Ok(x + 1))?
                .try_map(|x| if true { Err("oh foes") } else { Ok(x + 1) })?;
        
            Ok(x)
        }
        assert_eq!(inner(), Err("oh noes"));
    }

    #[test]
    fn test_flip_1() {
        fn inner() -> Result<Option<i32>, &'static str> {
            let x = Some(42)
                .map(|x| Ok(x + 1)).flip()?
                .map(|x| Ok(x + 1)).flip()?
                .map(|x| if true { Err("oh noes") } else { Ok(x + 1) }).flip()?
                .map(|x| Ok(x + 1)).flip()?;
        
            Ok(x)
        }
        assert_eq!(inner(), Err("oh noes"));
    }

    #[test]
    fn test_flip_2() {
        fn inner() -> Result<Option<i32>, &'static str> {
            let x = Some(42)
                .map(|x| Ok(x + 1)).flip()?
                .map(|x| Ok(x + 1)).flip()?
                .map(|x| Ok(x + 1)).flip()?;
        
            Ok(x)
        }
        assert_eq!(inner(), Ok(Some(45)));
    }

    #[test]
    fn test_flip_3() {
        fn inner() -> Result<Option<i32>, &'static str> {
            let x = Some(42)
                .map(|x| Ok(x + 1)).flip()?
                .map(|x| Ok(x + 1)).flip()?
                .map(|x| if true { Err("oh noes") } else { Ok(x + 1) }).flip()?
                .map(|x| Ok(x + 1)).flip()?
                .map(|x| if true { Err("oh foes") } else { Ok(x + 1) }).flip()?;
        
            Ok(x)
        }
        assert_eq!(inner(), Err("oh noes"));
    }
}
