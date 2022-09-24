<template>
  <div id="CreateFileContent">
    <button
      style="padding: 0.625rem"
      @click="exec_operation()"
      class="BlockMargin"
    >
      Create
    </button>
  </div>
</template>

<script setup>
import LabelFileSelector from "../layers/components/components/LabelFileSelector.vue";
import { inject, ref } from "vue";
let ImageFileList = inject("ImageFileList");
const { List } = ImageFileList;
let WASM_Module = inject("WASM_Module");
const { WASM } = WASM_Module;


let exec_operation = () => {
  let result = new WASM.ImageVector();
  List.push(result);
};
</script>

<style lang="scss" scoped>
#CreateFileContent {
  display: flex;
  flex-direction: row;
  align-items: center;
}
</style>