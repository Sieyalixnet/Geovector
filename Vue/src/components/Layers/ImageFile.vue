<template>
  <div id="ImageFileContent">
    <Upload></Upload>
    <div :style="ImageFileList.List.length>0?'overflow: scroll':'overflow:none'" id="main_canvas_div" style=" max-width: 100%;max-height:64rem;width:90vw; height:45vw;border:1px #000 solid;" >
    <canvas id="main_canvas"></canvas>
    </div>
    
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

<style lang="scss">
#ImageFileContent{
  select{
    background: none;
  border: 1px solid rgba(0,80,30,0.7);
  }
  *{box-sizing: border-box;}
  button {
  border: 1px solid rgba(0, 80, 30, 0.7);
  background: rgba(127, 202, 136, 0.3);
}
button:hover {
  background: rgba(127, 202, 136, 0.5);
}
input{
  border: 1px solid rgba(0, 80, 30, 0.7);
  outline: none;
  background: rgba(192, 252, 212, 0.3);
}
input:focus{
  border: 1px solid rgba(103, 176, 130, 0.7);
  animation: blinkBorder ease-in 2s infinite reverse;
}
@keyframes blinkBorder {
  0% {
    box-shadow: 0 0 5px rgba(0, 0, 0, 0);
  }
  50% {
    box-shadow: 0 0 5px rgba(1, 155, 87, 0.8);
  }
  100% {
    box-shadow: 0 0 5px rgba(0, 0, 0, 0);
  }
}}
</style>