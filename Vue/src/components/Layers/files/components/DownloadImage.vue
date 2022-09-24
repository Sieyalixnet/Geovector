<template>
          <button
        style="padding: 0.625rem"
        @click="exec_calculate()"
        class="BlockMargin"
      >
        Download
      </button>
</template>

<script setup>

import {inject} from "vue";
let WASM_Module=inject("WASM_Module");
const {WASM} = WASM_Module
    let exec_calculate=()=>{
        WASM.download_main_canvas()
    }
</script>

<style lang="scss" scoped>
#mergeRGBContent {
  display: flex;
  flex-direction: row;
  align-items: center;
}
</style>