<template>
  <div id="paddingContent">
        <div class="label_input_block block">
      <label>Padding Value</label>
      <input
        type="number"
        v-model="padding_value"
      />
    </div>
        <div class="label_input_block block">
      <label>Padding Times</label>
      <input
        type="number"
        v-model="times"
      />
    </div>
    <div>
    <button @click="exec_padding()">Padding!</button>
    </div>
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
#paddingContent{
  display: flex;
  flex-direction: row;
  align-items: center;
  .block{
    margin-left: 20px;

  }
  .label_input_block {
    text-align: center;
    label {
      display: block;
    }
    input {
      width: 100px;
    }
  }

}
</style>