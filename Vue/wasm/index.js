export {BaseVector} from "geo-vector";
export {memory} from "geo-vector/geo_vector_bg.wasm";
export * from "./class/index";
import { arraytest_1,memory_test } from "./tests/base_vector";
arraytest_1();memory_test();

