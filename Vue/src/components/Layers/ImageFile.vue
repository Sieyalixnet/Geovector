<template>
  <div id="ImageFileContent">
    <div v-show="ImageFileList.List.length>0" id="main_canvas_div" style="overflow: scroll; max-width: 100%;max-height:64rem;width:90vw; height:45vw;" >
    <canvas id="main_canvas"></canvas>
    </div>
    <Upload></Upload>
    <div id="ImageFileContentFile" v-for="(item, index) in ImageFileList.List" :key="index">
      <File :file="item"></File>
    </div>
  </div>
</template>

<script setup>
import { reactive } from "@vue/reactivity";
import { computed, provide } from "@vue/runtime-core";
import Upload from "./components/upload.vue";
import File from "./files/file.vue"
let ImageFileList = reactive({ List: [] });
// let main_canvas_div_height = computed(()=>{
//   let canvas_div = document.getElementById("main_canvas_div");
//   let width = canvas_div.style.width
//   return 0.5*width;
// })
provide("ImageFileList", ImageFileList);
</script>

<style lang="scss" scoped>
#ImageFileContent{
  max-width: 80%;
  margin:0 auto;
  display:flex;
  flex-direction:column;
  justify-content:center;
  align-items:center;
  #ImageFileContentFile{
    width:100%;

  }
}
</style>