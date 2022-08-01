<template>
  <div>
    <input type="number" v-model="padding_value" />
    <input type="number" v-model="times" />
    <button @click="exec_padding()">Padding!</button>
  </div>
</template>

<script setup>
import { ref } from "vue";
const props = defineProps(["layer"]);
let padding_value = ref(0);
let times = ref(1);
let exec_padding = () => {
  if (
    !isNaN(Number(padding_value.value)) &&
    !isNaN(Number(times.value)) &&
    times.value > 0
  ) {
    props.layer.padding_times(padding_value.value, times.value);
  }
};
</script>

<style lang="scss" scoped>
</style>