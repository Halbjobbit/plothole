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

pub struct FunctionCanvasPivot {
    pub x: i32,
    pub y: i32,
}

impl FunctionCanvasPivot {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn x(&mut self, x: i32) {
        self.x = x;
    }

    pub fn y(&mut self, y: i32) {
        self.y = y;
    }
}
