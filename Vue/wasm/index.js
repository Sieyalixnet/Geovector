export { BaseVector } from "geo-vector";
export { memory } from "geo-vector/geo_vector_bg.wasm";
export * from "./class/index";

//import exposed function in Rust as a module and export them.
//decrepated soon. because it will cause memory leak.
// import { add, sub, mul, div } from "geo-vector";
// export const cal = { add, sub, mul, div };


//TEST
// import { arraytest_1, memory_test, array_function_calculation_test,padding_conv2d_test } from "./tests/base_vector";
// arraytest_1(); memory_test(); array_function_calculation_test();padding_conv2d_test()

