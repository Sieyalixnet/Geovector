<template>
  <div id="mmContent">
    <LabelSelector
      v-model:value="operation"
      :list="operations"
      :label="`Operation`"
      class="BlockMargin"
    ></LabelSelector>
          <LabelInput
        type="number"
        class="BlockMargin"
        label="Rescale to(px)"
        v-model:value="calculate_value"
        v-if="operation=='Rescale to'"
        placeholder="px"
      />
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
import LabelInput from "./components/LabelInput.vue";
const props = defineProps(["layer"]);
let ImageFileList = inject("ImageFileList");
const update_thumbnails=inject("update_thumbnails");
let operation = ref("Select an operation");
let operations = ["Transpose","Reverse Hori.","Reverse Vert.","Rescale to"];
let calculate_value = ref();


let exec_calculate = () => {
  if (!operations.includes(operation.value)) {
    alert("Please select an operation");
    return;
  }
  switch (operation.value) {
    case "Reverse Hori.":
        props.layer.reverse_horizontal()
        break;
    case "Reverse Vert.":
        props.layer.reverse_vertical()
        break;
    case "Transpose":
      props.layer.transpose();
      break;
    case "Rescale to":
      let scale = Number(calculate_value.value);
      if(isNaN(scale) || scale<=0){
        alert("Please enter a number bigger than 0.");
        return;
      }
      if(scale > Math.max(props.layer.get_cols(),props.layer.get_rows())){
        alert("Rescale only support to transform the data in a smaller size. Please enter a number smaller than the max size of the layer.");
        return;

      }
      props.layer.rescale_to(scale);
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