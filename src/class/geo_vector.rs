use crate::{
    log,
    tools::tools::{reflect_to, reshape, Timer},
};
use wasm_bindgen::prelude::*;

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
    pub fn clone_self(&self) -> BaseVector {
        return self.clone();
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
    pub fn clear(&self) {
        for i in 0..self.data.len(){
            drop(self.data[i]);
        };
        drop(self.total_rows);
        drop(self.total_cols);
    }
    // a JSarray Example
    // pub fn reset() -> js_sys::Array {
    //     let a = js_sys::Array::new();
    //     let b = JsValue::from_f64(10.0);
    //     a.push(&b);
    //     return a;
    // }

    //PARTS: Some getter and Setter
    pub fn set(&mut self, rows: usize, cols: usize, set_data: f64) {
        let position = rows * self.total_cols + cols;
        if position < self.data.len() {
            self.data[position] = set_data;
        }
    }
    pub fn get(&self, rows: usize, cols: usize) -> f64 {
        let position = rows * self.total_cols + cols;
        let result = self.data.get(position);
        match result {
            Some(result) => *result,
            None => 0.0,
        }
    }
    pub fn set_index(&mut self, index: usize, set_data: f64) {
        if (index < self.data.len()) {
            self.data[index] = set_data;
        }
    }
    pub fn get_index(&self, index: usize) -> f64 {
        let result = self.data.get(index);
        match result {
            Some(result) => *result,
            None => 0.0,
        }
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
    pub fn get_max(&self) -> f64 {
        let data = self.data.clone();
        let max = data.iter().cloned().fold(std::f64::MIN, |a, b| a.max(b));
        max
    }
    pub fn get_min(&self) -> f64 {
        let data = self.data.clone();
        let min = data.iter().cloned().fold(std::f64::MAX, |a, b| a.min(b));
        min
    }
    //PARTS: matrix operation
    //in this part, if some operation will change the shape of the matrix, we should also modify the total_rows and total_cols manually.
    pub fn mul(&mut self, b: &BaseVector) {
        self.data = self
            .data
            .iter()
            .zip(b.data.iter())
            .map(|(a, b)| a * b)
            .collect();
    }
    pub fn div(&mut self, b: &BaseVector) {
        self.data = self
            .data
            .iter()
            .zip(b.data.iter())
            .map(|(a, b)| a / b)
            .collect();
    }
    pub fn add(&mut self, b: &BaseVector) {
        self.data = self
            .data
            .iter()
            .zip(b.data.iter())
            .map(|(a, b)| a + b)
            .collect();
    }
    pub fn sub(&mut self, b: &BaseVector) {
        self.data = self
            .data
            .iter()
            .zip(b.data.iter())
            .map(|(a, b)| a - b)
            .collect();
    }
    pub fn mm(&mut self, _b: &BaseVector) {
        let _timer = Timer::new("mm");
        let a = reshape(&self.data, self.total_cols);
        let b = reshape(&_b.data, _b.total_cols);
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

    pub fn add_value(&mut self, b: f64) {
        for i in 0..self.data.len() {
            self.data[i] += b;
        }
    }
    pub fn sub_value(&mut self, b: f64) {
        for i in 0..self.data.len() {
            self.data[i] -= b;
        }
    }
    pub fn mul_value(&mut self, b: f64) {
        for i in 0..self.data.len() {
            self.data[i] *= b;
        }
    }
    pub fn div_value(&mut self, b: f64) {
        for i in 0..self.data.len() {
            self.data[i] /= b;
        }
    }
    pub fn replace(&mut self, min_range: f64, max_range: f64, new_value: f64) {
        self.data = self
            .data
            .iter()
            .map(|x| {
                if x >= &min_range && x <= &max_range {
                    new_value
                } else {
                    *x
                }
            })
            .collect();
    }
    pub fn range_reflect(
        &mut self,
        min_range: f64,
        max_range: f64,
        min_reflect: f64,
        max_reflect: f64,
    ) {


        self.data = self
            .data
            .iter()
            .map(|x| {
                if x >= &min_range && x <= &max_range {
                    let temp = (x - min_range) / (max_range - min_range);
                    return temp * (max_reflect - min_reflect) + min_reflect;
                } else {
                    *x
                }
            })
            .collect();
    }
    pub fn abs(&mut self) {
        self.data = self.data.iter().map(|x| x.abs()).collect();
    }

    //PARTS: matrix/science calculate
    pub fn padding(&mut self, padding_value: f64) {
        let mut new_data = self.reshape(self.total_cols);
        for row in &mut new_data {
            row.push(padding_value);
            row.insert(0, padding_value);
        }
        let new_length = new_data[0].len();
        let padding_row = vec![padding_value; new_length];
        new_data.insert(0, padding_row.clone());
        new_data.push(padding_row.clone());

        self.total_rows = new_data.len();
        self.total_cols = new_data[0].len();
        self.data = new_data.into_iter().flat_map(|x| x).collect();
    }
    pub fn padding_times(&mut self, padding_value: f64, times: usize) {
        let mut new_data = self.reshape(self.total_cols);
        for row in &mut new_data {
            for _ in 0..times {
                row.push(padding_value);
                row.insert(0, padding_value);
            }
        }
        let new_length = new_data[0].len();
        let padding_row = vec![padding_value; new_length];
        for _ in 0..times {
            new_data.insert(0, padding_row.clone());
            new_data.push(padding_row.clone());
        }

        self.total_rows = new_data.len();
        self.total_cols = new_data[0].len();
        self.data = new_data.into_iter().flat_map(|x| x).collect();
    }
    pub fn conv2d_to_array(&self, kernel: Vec<f64>, stride: usize) -> Vec<f64> {
        let kernel = reshape(&kernel, (kernel.len() as f64).sqrt().floor() as usize);
        let mut result: Vec<f64> = Vec::new();

        let mut row_lock: usize = 0; //Locked row
        let mut row_lock_locked: bool = false; //In fact, it will not affect anything. But this lock is needed in this process to make sure locked row will not be modified again.
        let mut column_number: usize = 0;
        for row in (0..self.total_rows()).step_by(stride) {
            for col in (0..self.total_cols()).step_by(stride) {
                let value = self._conv2d(row, col, &kernel);
                match value {
                    Some((v, _row)) => {
                        result.push(v);
                        if row_lock == 0 && !row_lock_locked {
                            row_lock = _row;
                            row_lock_locked = true;
                        }
                        if row_lock == _row {
                            column_number += 1;
                        }
                    }
                    None => {}
                }
            }
        }
        println!("column_number is {}", column_number);
        result
    }
    pub fn conv2d(&mut self, kernel: Vec<f64>, stride: usize) {
        let kernel = reshape(&kernel, (kernel.len() as f64).sqrt().floor() as usize);
        let mut result: Vec<f64> = Vec::new();

        let mut row_lock: usize = 0; //Locked row
        let mut row_lock_locked: bool = false; //In fact, it will not affect anything. But this lock is needed in this process to make sure locked row will not be modified again.
        let mut column_number: usize = 0;
        for row in (0..self.total_rows()).step_by(stride) {
            for col in (0..self.total_cols()).step_by(stride) {
                let value = self._conv2d(row, col, &kernel);
                match value {
                    Some((v, _row)) => {
                        result.push(v);
                        if row_lock == 0 && !row_lock_locked {
                            row_lock = _row;
                            row_lock_locked = true;
                        }
                        if row_lock == _row {
                            column_number += 1;
                        }
                    }
                    None => {}
                }
            }
        }
        self.total_cols = column_number;
        self.total_rows = result.len() / column_number;
        self.data = result;
    }
    pub fn normalize(&mut self, normalize_type: &str) {
        let data = self.data.clone();
        let mut result = Vec::new();
        match normalize_type {
            "min_max" => {
                //fold like reduce
                let min = data.iter().cloned().fold(std::f64::MAX, |a, b| a.min(b));
                let max = data.iter().cloned().fold(std::f64::MIN, |a, b| a.max(b));
                result = data.into_iter().map(|x| (x - min) / (max - min)).collect();
            }
            "z_score" => {
                let mean = data.iter().cloned().fold(0.0, |a, b| a + b) / data.len() as f64;
                let std = data
                    .iter()
                    .cloned()
                    .fold(0.0, |a, b| a + ((b - mean) * (b - mean)))
                    / data.len() as f64;
                result = data
                    .into_iter()
                    .map(|x| (x - mean) / (std.sqrt()))
                    .collect();
            }
            _ => {
                result = data;
            }
        };
        self.data = result;
    }

    pub fn transpose(&mut self) {
        let origin_data = self.reshape(self.total_cols);
        let mut result: Vec<Vec<f64>> = vec![];
        for i in 0..origin_data[0].len() {
            let mut temp: Vec<f64> = vec![];
            for j in 0..origin_data.len() {
                temp.push(origin_data[j][i]);
            }
            result.push(temp);
        }
        self.total_rows = result.len();
        self.total_cols = result[0].len();
        self.data = result.into_iter().flat_map(|x| x).collect();
    }

    pub fn reflect_to(&mut self, min_reflect: f64, max_reflect: f64) {
        let data = self.data.clone();

        let min = data.iter().cloned().fold(std::f64::MAX, |a, b| a.min(b));
        let max = data.iter().cloned().fold(std::f64::MIN, |a, b| a.max(b));

        let result = data
            .into_iter()
            .map(|x| (x - min) / (max - min))
            .map(|x| x * (max_reflect - min_reflect) + min_reflect)
            .collect();
        self.data = result;
    }
    pub fn rescale_to(&mut self, ratio: usize) {
        //TODO need to calculate the real pixel value
        let temp = self.reshape(self.total_cols);
        let mut temp_rows: Vec<Vec<f64>> = vec![];
        for i in 0..temp.len() {
            if i % ratio == 0 {
                let mut new_row: Vec<f64> = vec![];
                for j in 0..temp[i].len() {
                    if j % ratio == 0 {
                        new_row.push(temp[i][j].clone());
                    }
                }
                temp_rows.push(new_row);
            }
        }
        self.total_rows = temp_rows.len();
        self.total_cols = temp_rows[0].len();
        self.data = temp_rows.into_iter().flat_map(|x| x).collect();
    }

    pub fn reverse_vertical(&mut self) {
        self.data = self
            .reshape(self.total_cols)
            .into_iter()
            .rev()
            .flat_map(|x| x)
            .collect();
    }

    pub fn reverse_horizontal(&mut self) {
        let temp = self.reshape(self.total_cols);
        let mut result: Vec<Vec<f64>> = vec![];
        for i in 0..temp.len() {
            result.push(temp[i].clone().into_iter().rev().collect());
        }
        self.data = result.into_iter().flat_map(|x| x).collect();
    }

    //PARTS: Outputs
    //WARNING: DO NOT USE THE METHODS ABOVE HERE, IT WILL LEAD TO MEMORY LEAKS.
    // pub fn render(&self, reflect: bool) -> Vec<u8> {
    //     //TODO need to calculate the real pixel value
    //     let mut result: Vec<u8> = Vec::new();

    //     let data = if reflect {
    //         reflect_to(self.data.clone(), 0.0, 255.0)
    //     } else {
    //         self.data.clone()
    //     };

    //     for i in 0..self.data.len() {
    //         result.push((data[i]).round() as u8); //*255?
    //         result.push((data[i]).round() as u8);
    //         result.push((data[i]).round() as u8);
    //         result.push(255);
    //     }
    //     result
    // }
    // pub fn render_thumbnails(&self, ratio: usize, reflect: bool) -> Vec<u8> {
    //     //TODO need to calculate the real pixel value
    //     let temp = self.reshape(self.total_cols);
    //     let mut temp_rows: Vec<Vec<f64>> = vec![];
    //     for i in 0..temp.len() {
    //         if i % ratio == 0 {
    //             let mut new_row: Vec<f64> = vec![];
    //             for j in 0..temp[i].len() {
    //                 if j % ratio == 0 {
    //                     new_row.push(temp[i][j].clone());
    //                 }
    //             }
    //             temp_rows.push(new_row);
    //         }
    //     }
    //     let mut temp_rows: Vec<f64> = temp_rows.into_iter().flat_map(|x| x).collect();
    //     if reflect {
    //         temp_rows = reflect_to(temp_rows, 0.0, 255.0);
    //     }
    //     let mut result: Vec<u8> = Vec::new();
    //     for i in 0..temp_rows.len() {
    //         result.push((temp_rows[i]).round() as u8); //*255?
    //         result.push((temp_rows[i]).round() as u8);
    //         result.push((temp_rows[i]).round() as u8);
    //         result.push(255);
    //     }
    //     result
    // }
}
// impl Drop for BaseVector{

//     fn drop(&mut self) {
//         log!("BaseVector dropped");
//     }
// }
impl BaseVector {
    pub fn reshape(&self, shape_value: usize) -> Vec<Vec<f64>> {
        let mut result: Vec<Vec<f64>> = Vec::new();
        let pieces = self.data.len() / shape_value;
        for i in 0..pieces {
            result.push((&self.data[(i * shape_value)..((i + 1) * shape_value)]).to_vec());
        }
        result
    }
    pub fn get_data(&self) -> Vec<f64> {
        self.data.clone()
    }

    fn _conv2d(&self, row: usize, column: usize, kernel: &Vec<Vec<f64>>) -> Option<(f64, usize)> {
        let mut count: f64 = 0.0;
        let kernel_length = kernel.len();
        let kernel_range: core::ops::RangeInclusive<i32>;
        match kernel_length % 2 {
            0 => {
                kernel_range =
                    (-((kernel_length / 2) as i32)..=((kernel_length / 2) as i32 - 1)).into_iter();
            }
            1 => {
                kernel_range =
                    (-((kernel_length / 2) as i32)..=(kernel_length / 2) as i32).into_iter();
            }
            _ => {
                return None; //This should be paied attention if return a unexpected value
            }
        }
        for delta_rows in kernel_range.clone() {
            for delta_cols in kernel_range.clone() {
                let neighbor_row = row as i32 + delta_rows;
                let neighbor_col = column as i32 + delta_cols;
                if (neighbor_row < 0 || neighbor_row >= self.total_rows as i32)
                    || (neighbor_col < 0 || neighbor_col >= self.total_cols as i32)
                {
                    return None;
                    //continue;//this can be other type of conv2d
                } else {
                    let idx = self.get_index_2d(neighbor_row, neighbor_col) as usize;
                    let kernel_value = kernel
                        .get((delta_rows + (kernel_length / 2) as i32) as usize)
                        .unwrap()
                        .get((delta_cols + (kernel_length / 2) as i32) as usize)
                        .unwrap();
                    count += self.data[idx] * kernel_value;
                }
            }
        }
        return Some(((count) / (kernel_length as f64).powi(2), row as usize)); //if we need ignore the center, we should change the kernel_length to kernel_length-1
    }
    pub fn get_index_2d(&self, rows: i32, cols: i32) -> i32 {
        let position = rows * self.total_cols as i32 + cols;
        return position;
    }
}

////! DO NOT USE ANY LOG!() WHEN USE `cargo test` ////
// THESE TEST CANNOT TEST ALL THE FUNCTIONS OR METHODS ON THIS FILE. YOU MAY NEED TO WRITE ANOTHER TESTS.
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_conv2d() {
        let mut test_vec = vec![0.0; 40];
        for i in 0..test_vec.len() {
            test_vec[i] = i as f64;
        }
        let mut a = BaseVector::new(5, 8, test_vec);
        let kernel = vec![0.5; 16];

        let b = a.conv2d_to_array(kernel.clone(), 1);
        println!("{:?}", &b.len());
        println!("{:?}", &b);

        a.conv2d(kernel, 2);
        println!("{:?}", &a.data.len());
        println!("{:?}", &a);
    }

    #[test]
    fn add_sub_mul_div() {
        let test_vec: Vec<f64> = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
        let mut a = BaseVector::new(2, 3, test_vec.clone());
        let mut b = BaseVector::new(2, 3, test_vec.clone());
        a.add(&b);
        assert_eq!(a.get_data(), vec![2.0, 4.0, 6.0, 8.0, 10.0, 12.0]);
        a.sub(&b);
        assert_eq!(a.get_data(), test_vec);
        a.div(&b);
        assert_eq!(a.get_data(), vec![1.0; 6]);
        a.mul(&b);
        assert_eq!(a.get_data(), b.get_data());
    }

    #[test]
    pub fn padding_test() {
        let mut test_vec = vec![0.0; 100];
        for i in 0..test_vec.len() {
            test_vec[i] = i as f64;
        }
        let mut a = BaseVector::new(10, 10, test_vec);
        a.padding(0.0);
        assert_eq!(a.total_cols, 12);
        assert_eq!(a.total_rows, 12);
        let kernel = vec![0.5; 9];
        a.conv2d(kernel, 1);
        assert_eq!(a.get_data().len(), 100);
        println!("{:?}", a.get_data().len());
        println!("col: {}, rows: {}", a.total_cols, a.total_rows);
    }

    // #[test]
    //Deprecated
    // pub fn render_thumbnails_test() {
    //     let mut test_vec = vec![0.0; 100];
    //     for i in 0..test_vec.len() {
    //         test_vec[i] = i as f64;
    //     }
    //     let a = BaseVector::new(10, 10, test_vec);
    //     let result = a.render_thumbnails(4, false);
    //     println!("lenth is {}", result.len());
    //     println!("{:?}", result);
    // }

    #[test]
    pub fn transpose_test() {
        let mut a = BaseVector::new(2, 3, vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);
        a.transpose();
        assert_eq!(a.total_cols, 2);
        assert_eq!(a.total_rows, 3);
        assert_eq!(a.get_data(), vec![1.0, 4.0, 2.0, 5.0, 3.0, 6.0]);
        println!("{:?}", a.clone().reshape(a.total_cols));
    }

    #[test]
    pub fn reflect_normalize_test() {
        let mut a = BaseVector::new(2, 3, vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);
        // let result = reflect_to(a.data.clone(), 0.0, 255.0);
        // println!("{:?}", result);
        a.normalize("min_max");
        println!("{:?}", a.get_data());
        assert_eq!(
            a.get_data().iter().fold(std::f64::MAX, |a, b| a.min(*b)),
            0.0
        );
        let mut a = BaseVector::new(2, 3, vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);
        // let result = reflect_to(a.data.clone(), 0.0, 255.0);
        // println!("{:?}", result);
        a.normalize("z_score");
        println!("{:?}", a.get_data());
        // assert_eq!(
        //     a.get_data().iter().fold(std::f64::MAX, |a, b| a.min(*b)),
        //     0.0
        // );
        let mut a = BaseVector::new(2, 3, vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);
        a.reflect_to(0.0, 255.0);
        println!("{:?}", a.get_data());
        assert_eq!(
            a.get_data().iter().fold(std::f64::MIN, |a, b| a.max(*b)),
            255.0
        );
    }

    #[test]
    pub fn calculate_value_test() {
        let mut a = BaseVector::new(2, 3, vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);
        a.add_value(1.0);
        assert_eq!(a.get_data(), vec![2.0, 3.0, 4.0, 5.0, 6.0, 7.0]);
        a.sub_value(1.0);
        assert_eq!(a.get_data(), vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);
        a.mul_value(2.0);
        assert_eq!(a.get_data(), vec![2.0, 4.0, 6.0, 8.0, 10.0, 12.0]);
        a.div_value(2.0);
        assert_eq!(a.get_data(), vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);
    }

    #[test]
    pub fn reverse_test() {
        let mut a = BaseVector::new(2, 3, vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);
        a.reverse_horizontal();
        assert_eq!(a.total_cols, 3);
        assert_eq!(a.total_rows, 2);
        assert_eq!(a.get_data(), vec![3.0, 2.0, 1.0, 6.0, 5.0, 4.0]);
        println!("{:?}", a.clone().reshape(a.total_cols));

        let mut b = BaseVector::new(2, 3, vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);
        b.reverse_vertical();
        assert_eq!(b.total_cols, 3);
        assert_eq!(b.total_rows, 2);
        assert_eq!(b.get_data(), vec![4.0, 5.0, 6.0, 1.0, 2.0, 3.0]);
        println!("{:?}", b.clone().reshape(a.total_cols));
    }

    #[test]
    pub fn replace_abs_test() {
        let mut a = BaseVector::new(2, 3, vec![-1.0, -2.0, -3.0, -4.0, 5.0, 6.0]);
        a.replace(5.0, 6.0, 0.0);
        assert_eq!(a.get_data(), vec![-1.0, -2.0, -3.0, -4.0, 0.0, 0.0]);
        a.abs();
        assert_eq!(a.get_data(), vec![1.0, 2.0, 3.0, 4.0, 0.0, 0.0]);
    }
    #[test]
    pub fn range_reflect_test() {
        let mut a = BaseVector::new(2, 4, vec![-30.0,-20.0, -10.0, 0.0, 10.0, 20.0, 30.0,40.0]);
        a.range_reflect(-20.0, 20.0, 100.0, 200.0);
        println!("{:?}", a.get_data());
    }
}
