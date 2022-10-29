pub trait Write<T, U> {

    fn write(&self, address: T, data: U);
}