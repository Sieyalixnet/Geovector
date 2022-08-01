<template>
  <div class="layerContent">
    <canvas height="100" width="100" :id="props.canvansid"></canvas>
  </div>
</template>

<script setup>
import { reactive } from "@vue/reactivity";
import { provide } from "@vue/runtime-core";
const props = defineProps(["layer", "canvansid"]);
</script>

<style lang="scss" scoped>
.layerContent {
    width: 100%;
    border: 1px rgba(0,90,30,0.5) solid;
    display: flex;
    flex-direction: row;
    justify-content: flex-start;
    align-items: center;
  canvas {
    border-right: 1px rgba(90,160,80,0.5) solid;
  }
}
</style>