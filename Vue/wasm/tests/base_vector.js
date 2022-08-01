import { BaseVector, add,sub,mul,mm,div } from "geo-vector";
import { memory } from "geo-vector/geo_vector_bg.wasm";
//This is a test file.

export function arraytest_1() {
    //test array
    let _test_array1 = []
    let _test_array2 = []
    for (let i = 0; i < 360 * 360; i++) { _test_array1.push(Math.random()); _test_array2.push(Math.random()) }
    console.log(_test_array1.length)
    let test_array1 = BaseVector.new(360, 360, _test_array1)
    let test_array2 = BaseVector.new(360, 360, _test_array2)
    test_array1.mul(test_array2)
    console.log('mul_test', test_array1.get(0, 0) == _test_array1[0] * _test_array2[0])
    //720*720 ~4s、360*360 ~0.5s
    //test_array1.mm(test_array2)

    //fixed data
    let test_array_3 = BaseVector.new(2, 3, [NaN, 2, 3, 4, 5, 6]);
    let test_array_4 = BaseVector.new(2, 3, [7, 8, 9, 10, 11, 12]);
    test_array_3.mul(test_array_4)
    test_array_3.log_data_string()

    //mm
    let test_array_5 = BaseVector.new(3, 2, [1, 2, 3, 4, 5, 6]);
    let test_array_6 = BaseVector.new(2, 3, [7, 8, 9, 10, NaN, 12]);
    test_array_5.mm(test_array_6)
    test_array_5.log_data_string()

}

export function memory_test() {

    let test_array_5 = BaseVector.new(3, 2, [1, 2, 3, 4, 5, 6]);
    let test_array_6 = BaseVector.new(2, 3, [7, 8, 9, 10, 11, 12]);
    test_array_5.mm(test_array_6)
    console.log({ memory })
    const vector = new Float64Array(memory.buffer, test_array_5.get_ptr(), test_array_5.get_cols() * test_array_5.get_rows());
    for (let i in vector) { console.log(vector[i]) }
    console.log({ vector, test_array_5 })

}

export function array_function_calculation_test() {
    let _test_array1 = []
    let _test_array2 = []
    let _test_array3 = []
    for (let i = 0; i < 360 * 360; i++) {
        _test_array1.push(300 + Math.random());
        _test_array2.push(100 + Math.random());
        _test_array3.push(200 + Math.random());
    }
    let test_array1 = BaseVector.new(360, 360, _test_array1)
    let test_array2 = BaseVector.new(360, 360, _test_array2)
    let test_array3 = BaseVector.new(360, 360, _test_array3)
    console.log(add(test_array2, test_array3).get_index(0))
    let l = [test_array1,test_array2,test_array3]
    console.log(l.reduce((a,b)=>{return sub(a,b)}).get_index(0))
    console.log(l.reduce((a,b)=>{return add(a,b)}).get_index(0))
    console.log(l.reduce((a,b)=>{return div(a,b)}).get_index(0))
    console.log(l.reduce((a,b)=>{return mul(a,b)}).get_index(0))
}

export function padding_conv2d_test(){
    let _test_array1 = []
    for (let i = 0; i < 360 * 360; i++) {
        _test_array1.push(300 + i);
    }
    let test_array1 = BaseVector.new(50, 50, _test_array1)
    test_array1.padding(0);
    test_array1.conv2d([0.5,0.5,0.5,0.5,0.5,0.5,0.5,0.5,0.5],1)
    console.log(test_array1)


}