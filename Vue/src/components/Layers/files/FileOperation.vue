<template>
  <div id="FileOperationContent">
    <div class="filename" @click="showLayers=!showLayers">
      <label >File Operation</label>
    </div>
    <div style="width:100%;height:500px;" v-if="showLayers">aaa</div>
  </div>
</template>

<script setup>
import { reactive,ref } from "@vue/reactivity";
import { onMounted, provide } from "@vue/runtime-core";

let showLayers = ref(false);
</script>

<style lang="scss" scoped>
#FileOperationContent {
  width: 100%;
  border: 1px solid rgba(0, 90, 40, 0.5);
  border-radius: 0.3125rem;
  transition: height 10s;
  h2 {
    text-align: center;
  }
  .filename {
    margin-top:10px;
    text-align: center;
    cursor: pointer;
    label{margin-right:10px;font-size: larger;cursor: pointer;}
    input{height:30px;font-size: medium;cursor: text;}
  }
  .layers {
    margin: 10px;
    display: flex;
    flex-direction: column;
    justify-content: flex-start;
    align-items: flex-start;
  overflow: hidden;
    
    .layer {
      width: 100%;
      margin-top: 0.3125rem;
    }
    .layer:first-child {
      margin-top: 0;
    }
  }
  button {
    margin-right: 20px;
  }
}
</style>