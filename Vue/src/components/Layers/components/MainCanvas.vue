<template>
  <div style="width: 100%; position: relative" >
    <div
      id="canvas_attr"
      @mousemove="attrshowController($event)"
      v-show="lastRenderedLayer.layer"
    >
      {{ mouseValueShow }}
    </div>

    <div
      :style="
        ImageFileList.List.length > 0 ? 'overflow: scroll' : 'overflow:none'
      "
      id="main_canvas_div"
    >
      <div
        id="changeValueInput"
        v-show="ClickToSetValueShow"
        style="position: absolute; z-index: 15"
        v-on:click.stop
      >
        <input
          type="number"
          v-model="ClickToSetValue.value"
          style="margin-right: 3px"
          v-on:click.stop
          :placeholder="`X: ${ClickToSetValue.x},Y: ${ClickToSetValue.y }`"
        /><button @click="ClickToSetValue_Set()">√</button>
        <button @click="ClickToSetValue.show = false">X</button>
      </div>
      <canvas
        id="main_canvas"
        @mousemove="mouseMove($event)"
        @click="mouseClick($event)"
      >
      </canvas>
    </div>

    <div id="transform_div">
      <button @click="zoom_add(0.1)">+</button
      ><button @click="zoom_add(-0.1)">-</button
      ><button @click="zoom_reset()">
        Zoom: {{ Math.round(main_canvas_size.size * 10) / 10 }}
      </button>
      <button style="cursor: default; background-color: #eee" disabled>
      Edit Mode
        <!-- <a href="#main_canvas">Edit Mode</a> -->
      </button>
    </div>
  </div>
</template>


<script setup>
import { computed, inject, reactive, ref } from "vue";
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
  }, 2000);
};
let mouseValueShow = computed(() => {
  if (Array.isArray(MouseCanvasPosition.value)) {
    let v = MouseCanvasPosition.value.map((x) => Math.round(x * 100) / 100);
    if (v.length == 3) {
      return `R:${v[0]}, G:${v[1]}, B:${v[2]}, X:${MouseCanvasPosition["x"]}, Y:${MouseCanvasPosition["y"]}`;
    } else if (v.length == 4) {
      return `R:${v[0]}, G:${v[1]}, B:${v[2]}, A:${v[3]}, X:${MouseCanvasPosition["x"]}, Y:${MouseCanvasPosition["y"]}`;
    }
  } else {
    let v = MouseCanvasPosition.value;
    return `V:${Math.round(v * 100) / 100}, X:${MouseCanvasPosition["x"]}, Y:${
      MouseCanvasPosition["y"]
    }`;
  }
});

let ClickToSetValue = reactive({
  value: 0,
  x: 0,
  y: 0,
  show: false,
});
let ClickToSetValueShow = computed(() => {//click only trigger, and we should check another condition to show the DIV
  return ClickToSetValue.show && !Array.isArray(lastRenderedLayer.layer);
});

let ClickToSetValue_Set =()=>{

  lastRenderedLayer.layer.set(ClickToSetValue.y,  ClickToSetValue.x,ClickToSetValue.value);
  ClickToSetValue.show=false;
}

let mouseClick = (e) => {
  let cs = document.getElementById("cs");
  // console.log(window.getComputedStyle(cs).width,window.getComputedStyle(cs).height);
  if (lastRenderedLayer.layer && !Array.isArray(lastRenderedLayer.layer)) {
    let input = document.getElementById("changeValueInput");
    // input.style = `position:absolute;z-index:1;left:${
    //   e.layerX *main_canvas_size.size
    // }px;top:${e.layerY *main_canvas_size.size}px;`;
    ClickToSetValue.x = e.offsetX;
    ClickToSetValue.y = e.offsetY;
    ClickToSetValue.show = true;
    ClickToSetValue.value = lastRenderedLayer.layer.get(ClickToSetValue.y,  ClickToSetValue.x);
  }
};

let mouseMove = (e) => {
  MouseCanvasPosition.x = e.offsetX;
  MouseCanvasPosition.y = e.offsetY;
  if (lastRenderedLayer.layer) {
    if (Array.isArray(lastRenderedLayer.layer)) {
      MouseCanvasPosition.value = lastRenderedLayer.layer.map((x) =>
        x.get(e.offsetY, e.offsetX)
      );
    } else {
      MouseCanvasPosition.value = lastRenderedLayer.layer.get(
        e.offsetY,
        e.offsetX
      );
    }
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
#changeValueInput{
  position: absolute;
  left: 0.125rem;
  bottom: 2.5rem;
  background-color:rgba(230,255,230,0.6) ;
  padding: 1px;
  border: 1px #fff;
@media screen and (orientation: portrait) {bottom: 1.5625rem;}
}
#canvas_attr {
  position: absolute;
  right: 1.5625rem;
  bottom: 2.5rem;
  padding: 0 5px;
  color: #fff;
  background-color: rgba(0, 0, 0, 0.7);
  user-select: none;
  box-shadow: 0 0 1px #000;
  z-index: 10;
  transition: 0.1s ease-in-out transform;
  @media screen and (orientation: portrait) {bottom: 1.5625rem; right:0.125rem;}
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