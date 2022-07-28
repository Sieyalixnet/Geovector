mod utils;
use std::fmt;
use wasm_bindgen::prelude::*;
extern crate web_sys;
use web_sys::console;
extern crate wee_alloc;
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
macro_rules! log {
       ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

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
#[wasm_bindgen]
pub fn test() -> Vec<i64> {
    log!("hello wasm");
    let mut a = Vec::new();
    a.push(10);
    return a;
}

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct BaseVector {
    total_rows: usize,
    total_cols: usize,
    data: Vec<f64>,
}

#[wasm_bindgen]
impl BaseVector {
    pub fn new(total_rows: usize, total_cols: usize, data: Vec<f64>) -> BaseVector {
        BaseVector {
            total_rows,
            total_cols,
            data,
        }
    }
    pub fn log_data_string(&self) {
        let mut string = String::new();
        for i in 0..self.data.len() {
            string.push_str(&format!("{} , ", self.data[i].to_string()));
        }
        log!("LOG_FROM WASM: data is {}", string);
    }
    pub fn data_string(&self) -> String {
        let mut string = String::new();
        for i in 0..self.data.len() {
            string.push_str(&format!("{} , ", self.data[i].to_string()));
        }
        string
    }

    //Some getter and Setter
    pub fn set(&mut self, rows: usize, cols: usize, set_data: f64) {
        let position = rows * self.total_cols + cols;
        self.data[position] = set_data;
    }
    pub fn get(&self, rows: usize, cols: usize) -> f64 {
        let position = rows * self.total_cols + cols;
        return self.data[position];
    }
    pub fn set_index(&mut self, index: usize, set_data: f64) {
        self.data[index] = set_data;
    }
    pub fn get_index(&self, index: usize) -> f64 {
        self.data[index]
    }

    pub fn get_rows(&self) -> usize {
        return self.total_rows;
    }
    pub fn get_cols(&self) -> usize {
        return self.total_cols;
    }
    pub fn get_ptr(&self) -> *const f64 {
        self.data.as_ptr()
    }
    pub fn total_cols(&self) -> usize {
        self.total_cols
    }
    pub fn total_rows(&self) -> usize {
        self.total_rows
    }

    //matrix operation
    //in this part, if some operation will change the shape of the matrix, we should also modify the total_rows and total_cols manually.
    pub fn mul(&mut self, b: &BaseVector) {
        let _timer = Timer::new("mul");
        self.data = self
            .data
            .iter()
            .zip(b.data.iter())
            .map(|(a, b)| a * b)
            .collect();
    }
    pub fn divide(&mut self, b: &BaseVector) {
        self.data = self
            .data
            .iter()
            .zip(b.data.iter())
            .map(|(a, b)| a / b)
            .collect();
    }
    pub fn plus(&mut self, b: &BaseVector) {
        self.data = self
            .data
            .iter()
            .zip(b.data.iter())
            .map(|(a, b)| a + b)
            .collect();
    }
    pub fn minus(&mut self, b:&BaseVector) {
        self.data = self
            .data
            .iter()
            .zip(b.data.iter())
            .map(|(a, b)| a - b)
            .collect();
    }
    pub fn mm(&mut self, _b: &BaseVector) {
        let _timer = Timer::new("mm");
        let a = self.reshape(self.total_cols);
        let b = _b.reshape(_b.total_cols);
        let mut c = vec![vec![0.0; b[0].len()]; a.len()];
        for i in 0..c.len() {
            for j in 0..c[0].len() {
                for k in 0..a[0].len() {
                    c[i][j] += a[i][k] * b[k][j];
                }
            }
        }
        self.total_rows = c.len();
        self.total_cols = c[0].len();
        self.data = c.into_iter().flat_map(|x| x).collect();
    }
}

impl BaseVector {
    pub fn reshape(&self, shape_value: usize) -> Vec<Vec<f64>> {
        let mut result: Vec<Vec<f64>> = Vec::new();
        let pieces = self.data.len() / shape_value;
        for i in 0..pieces {
            result.push((&self.data[(i * shape_value)..((i + 1) * shape_value)]).to_vec());
        }
        result
    }
}
