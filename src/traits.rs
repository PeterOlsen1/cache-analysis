pub trait SimpleCache {
    ///
    /// Insert a value into the cache
    ///
    /// This method should call the `evict()` method
    /// if the given cache has exceeded the maximum
    /// acceptable size.
    ///
    fn put(key: &str, value: &str);

    ///
    /// Retrieve a value from the table
    ///
    fn get(key: &str) -> String;

    ///
    /// Evict the next decided value from the cache
    ///
    /// No parameters or return, it is up to each
    /// different caching method how this should be implemented.
    ///
    fn evict();
}
