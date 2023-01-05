pub(crate) trait ResultLike<T, E> {
    fn map<U, F: FnOnce(T) -> U>(self, f: F) -> Result<U, E>;
    fn map_err<F: FnOnce(E) -> E>(self, f: F) -> Result<T, E>;
}

impl<T, E> ResultLike<T, E> for Result<T, E> {
    fn map<U, F: FnOnce(T) -> U>(self, f: F) -> Result<U, E> {
        self.map(f)
    }
    fn map_err<F: FnOnce(E) -> E>(self, f: F) -> Result<T, E> {
        self.map_err(f)
    }
}
