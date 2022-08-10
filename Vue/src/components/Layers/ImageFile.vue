<template>
  <div id="ImageFileContent">
    <Upload></Upload>
    <Introduction v-if="ImageFileList.List.length==0"></Introduction>
    <MainCanvas v-if="ImageFileList.List.length>0"></MainCanvas>
    <div v-if="ImageFileList.List.length>0" id="FileAndFileOperation">
      <div id="ImageFileContentFileOperation">
        <file-operation></file-operation>
      </div>
      <div id="ImageFileContentFile">
        <File
          v-for="(item, index) in ImageFileList.List"
          :key="item.name"
          :file="item"
        ></File>
      </div>
    </div>
  </div>
</template>

<script setup>
import { reactive } from "@vue/reactivity";
import { computed, provide, watch } from "@vue/runtime-core";
import Introduction from "./components/Introduction.vue";
import MainCanvas from "./components/MainCanvas.vue";
import Upload from "./components/Upload.vue";
import File from "./files/File.vue";
import FileOperation from "./files/FileOperation.vue";
let ImageFileList = reactive({ List: [], ALL_Channel_List: [] });
// let main_canvas_div_height = computed(()=>{
//   let canvas_div = document.getElementById("main_canvas_div");
//   let width = canvas_div.style.width
//   return 0.5*width;
// })

let MouseCanvasPosition = reactive({
  x: 0,
  y: 0,
  value: 0,
});

let lastRenderedLayer = reactive({
  file: undefined,
  layer: undefined,
  index: undefined,
});

provide("MouseCanvasPosition", MouseCanvasPosition);
provide("lastRenderedLayer", lastRenderedLayer);
watch(
  () => ImageFileList.List,
  (newValue) => {
    ImageFileList.ALL_Channel_List = [];
    for (let file of ImageFileList.List) {
      //reactive's List
      ImageFileList.ALL_Channel_List.push(...file.ChannelList); //every ImageVector's ChannelList
    }
  },
  { deep: true }
);

provide("ImageFileList", ImageFileList);
</script>

<style lang="scss" scoped>
#ImageFileContent {
  max-width: 80%;
  margin: 0 auto;
  display: flex;
  flex-direction: column;
  justify-content: center;
  align-items: center;
  #FileAndFileOperation {
    width: 100%;

    #ImageFileContentFile,
    #ImageFileContentFileOperation {
      width: 100%;
      margin-top: 0.625rem;
    }
    #ImageFileContentFileOperation {
      flex: 1;
    }
    #ImageFileContentFile {
      flex: 3;
    }
  }
}
</style>

<style lang="scss">
#ImageFileContent {
  * {
    box-sizing: border-box;
  }
  select {
    background: none;
    border: 1px solid rgba(0, 80, 30, 0.7);
  }
  select:focus {
    border: 1px solid rgba(103, 176, 130, 0.7);
    animation: blinkBorder ease-in 5s infinite reverse;
  }

  button {
    border: 1px solid rgba(0, 80, 30, 0.7);
    border-radius: 0.25em;
    background: rgba(127, 202, 136, 0.3);
    text-shadow: 0 0 3px #000, 0 0 1px #000, 0 0 2px #000, 0 0 4px #444;
    color: #fff;
  }
  button:hover {
    background: rgba(127, 202, 136, 0.5);
    cursor: pointer;
  }
  input {
    border: 0.0625rem solid rgba(0, 80, 30, 0.7);
    outline: none;
    background: rgba(220, 255, 220, 0.3);
  }
  input:focus {
    background: rgba(220, 255, 220, 0.1);
    border: 1px solid rgba(103, 176, 130, 0.7);
    animation: blinkBorder ease-in 3s infinite reverse;
  }
  @keyframes blinkBorder {
    0% {
      box-shadow: 0 0 10px rgba(0, 0, 0, 0);
    }
    50% {
      box-shadow: 0 0 10px rgba(1, 155, 87, 0.8);
    }
    100% {
      box-shadow: 0 0 10px rgba(0, 0, 0, 0);
    }
  }
}

.slide-fade-enter-active {
  transition: all 0.2s ease-out;
}

.slide-fade-leave-active {
  transition: all 0.2s cubic-bezier(0.5, 0.5, 0.8, 1);
}

.slide-fade-enter-from,
.slide-fade-leave-to {
  transform-origin: top;
  transform: translateX(20px) scale(0);
  opacity: 0;
}

.fade-enter-active {
  transition: all 0.1s ease-out;
}

.fade-enter-from,
.fade-leave-to {
  opacity: 0;
}
</style>