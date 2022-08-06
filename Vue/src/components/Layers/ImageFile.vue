<template>
  <div id="ImageFileContent">
    {{ ImageFileList.List }}
    <Upload></Upload>
    <div style="width: 100%">
      <div
        :style="
          ImageFileList.List.length > 0 ? 'overflow: scroll' : 'overflow:none'
        "
        id="main_canvas_div"
      >
        <canvas id="main_canvas"></canvas>
      </div>

      <div id="transform_div">
        <button @click="zoom_add(0.1)">+</button
        ><button @click="zoom_add(-0.1)">-</button
        ><button @click="zoom_reset()">
          Zoom: {{ Math.round(main_canvas_size.size * 10) / 10 }}
        </button>
      </div>
    </div>
    <div id="ImageFileContentFileOperation"><file-operation></file-operation></div>
    <div
      id="ImageFileContentFile"
      v-for="(item, index) in ImageFileList.List"
      :key="index"
    >
      <File :file="item"></File>
    </div>
  </div>
</template>

<script setup>
import { reactive } from "@vue/reactivity";
import { computed, provide, watch } from "@vue/runtime-core";
import Upload from "./components/Upload.vue";
import File from "./files/File.vue";
import FileOperation from "./files/FileOperation.vue";
let ImageFileList = reactive({ List: [], ALL_Channel_List: [] });
// let main_canvas_div_height = computed(()=>{
//   let canvas_div = document.getElementById("main_canvas_div");
//   let width = canvas_div.style.width
//   return 0.5*width;
// })
let main_canvas_size = reactive({
  size: 1,
});

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
let zoom_reset = () => {
  let canvas = document.getElementById("main_canvas");
  if (canvas) {
    main_canvas_size.size = 1;
    canvas.style = `transform: scale(${main_canvas_size.size},${main_canvas_size.size}) translate(0%,0%);`;
  } else {
    return;
  }
};
let zoom_add = (delta) => {
  let canvas = document.getElementById("main_canvas");
  if (canvas) {
    main_canvas_size.size += delta;

    canvas.style = `transform-origin:top left;transform: scale(${main_canvas_size.size},${main_canvas_size.size})`;
  } else {
    return;
  }
};
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
  #ImageFileContentFile,#ImageFileContentFileOperation {
    width: 100%;
  }
  #main_canvas_div {
    max-width: 100%;
    max-height: 64rem;
    width: 90vw;
    height: 45vw;
    border: 1px #000 solid;

    display: flex;
    justify-content: flex-start;
    align-items: flex-start;
  }
  #transform_div {
    width: 100%;

    display: flex;
    flex-direction: row;
    justify-content: flex-start;
    align-items: center;
    button {
      flex: 1;
      border-radius: 0px;
      border-top: 0;
    }
    button:nth-child(2) {
      border-left: 0;
      border-right: 0;
    }
    margin-bottom: 1.25rem;
  }
}
</style>

<style lang="scss">
#ImageFileContent {
  select {
    background: none;
    border: 1px solid rgba(0, 80, 30, 0.7);
  }
  * {
    box-sizing: border-box;
  }
  button {
    border: 1px solid rgba(0, 80, 30, 0.7);
    border-radius: 0.25em;
    background: rgba(127, 202, 136, 0.3);
    text-shadow: 0 0 3px #000, 0 0 1px #000;
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
    animation: blinkBorder ease-in 5s infinite reverse;
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
  }
}

.slide-fade-enter-active {
  transition: all 0.3s ease-out;
}

.slide-fade-leave-active {
  transition: all 0.4s cubic-bezier(0.5, 0.5, 0.8, 1);
}

.slide-fade-enter-from,
.slide-fade-leave-to {
  transform: translateX(20px);
  opacity: 0;
}

.fade-enter-active {
  transition: all 0.3s ease-out;
}

.fade-enter-from,
.fade-leave-to {
  opacity: 0;
}
</style>