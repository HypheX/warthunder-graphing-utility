struct Function<F>
where 
    F: Fn(f32) -> f32
{
    closure: F
}

impl<F> Function<F>
where
    F: Fn(f32) -> f32
{
    fn iter_range(&self, from: f32, to: f32, iterations: i32) -> std::vec::IntoIter<(f32, f32)> {
        let step_size = (to - from) / iterations as f32;

        let mut my_vec = Vec::new();

        let mut x = from;
        while x <= to {
            my_vec.push((x, (self.closure)(x)));
            x += step_size;
        }

        my_vec.into_iter()
    }
}