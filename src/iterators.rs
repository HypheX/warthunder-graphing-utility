use std::marker::PhantomData;

use plotters::{backend::DrawingBackend, element::{Circle, ComposedElement, DynElement, EmptyElement, IntoDynElement, PathElement, Text}, style::{ShapeStyle, TextStyle}};

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
    F: Fn(f64) -> f64,
{
    pub fn new(iterations: i32, from: f64, to: f64, closure: F) -> Self {
        Self { 
            closure, 
            iterations, 
            from, 
            step_size: (to - from) / iterations as f64, 
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




struct LabeledFunction<'a, F, DB>
where
    F: Fn(f64) -> f64,
    DB: DrawingBackend,
{
    function: FunctionIterator<F>,
    label_pos: (f64, f64),
    lable_is_drawn: bool,
    prev_point: Option<(f64, f64)>,
    line_style: ShapeStyle,
    label_style: TextStyle<'a>,
    label: String,
    phantom: PhantomData<DB>,
}
impl<'a, F, DB> LabeledFunction<'a, F, DB> 
    where 
    F: Fn(f64) -> f64,
    DB: DrawingBackend,
{
    pub fn new(iterations: i32, from: f64, to: f64, closure: F, line_style: ShapeStyle, label_pos: f64, label_offset: (f64, f64), label: String, label_style: TextStyle<'a>) -> Self {
        Self { 
            label_pos: (label_pos + label_offset.0, closure(label_pos) + label_offset.1),
            function: FunctionIterator::new(iterations, from, to, closure),
            lable_is_drawn: false,
            prev_point: None,
            line_style,
            label,
            label_style,
            phantom: PhantomData,
        }
    }
}



impl<'a, F, DB> Iterator for LabeledFunction<'a, F, DB> 
where
    F: Fn(f64) -> f64,
    DB: DrawingBackend,
{
    type Item = DynElement<'a, DB, (f64, f64)>;

    fn next(&mut self) -> Option<Self::Item> {
        let current_point = self.function.next();

        if let (Some(coord1), Some(coord2)) = (self.prev_point, current_point) {
            return Some(
                PathElement::new(vec![coord1, coord2], self.line_style).into_dyn()
            )
        }
        
        if !self.lable_is_drawn {

            return Some( 
                Text::new(
                    self.label.clone(), 
                    self.label_pos.clone(), 
                    self.label_style.clone()
                ).into_dyn()
            )
        }

        None
    }
}
