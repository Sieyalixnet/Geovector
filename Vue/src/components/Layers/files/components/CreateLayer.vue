<template>
  <div id="DownloadDataContent">
    <LabelInput
      type="number"
      class="BlockMargin"
      label="Cols(Width)"
      v-model:value="creatVector.cols"
    />
    <LabelInput
      type="number"
      class="BlockMargin"
      label="Rows(Height)"
      v-model:value="creatVector.rows"
    />
    <LabelInput
      type="number"
      class="BlockMargin"
      label="Value"
      v-model:value="creatVector.value"
    />

    <LabelFileSelector
      v-model:value="SelectedFile"
      :list="List"
      label="File Name"
      class="BlockMargin selectfile"
    ></LabelFileSelector>
    <button
      style="padding: 0.625rem"
      @click="exec_operation()"
      class="BlockMargin"
    >
      Download
    </button>
  </div>
</template>

<script setup>
import LabelInput from "../layers/components/components/LabelInput.vue";
import LabelFileSelector from "../layers/components/components/LabelFileSelector.vue";
import { inject, reactive, ref } from "vue";
let ImageFileList = inject("ImageFileList");
const { List } = ImageFileList;
let WASM_Module = inject("WASM_Module");
const { WASM } = WASM_Module;
let creatVector = reactive({
  cols: 10,
  rows: 10,
  value: 255,
});
let SelectedFile = ref("");

let exec_operation = () => {
  if(creatVector.cols<=0 || creatVector.rows<=0){
    alert("Cols and Rows must be greater than 0");
    return;
  }

  let v = new Array(Number(creatVector.cols) * Number(creatVector.rows)).fill(
    Number(creatVector.value)
  );
  let newVector = WASM.createVector(v, creatVector.rows, creatVector.cols);
  let targetFile = List.find((item) => item.name === SelectedFile.value);
  targetFile.add_List(newVector);
};
</script>

<style lang="scss" scoped>
#DownloadDataContent {
  display: flex;
  flex-direction: row;
  align-items: center;
  .selectfile {
    :deep() select {
      max-width: 200px;
    }
  }
}
</style>