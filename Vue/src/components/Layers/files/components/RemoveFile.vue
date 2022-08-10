<template>
  <div id="RemoveFileContent">
            <LabelFileSelector
      v-model:value="SelectedFile"
      :list="List"
      :label="`File Name`"
      class="BlockMargin selectfile"
    ></LabelFileSelector>
    <button
      style="padding: 0.625rem"
      @click="exec_operation()"
      class="BlockMargin"
    >
      Delete
    </button>
  </div>
</template>

<script setup>
import {inject,ref} from "vue";
import LabelFileSelector from "../layers/components/components/LabelFileSelector.vue";;
let ImageFileList = inject("ImageFileList");
const { List } = ImageFileList;
let WASM_Module = inject("WASM_Module");
const { WASM } = WASM_Module;
let SelectedFile = ref("");
let exec_operation = () => {
      let targetFile = List.find((item) => item.name === SelectedFile.value);
      targetFile.delete_all();
      List.splice(List.indexOf(targetFile), 1);
};
</script>

<style lang="scss" scoped>
#RemoveFileContent {
  display: flex;
  flex-direction: row;
  align-items: center;
}
</style>