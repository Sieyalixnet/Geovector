<template>
  <div id="LayerOperationContent">
    <LabelSelector
      v-model:value="operation"
      :list="operations"
      :label="`Operation`"
      class="BlockMargin"
    ></LabelSelector>
        <LabelFileSelector
        v-show="operation=='Move To'"
      v-model:value="SelectedFile"
      :list="List"
      :label="operation"
      class="BlockMargin"
    ></LabelFileSelector>
    <div>
      <button
        style="padding: 0.625rem"
        class="BlockMargin"
        @click="exec_operation()"
      >
        {{operation}}
      </button>
    </div>
  </div>
</template>

<script setup>
import LabelSelector from "./components/LabelSelector.vue";
import LabelFileSelector from "./components/LabelFileSelector.vue";
import { ref,inject} from "vue";
let ImageFileList = inject("ImageFileList");
const { List } = ImageFileList;
let SelectedFile = ref("");
const emits = defineEmits(["LayerOperation_exec"]);
let operation = ref("Select an operation");
let operations = ["Copy", "Upward", "Downward", "Delete","Move To"];
const props = defineProps(["layer"]);
let exec_operation = () => {
  if (operations.includes(operation.value)) {
    
    emits("LayerOperation_exec", {operation:operation.value,SelectedFile:SelectedFile.value});
  }
};
</script>

<style lang="scss" scoped>
#LayerOperationContent{
    display:flex;
    flex-direction:row;
    align-items:center;
    justify-content:flex-start;
}
</style>