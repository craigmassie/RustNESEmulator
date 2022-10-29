pub trait Read<T, U> {

    fn read(&self, address: T) -> Option<U>;

    fn read_only(&self, address: T) -> Option<U>;
}