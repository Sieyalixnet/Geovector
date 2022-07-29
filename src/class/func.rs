use crate::class::geo_vector::BaseVector;
use wasm_bindgen::prelude::*;

// #[wasm_bindgen]
// pub fn cal(a:&BaseVector,b:&BaseVector,t:&str) -> BaseVector {
//     //let c= BaseVector::new(a.get_rows(),a.get_cols(),vec![]);
//     let a_data = a.get_data();
//     let b_data = b.get_data();
//     let c_data= a_data.iter()
//             .zip(b_data.iter());

//     let c = match t {
//         "mul" => {
//             let collect = c_data.map(|(a, b)| a * b).collect();
//             let c= BaseVector::new(a.get_rows(),a.get_cols(),collect);
//             c
//         }
//         "add" => {
//             let collect = c_data.map(|(a, b)| a + b).collect();
//             let c= BaseVector::new(a.get_rows(),a.get_cols(),collect);
//             c
//         }
//         "minus" => {
//             let collect = c_data.map(|(a, b)| a - b).collect();
//             let c= BaseVector::new(a.get_rows(),a.get_cols(),collect);
//             c
//         }
//         "divide" => {
//             let collect = c_data.map(|(a, b)| a / b).collect();
//             let c= BaseVector::new(a.get_rows(),a.get_cols(),collect);
//             c
//         }
//         "mm" => {
//             let a = a.reshape(a.get_cols());
//             let b = b.reshape(b.get_cols());
//             let mut c = vec![vec![0.0; b[0].len()]; a.len()];
//             for i in 0..c.len() {
//                 for j in 0..c[0].len() {
//                     for k in 0..a[0].len() {
//                         c[i][j] += a[i][k] * b[k][j];
//                     }
//                 }
//             }
//             let result= BaseVector::new(c.len(),c[0].len(),c.into_iter().flat_map(|x| x).collect());
//             result
//         }
//         other => {panic!("{} is not supported",other)}
//     };

//     c

// }

#[wasm_bindgen]
pub fn add(a: &BaseVector, b: &BaseVector) -> BaseVector {
    //let c= BaseVector::new(a.get_rows(),a.get_cols(),vec![]);
    let a_data = a.get_data();
    let b_data = b.get_data();
    let c_data = a_data
        .iter()
        .zip(b_data.iter())
        .map(|(a, b)| a + b)
        .collect();
    let c = BaseVector::new(a.get_rows(), a.get_cols(), c_data);
    c
}

#[wasm_bindgen]
pub fn div(a: &BaseVector, b: &BaseVector) -> BaseVector {
    //let c= BaseVector::new(a.get_rows(),a.get_cols(),vec![]);
    let a_data = a.get_data();
    let b_data = b.get_data();
    let c_data = a_data
        .iter()
        .zip(b_data.iter())
        .map(|(a, b)| a / b)
        .collect();
    let c = BaseVector::new(a.get_rows(), a.get_cols(), c_data);
    c
}

#[wasm_bindgen]
pub fn mul(a: &BaseVector, b: &BaseVector) -> BaseVector {
    //let c= BaseVector::new(a.get_rows(),a.get_cols(),vec![]);
    let a_data = a.get_data();
    let b_data = b.get_data();
    let c_data = a_data
        .iter()
        .zip(b_data.iter())
        .map(|(a, b)| a * b)
        .collect();
    let c = BaseVector::new(a.get_rows(), a.get_cols(), c_data);
    c
}

#[wasm_bindgen]
pub fn sub(a: &BaseVector, b: &BaseVector) -> BaseVector {
    //let c= BaseVector::new(a.get_rows(),a.get_cols(),vec![]);
    let a_data = a.get_data();
    let b_data = b.get_data();
    let c_data = a_data
        .iter()
        .zip(b_data.iter())
        .map(|(a, b)| a - b)
        .collect();
    let c = BaseVector::new(a.get_rows(), a.get_cols(), c_data);
    c
}

#[wasm_bindgen]
pub fn mm(a: &BaseVector, b: &BaseVector) -> BaseVector {
    let a = a.reshape(a.get_cols());
    let b = b.reshape(b.get_cols());
    let mut c = vec![vec![0.0; b[0].len()]; a.len()];
    for i in 0..c.len() {
        for j in 0..c[0].len() {
            for k in 0..a[0].len() {
                c[i][j] += a[i][k] * b[k][j];
            }
        }
    }
    let result = BaseVector::new(c.len(), c[0].len(), c.into_iter().flat_map(|x| x).collect());
    result
}

