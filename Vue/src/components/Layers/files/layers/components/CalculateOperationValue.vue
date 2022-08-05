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
        label="Value"
        v-model:value="calculate_value"

      />

    <div>
      <button
        style="padding: 0.625rem"
        @click="exec_calculate()"
        class="BlockMargin"
      >
        {{operation}}
      </button>
    </div>
  </div>
</template>

<script setup>
import { inject, ref } from "vue";
import LabelSelector from "./components/LabelSelector.vue";
import LabelInput from "./components/LabelInput.vue";
const props = defineProps(["layer"]);
let calculate_value = ref(1);
let operation = ref("Select an operation");
let operations=["Add","Sub","Mul","Div"];
const update_thumbnails=inject("update_thumbnails");
let exec_calculate=()=>{
  if(!Number(calculate_value.value)){
    alert("Please input a number");
    return;
  }
  switch(operation.value){
    case "Add":
      props.layer.add_value(calculate_value.value);
      break;
    case "Sub":
      props.layer.sub_value(calculate_value.value);
      break;
    case "Mul":
      props.layer.mul_value(calculate_value.value);
      break;
    case "Div":
      props.layer.div_value(calculate_value.value);
      break;

  }
      if(update_thumbnails.if){
      update_thumbnails.fn()
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