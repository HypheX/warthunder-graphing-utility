pub struct FunctionIterator<F>
where 
    F: Fn(f64) -> f64
{
    closure: F,
    iterations: i32, 
    from: f64,
    step_size: f64, 
    i: i32,
}
impl<F> FunctionIterator<F>
where 
    F: Fn(f64) -> f64
{
    pub fn new(iterations: i32, from: f64, step_size: f64, closure: F) -> Self {
        Self { 
            closure, 
            iterations, 
            from, 
            step_size, 
            i: 0,
        }
    }
}

impl<F> Iterator for FunctionIterator<F>
where
    F: Fn(f64) -> f64
{
    type Item = (f64, f64);
    
    fn next(&mut self) -> Option<Self::Item> {
        if self.i > self.iterations {
            return None
        }
        
        let x = self.from + (self.step_size * self.i as f64);

        let y = (self.closure)(x);

        self.i += 1;
        Some((x, y))
    }
}