import { BaseVector } from "geo-vector";
import { memory } from "geo-vector/geo_vector_bg.wasm";
//This is a test file.

export function arraytest_1() {
    //test array
    let _test_array1 = []
    let _test_array2 = []
    for (let i = 0; i < 360*360; i++) { _test_array1.push(Math.random()); _test_array2.push(Math.random()) }
    console.log(_test_array1.length)
    let test_array1 = BaseVector.new(360, 360, _test_array1)
    let test_array2 = BaseVector.new(360, 360, _test_array2)
    test_array1.mul(test_array2)
    console.log('mul_test', test_array1.get(0, 0) == _test_array1[0] * _test_array2[0])
    //720*720 ~4s、360*360 ~0.5s
    //test_array1.mm(test_array2)

    //fixed data
    let test_array_3 = BaseVector.new(2, 3, [1, 2, 3, 4, 5, 6]);
    let test_array_4 = BaseVector.new(2, 3, [7, 8, 9, 10, 11, 12]);
    test_array_3.minus(test_array_4)
    test_array_3.log_data_string()

    //mm
    let test_array_5 = BaseVector.new(3, 2, [1, 2, 3, 4, 5, 6]);
    let test_array_6 = BaseVector.new(2, 3, [7, 8, 9, 10, 11, 12]);
    test_array_5.mm(test_array_6)
    test_array_5.log_data_string()

}

export function memory_test() {

    let test_array_5 = BaseVector.new(3, 2, [1, 2, 3, 4, 5, 6]);
    let test_array_6 = BaseVector.new(2, 3, [7, 8, 9, 10, 11, 12]);
    test_array_5.mm(test_array_6)
    console.log({ memory })
    const vector = new Float64Array(memory.buffer, test_array_5.get_ptr(), test_array_5.get_cols() * test_array_5.get_rows());
    for(let i in vector){console.log(vector[i])}
    console.log({ vector, test_array_5 })

}