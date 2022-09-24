<template>
<div id="DownloadDataContent">
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
        Download
      </button>
</div>

</template>

<script setup>
import LabelFileSelector from "../layers/components/components/LabelFileSelector.vue";
import {inject,ref} from "vue";
let ImageFileList = inject("ImageFileList");
const { List } = ImageFileList;
// let WASM_Module=inject("WASM_Module");
// const {WASM} = WASM_Module

let SelectedFile = ref("");

let exec_operation = () => {
      let targetFile = List.find((item) => item.name === SelectedFile.value)
      targetFile.json_to_download()
};
</script>

<style lang="scss" scoped>
#DownloadDataContent {
  display: flex;
  flex-direction: row;
  align-items: center;
  .selectfile{
    :deep() select{max-width: 200px;}

  }
}
</style>