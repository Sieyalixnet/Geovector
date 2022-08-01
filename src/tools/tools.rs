extern crate web_sys;
use web_sys::console;

pub struct Timer<'a> {
    name: &'a str,
}

impl<'a> Timer<'a> {
    pub fn new(name: &'a str) -> Timer<'a> {
        console::time_with_label(name);
        Timer { name }
    }
}

impl<'a> Drop for Timer<'a> {
    fn drop(&mut self) {
        console::time_end_with_label(self.name);
    }
}

pub fn reshape(array1d:&Vec<f64>, shape_value: usize) -> Vec<Vec<f64>> {
    let mut result: Vec<Vec<f64>> = Vec::new();
    let pieces = array1d.len() / shape_value;
    for i in 0..pieces {
        result.push((array1d[(i * shape_value)..((i + 1) * shape_value)]).to_vec());
    }
    result
}
