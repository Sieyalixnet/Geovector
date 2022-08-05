<template>
  <div id="mmContent">
    <LabelSelector
      v-model:value="operation"
      :list="operations"
      :label="`Operation`"
      class="BlockMargin"
    ></LabelSelector>
    <LabelLayerSelector
      v-model:value="selectedMatrixName"
      :list="ALL_Channel_List"
      :label="operation"
      class="BlockMargin"
      v-show="operation != 'Transpose'"
    ></LabelLayerSelector>

    <div>
      <button
        style="padding: 0.625rem"
        @click="exec_calculate()"
        class="BlockMargin"
      >
        {{ operation }}
      </button>
    </div>
  </div>
</template>

<script setup>
import { inject, ref } from "vue";
import LabelSelector from "./components/LabelSelector.vue";
import LabelLayerSelector from "./components/LabelLayerSelector.vue";
const props = defineProps(["layer"]);
let ImageFileList = inject("ImageFileList");
const update_thumbnails=inject("update_thumbnails");
const { ALL_Channel_List } = ImageFileList;
let selectedMatrixName = ref("");
let operation = ref("Select an operation");
let operations = ["Add", "Sub", "Mul", "Div", "MM", "Transpose"];

let exec_calculate = () => {
  if (props.layer.OptionalAttributes.name == selectedMatrixName.value) {
    alert(
      "Can not calculate with the same layer. Please select a different layer (you can firstly use Layer Operation to copy it). "
    );
    return;
  }

  let selected_martix;
  if (operation.value != "Transpose"){
    selected_martix = ALL_Channel_List.find((item) => {
      if (item.OptionalAttributes.name == selectedMatrixName.value) {
        return item;
      }
    });
  if (!selected_martix || !operations.includes(operation.value)) {
    alert("Please select an operation and a matrix");
    return;
  }}
  switch (operation.value) {
    case "Add":
      props.layer.add(selected_martix);
      break;
    case "Sub":
      props.layer.sub(selected_martix);
      break;
    case "Mul":
      props.layer.mul(selected_martix);
      break;
    case "Div":
      props.layer.div(selected_martix);
      break;
    case "MM":
      props.layer.mm(selected_martix);
      break;
    case "Transpose":
      props.layer.transpose();
      break;
  }
      if(update_thumbnails.if){
      update_thumbnails.fn()
    }
};
</script>

<style lang="scss" scoped>
#mmContent {
  display: flex;
  flex-direction: row;
  align-items: center;
}
</style>