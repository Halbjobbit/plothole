pub struct FunctionValueProvider<T>
where
    T: Fn(i64) -> i64,
{
    function: T,
}

impl<T> FunctionValueProvider<T>
where
    T: Fn(i64) -> i64,
{
    pub fn new(function: T) -> FunctionValueProvider<T> {
        Self { function }
    }

    pub fn get(&self, x: i64) -> i64 {
        (self.function)(x)
    }
}

