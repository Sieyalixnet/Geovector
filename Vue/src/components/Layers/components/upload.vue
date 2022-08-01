<template>
  <!-- <canvas id="canvas"></canvas> -->
  <button @click="input">upload</button>
</template>

<script setup>
import { inject } from "vue";
let ImageFileList = inject("ImageFileList");
let WASM_Module = inject("WASM_Module");
let input = () => {
  var __temp_InputElement__ = document.createElement("input");
  __temp_InputElement__.addEventListener("change", HandleChangeFile, false);
  __temp_InputElement__.type = "file";
  __temp_InputElement__.accept = "image/*";
  __temp_InputElement__.click();
};

function HandleChangeFile(e) {
  var file = e.target.files[0];
  let name = file.name;
  if (!/image\/\w+/.test(file.type)) {
    alert("Image only.");
    return false;
  }
  var fileReader = new FileReader();
  try {
    fileReader.readAsDataURL(file);
    fileReader.onload = function (e) {
      renderCanvas(this.result, name);
    };
  } catch (_) {
    alert("This format is not supported.");
  }
}
function renderCanvas(ImageSRC, name) {
  let canvas = document.createElement("canvas");
  let ctx = canvas.getContext("2d");
  let img = new Image();
  img.src = ImageSRC;
  img.onload = function () {
    canvas.width = img.width;
    canvas.height = img.height;
    ctx.drawImage(img, 0, 0, img.width, img.height);
    let result = WASM_Module.WASM.createImageVector_CTX(
      ctx,
      img.width,
      img.height
    );
    result.set_name(name + Date.now());
    ImageFileList.List.push(result);
  };
}
</script>

<style lang="scss" scoped>
button {
  width: 100%;
  height: 30px;
  border: none;
  background: rgba(127, 202, 136, 0.3);
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
}
button:hover {
  cursor: pointer;
}
</style>