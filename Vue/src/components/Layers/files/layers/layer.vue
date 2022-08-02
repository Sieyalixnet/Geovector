<template>
  <div class="layerContent">
    <div class="canvas_block" @click="$emit('renderMain')">
      <canvas height="100" width="100" :id="props.canvansid"></canvas>
    </div>
    <div class="layerTitle">
      <input
        type="text"
        style="width: 5.125rem"
        v-model="layer.OptionalAttributes.name"
      />

      <select v-model="select_option">
        <option value="padding">padding</option>
        <option value="conv2d">conv2d</option>
        <option value="matrix_multiple">matrix_multiple</option>
      </select>
    </div>
    <div class="operation_component">
      <Padding v-if="select_option == 'padding'" :layer="props.layer"></Padding>
      <conv2d v-if="select_option == 'conv2d'" :layer="props.layer"></conv2d>
    </div>
  </div>
</template>

<script setup>
import { reactive, ref } from "@vue/reactivity";
import { computed, provide } from "@vue/runtime-core";
import Padding from "./components/padding.vue";
import conv2d from "./components/conv2d.vue";

const props = defineProps(["layer", "canvansid"]);
let select_option = ref("select a operation");
const emits = defineEmits(["renderMain"]);
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
  .layerTitle{
    margin:0 0.625rem;
    display: flex;
    flex-direction: column;
    align-content: center;
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
    margin-left: 30px;
    @media screen and (orientation: portrait) {
      overflow: scroll;
    }
    display: flex;
    flex-direction: row;
    justify-content: flex-start;
    align-items: center;
  }
}
</style>