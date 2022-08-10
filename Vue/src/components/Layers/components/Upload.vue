<template>
  <button @click="input">Browse Your Image!</button>
</template>

<script setup>
import { inject } from "vue";
let ImageFileList = inject("ImageFileList");
const { List } = ImageFileList;
let WASM_Module = inject("WASM_Module");
let input = () => {
  var __temp_InputElement__ = document.createElement("input");
  __temp_InputElement__.addEventListener("change", HandleChangeFile, false);
  __temp_InputElement__.type = "file";
  (__temp_InputElement__.accept = "image/png,image/jpeg,application/json"),
    __temp_InputElement__.click();
};

function HandleChangeFile(e) {
  let file = e.target.files[0];
  let name = file.name;
  let fileReader = new FileReader();
  let permittedImageFormat = ["image/png", "image/jpeg"];
  try {
    if (permittedImageFormat.includes(String(file.type))) {
      fileReader.readAsDataURL(file);
      fileReader.onload = function (e) {
        renderCanvas(this.result, name);
      };
    } else if (String(file.type) == "application/json") {
      fileReader.readAsText(file);
      fileReader.onload = function (e) {
        parseJSON(this.result);
      };
    }
  } catch (e) {
    alert("This format is not supported.");
  }
}
function parseJSON(json) {
  let JSONFilesObject = JSON.parse(json);
  for (let item of JSONFilesObject.files) {
    let sameFile = List.find((x) => x.name == item.filename);
    if (!sameFile) {
      let result = WASM_Module.WASM.createImageVector_JSON(item);
      List.push(result);
    } else {
      sameFile.add_List_JSON(item);
    }
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
    List.push(result);
    if (canvas) {
      console.log("removed");
      canvas.remove();
    }
  };
}
</script>

<style lang="scss" scoped>
button {
  border-radius: 0px !important;
  width: 100%;
  height: 1.875rem;
  margin: 0 0 10px 0;
}
</style>