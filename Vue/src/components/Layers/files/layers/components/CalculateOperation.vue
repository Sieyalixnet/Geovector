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
    ></LabelLayerSelector>

    <div>
      <button
        style="padding: 0.625rem"
        @click="exec_calculate()"
        class="BlockMargin"
      >
        M
      </button>
    </div>
    <button @click="test()">test</button>
  </div>
</template>

<script setup>
import { inject, ref } from "vue";
import LabelSelector from "./components/LabelSelector.vue";
import LabelLayerSelector from "./components/LabelLayerSelector.vue";
const props = defineProps(["layer"]);
let ImageFileList = inject("ImageFileList");
const { ALL_Channel_List } = ImageFileList;
let selectedMatrixName = ref();
let operation = ref("Select an operation");
let operations=["Add","Sub","Mul","Div","MM"];

let exec_calculate=()=>{
  let selected_martix=  ALL_Channel_List.find((item) => {
    if (item.OptionalAttributes.name == selectedMatrixName.value) {
      return item
    }
  });

  switch(operation.value){
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
  }

}

</script>

<style lang="scss" scoped>
#mmContent {
  display: flex;
  flex-direction: row;
  align-items: center;
}
</style>