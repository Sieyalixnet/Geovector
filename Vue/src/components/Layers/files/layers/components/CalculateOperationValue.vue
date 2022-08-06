<template>
  <div id="mmContent">
    <LabelSelector
      v-model:value="operation"
      :list="operations"
      :label="`Operation`"
      class="BlockMargin"
    ></LabelSelector>
      <LabelSelector
      v-model:value="NormalizeType"
      :list="NormalizeTypes"
      :label="`Normalize Type`"
      v-show="operation=='Normalize'"
      class="BlockMargin"
      ></LabelSelector>
      <LabelInput
        type="number"
        class="BlockMargin"
        label="Value"
        v-show="operation!='Normalize'"
        v-model:value="calculate_value[0]"

      />
      <LabelInput
        type="number"
        class="BlockMargin"
        label="Value"
        v-show="operation=='Reflect to'"
        v-model:value="calculate_value[1]"
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
let calculate_value = ref([1,1]);
let operation = ref("Select an operation");
let operations=["Add","Sub","Mul","Div","Normalize","Reflect to"];
let NormalizeTypes =  ["min_max","z_score"];
let NormalizeType = ref("max_min");
const update_thumbnails=inject("update_thumbnails");
let exec_calculate=()=>{
  // if(!Number(calculate_value.value)){
  //   alert("Please input a number");
  //   return;
  // }
  switch(operation.value){
    case "Add":
      props.layer.add_value(calculate_value.value[0]);
      break;
    case "Sub":
      props.layer.sub_value(calculate_value.value[0]);
      break;
    case "Mul":
      props.layer.mul_value(calculate_value.value[0]);
      break;
    case "Div":
      props.layer.div_value(calculate_value.value[0]);
      break;
    case "Reflect to":
      let min = Number(calculate_value.value[0]);
      let max = Number(calculate_value.value[1]);
      if(isNaN(min) || isNaN(max) ||  max-min<=0){
        alert("Please input valid numbers and value1(min) should less than value2(max). Please read Docs to determine how to input numbers.");
        return;
      }
      props.layer.reflect_to(min,max);
      break;
    case "Normalize":
      props.layer.normalize(NormalizeType.value);
      break;
  }
      console.log(props.layer.array())
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