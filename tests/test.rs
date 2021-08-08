#[cfg(test)]
mod tests {
    /// Adds one to the number given.
    ///
    /// # Examples
    ///
    /// ```
    /// let x = 5;
    /// let y = 5;
    ///
    /// assert_eq!(x + y, 10);
    /// ```
    #[test]
    fn it_works() {
        assert_eq!(5 + 5, 10);
    }

    #[test]
    fn test_new() {
        println!("{:?}",falsework::cli::new())
    }
}
