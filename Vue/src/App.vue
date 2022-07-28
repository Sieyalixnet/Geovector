<template>
  <div>
    <TheHeader></TheHeader>
    <button @click="handleClick">Click me</button>
    <TheFooter></TheFooter>
  </div>
</template>

<script setup>
import TheHeader from "./components/Global/TheHeader.vue";
import TheFooter from "./components/Global/TheFooter.vue";
import { inject, onMounted, provide, reactive } from "vue";
const GP = inject("GP");
let data_local = reactive({ WASM: null });
let initWASM = async () => {
  return await import("../wasm/index.js");
};
onMounted(() => {
  initWASM().then((wasm) => (data_local.WASM = wasm)); //注意这必须引入reactive，在main.js其实也可以，但是由于没有reactive，所以在更新之后，Provide的仍然是一个undefined
});
let handleClick = () => {
  // let BaseVector = data_local.WASM.BaseVector;
  // let test_array_5 = BaseVector.new(3, 2, [1, 2, 3, 4, 5, 6]);
  // let test_array_6 = BaseVector.new(2, 3, [7, 8, 9, 10, 11, 12]);
  // test_array_5.mm(test_array_6);
  // test_array_5.data_string();
  // console.log(test_array_5.get_rows(), test_array_5.get_cols());
  const { Vector, createVector } = data_local.WASM;
  console.log(data_local.WASM);
  let a = [
    [1, 2, 3],
    [4, 5, 6],
  ];
  let b = [
    [1, 2],
    [3, 4],
    [5, 6],
  ];
  let a1 = createVector(a);
  let b1 = createVector(b);
  a1.mm(b1);
  for (let i in a1.memoryArray()) {
    console.log(a1.memoryArray()[i]);
  }
};
provide("WASM", data_local.WASM);
</script>

<style lang="scss">
*{margin:0px}

#app {
  font-family: Avenir, Helvetica, Arial, sans-serif;
  color: #2c3e50;
}
</style>
