<template>
  <div id="paddingContent">
        <LabelInput class="BlockMargin" label="Padding Value" v-model:value="padding_value" type="number"/>
        <LabelInput class="BlockMargin" label="Padding Times" v-model:value="times" type="number"/>
    <div>
    <button style="padding:0.625rem;" @click="exec_padding()" class="BlockMargin">Padding!</button>
    </div>
  </div>
</template>

<script setup>
import { inject, ref } from "vue";
import LabelInput from "./components/LabelInput.vue";
const props = defineProps(["layer"]);
const update_thumbnails=inject("update_thumbnails");
let padding_value = ref(0);
let times = ref(1);
let exec_padding = () => {
  if (
    !isNaN(Number(padding_value.value)) &&
    !isNaN(Number(times.value)) &&
    times.value > 0
  ) {
    props.layer.padding_times(padding_value.value, times.value);
    if(update_thumbnails.if){
      update_thumbnails.fn()
    }
  }
};
</script>

<style lang="scss" scoped>
#paddingContent {
  display: flex;
  flex-direction: row;
  align-items: center;

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