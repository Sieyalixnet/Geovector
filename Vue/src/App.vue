<template>
  <div id="AppContent">
    <TheHeader></TheHeader>
    <!-- <button @click="handleClick">Click me</button> -->
    <!-- <button @click="handleClick2">Click me2</button> -->
    <Imagefile></Imagefile>
    <TheFooter></TheFooter>
  </div>
</template>

<script setup>
import TheHeader from "./components/Global/TheHeader.vue";
import TheFooter from "./components/Global/TheFooter.vue";
import Imagefile from "./components/Layers/ImageFile.vue";
import { ref, computed, inject, onMounted, provide, reactive } from "vue";
const GP = inject("GP");
let WASM_Module = reactive({ WASM: null });
let initWASM = async () => {
  return await import("../wasm/index.js");
};
let initWASM_Promise = () => {
  WASM_Module.WASM = null;
  initWASM().then((wasm) => (WASM_Module.WASM = wasm));
};

onMounted(() => {
  initWASM_Promise();
});

let a1;
let b1;

let handleClick = () => {//a test function
  // let BaseVector = data_local.WASM.BaseVector;
  // let test_array_5 = BaseVector.new(3, 2, [1, 2, 3, 4, 5, 6]);
  // let test_array_6 = BaseVector.new(2, 3, [7, 8, 9, 10, 11, 12]);
  // test_array_5.mm(test_array_6);
  // test_array_5.data_string();
  // console.log(test_array_5.get_rows(), test_array_5.get_cols());
  const { Vector, createVector } = WASM_Module.WASM;
  console.log(WASM_Module.WASM);
  let a = new Array(500000).fill([1, 2, 3]);
  console.log(a);
  let b = [
    [1, 2],
    [3, 4],
    [5, 6],
  ];
  a1 = createVector(a);
  console.log(a1);
  b1 = createVector(b);
  //a1.mm(b1);
  // for (let i in a1.memoryArray()) {
  //   console.log(a1.memoryArray()[i]);
  // }
};

let handleClick2 = () => {//a test function
  console.log(WASM_Module.WASM.exit);
  WASM_Module.WASM.exit();
  // console.log(a1)
  // a1.drop()
  // b1.drop()
  // console.log(a1.array())
};

provide("WASM_Module", WASM_Module);
</script>

<style lang="scss">
* {
  margin: 0px;
}

#app {
  font-family: Avenir, Helvetica, Arial, sans-serif;
  color: #2c3e50;
}
</style>

<style lang="scss" scoped>
</style>