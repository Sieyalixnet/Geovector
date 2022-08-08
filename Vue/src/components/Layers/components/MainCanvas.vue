<template>
  <div style="width: 100%; position: relative">
    <div id="canvas_attr" @mousemove="attrshowController($event)" v-show="lastRenderedLayer.layer">
      v: {{ Math.round(MouseCanvasPosition["value"] * 100) / 100 }}, x:
      {{ MouseCanvasPosition["x"] }}, y: {{ MouseCanvasPosition["y"] }}
    </div>
    <div
      :style="
        ImageFileList.List.length > 0 ? 'overflow: scroll' : 'overflow:none'
      "
      id="main_canvas_div"
    >
      <canvas id="main_canvas" @mousemove="mouseMove($event)"> </canvas>
    </div>

    <div id="transform_div">
      <button @click="zoom_add(0.1)">+</button
      ><button @click="zoom_add(-0.1)">-</button
      ><button @click="zoom_reset()">
        Zoom: {{ Math.round(main_canvas_size.size * 10) / 10 }}
      </button>
      <button style="cursor:default;background-color: #eee;" disabled >Edit Mode</button>
    </div>
  </div>
</template>


<script setup>
import { inject, reactive, ref } from "vue";
let ImageFileList = inject("ImageFileList");
let lastRenderedLayer = inject("lastRenderedLayer");
let main_canvas_size = reactive({
  size: 1,
});
let MouseCanvasPosition = inject("MouseCanvasPosition");
let attrshowTimer;
let attrshowController = (e) => {
  clearTimeout(attrshowTimer);
  e.target.style = "transform:translateX(-100%);";
  attrshowTimer = setTimeout(() => {
    e.target.style = "transform:none;";
  }, 1000);
};
let mouseMove = (e) => {
  MouseCanvasPosition.x = e.offsetX;
  MouseCanvasPosition.y = e.offsetY;
  if (lastRenderedLayer.layer) {
    MouseCanvasPosition.value = lastRenderedLayer.layer.get(
      e.offsetY,
      e.offsetX
    );
  }
};

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
</script>

<style lang="scss" scoped>
#main_canvas_div {
  max-width: 100%;
  max-height: 64rem;
  width: 90vw;
  height: 45vw;
  border: 1px #000 solid;
  z-index: -1;
  display: flex;
  justify-content: flex-start;
  align-items: flex-start;
}
#canvas_attr {
  position: absolute;
  right: 25px;
  bottom: 40px;
  padding: 0 5px;
  color: #fff;
  background-color: rgba(0, 0, 0, 0.7);
  user-select: none;
  box-shadow: 0 0 1px #000;
  z-index: 10;
  // font-size: 0.5rem;
  // font-weight: bold;
}
#transform_div {
  width: 100%;

  display: flex;
  flex-direction: row;
  justify-content: flex-start;
  align-items: center;
  * {
    flex: 1;
    border-radius: 0px;
    border-top: 0;
    border-right: 0;
    label {
      overflow: hidden;
    }
  }
  button:first-child {
    border-radius: 0 0 0 0.25rem;
  }
  button:last-child {
    border-radius: 0 0 0.25rem 0;
    border-right: 1px solid rgba(0, 80, 30, 0.7);
  }
}
</style>