pub trait SimpleCache {
    ///
    /// Return the size of the cache
    ///
    fn size(&self) -> usize;

    ///
    /// Insert a value into the cache
    ///
    /// All inserts will be written to memory,
    /// and then cached later on gets. No caching
    /// will be done on puts
    ///
    fn put(&mut self, key: &str, value: &str);

    ///
    /// Retrieve a value from the table
    ///
    /// This method should call the `evict()` method
    /// if the given cache has exceeded the maximum
    /// acceptable size.
    ///
    fn get(&mut self, key: &str) -> Option<String>;

    ///
    /// Evict the next decided value from the cache
    ///
    /// No parameters or return, it is up to each
    /// different caching method how this should be implemented.
    ///
    fn evict(&mut self);

    ///
    /// Check if the given cache contains a key
    ///
    fn contains(&self, key: &str) -> bool;
}
