<template>
  <div class="layerContent">
    <div class="canvas_block" @click="$emit('renderMain')">
      <canvas height="100" width="100" :id="props.canvansid"></canvas>
    </div>
    <div class="layerTitle">
      <input
        type="text"
        v-model="layer.OptionalAttributes.name"
      />

      <select v-model="select_option">
        <option
          v-for="(item, index) in operation"
          :key="index"
          :value="item.value"
        >
          {{ item.name }}
        </option>
      </select>
    </div>
    <div class="operation_component">
      <Padding v-if="select_option == 'padding'" :layer="props.layer"></Padding>
      <conv2d v-if="select_option == 'conv2d'" :layer="props.layer"></conv2d>
      <transpose v-if="select_option == 'transpose'" :layer="props.layer"></transpose>
      <cal-operation v-if="select_option == 'cal_operation'"  :layer="props.layer"></cal-operation>
      <cal-operation-value v-if="select_option == 'cal_operation_value'"  :layer="props.layer"></cal-operation-value>
    </div>

  </div>
</template>

<script setup>
import { reactive, ref } from "@vue/reactivity";
import { computed, provide } from "@vue/runtime-core";
import Padding from "./components/padding.vue";
import conv2d from "./components/conv2d.vue";
import transpose from "./components/transpose.vue";
import CalOperation from "./components/CalculateOperation.vue"
import CalOperationValue from "./components/CalculateOperationValue.vue"

const props = defineProps(["layer", "canvansid","file"]);
let select_option = ref("select a operation");
const emits = defineEmits(["renderMain"]);

let operation = [
  {
    name: "Padding",
    value: "padding",
  },
  {
    name: "Conv2d",
    value: "conv2d",
  },
  {
    name: "Calculate",
    value: "cal_operation",
  },
  {
    name: "Calculate_value",
    value: "cal_operation_value",
  },
  {
    name:"Transpose",
    value:"transpose",
  }
];
</script>

<style lang="scss" scoped>
.layerContent {
  width: 100%;
  height: 6.875rem;
  border: 1px rgba(0, 90, 30, 0.5) solid;
  display: flex;
  flex-direction: row;
  justify-content: flex-start;
  align-items: center;
    @media screen and (orientation: portrait) {
      overflow: scroll;
    }
  .layerTitle {
    input{width: 5.125rem;}
    padding: 0 0.625rem;
    height: 100%;
    display: flex;
    flex-direction: column;
    align-content: center;
    justify-content: center;
    box-shadow: 0 0 5px rgba(0, 0,0, 0.0);
    border-right: 1px rgba(90, 160, 80, 0.5) solid;
  }
  .canvas_block {
    height: 100%;
    border-right: 1px rgba(90, 160, 80, 0.5) solid;
  }
  .canvas_block:hover {
    cursor: pointer;
  }
  .operation_component {
    height: 100%;
    margin:0 0.625rem;

    display: flex;
    flex-direction: row;
    justify-content: flex-start;
    align-items: center;
  }
}

</style>

<style lang="scss">
.BlockMargin{
  margin:0 10px;
}
</style>