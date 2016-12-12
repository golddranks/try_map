/// Extend `Option` with a fallible map method
///
/// # Type parameters
///
/// - `T`: The input `Option`'s value type
/// - `U`: The outputs `Option`'s value type
/// - `E`: The possible error during the mapping
pub trait FallibleMapExt<T, U, E> {
    /// Try to apply a fallible map function to the option
    fn try_map<F>(&self, f: F) -> Result<Option<U>, E> where
        F: FnOnce(&T) -> Result<U, E>;
}

impl<T, U, E> FallibleMapExt<T, U, E> for Option<T> {
    fn try_map<F>(&self, f: F) -> Result<Option<U>, E> where
        F: FnOnce(&T) -> Result<U, E>
    {
        match self {
            &Some(ref x) => f(x).map(Some),
            &None => Ok(None),
        }
    }
}

fn try_main() -> Result<Option<i32>, &'static str> {
    let x = Some(42)
        .try_map(|x| Ok(x + 1))?
        .try_map(|x| Ok(x + 1))?
        .try_map(|_| Err("oh noes"))?
        .try_map(|x| Ok(x + 1))?;

    Ok(x)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        println!("{:?}", try_main());
    }
}
