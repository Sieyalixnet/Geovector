<template>
  <div>
    <div class="card" :style="styleConcat">
      <h1>{{ props.title }}</h1>
      <p>{{ props.text }}</p>
      <div style="margin: 0 20px"><slot></slot></div>
    </div>
  </div>
</template>
<script setup>
import {computed,defineProps} from "vue"
let props = defineProps([
  "WidthHeight",
  "backgroundColor",
  "textColor",
  "title",
  "text",
  "justify",
]);
//这里的wh需要传入单位，这里只是把它们进行拼接

function randomColor() {
  let r = Math.floor(Math.random() * (256 - 180) + 180);
  let g = Math.floor(Math.random() * (256 - 180) + 180);
  let b = Math.floor(Math.random() * (256 - 180) + 180);
  return "rgb(" + r + "," + g + "," + b + ")";
}
let lockColor = false;
let lockedColor = ""; //被锁定的随机颜色

let styleConcat = computed(() => {
  let style = "";
  if (props.WidthHeight) {
    style += "width:" + props.WidthHeight[0] + ";";
    style += "height:" + props.WidthHeight[1] + ";";
  }
  if (props.backgroundColor) {
    style += "background-color:" + props.backgroundColor + ";";
  } else {
    if (!lockColor) {
      lockedColor = randomColor();
      style += "background-color:" + lockedColor + ";";
      lockColor = true;
    } else {
      style += "background-color:" + lockedColor + ";";
    }
  }
  if (props.textColor) {
    style += "color:" + props.textColor + ";";
  } else {
    style += "color:#fff;";
  }
  if (props.justify) {
    style += "justify-content:" + props.justify + ";";
    style += "align-items:" + props.justify + ";";
  } else {
    style += "justify-content:flex-start;";
    style += "align-items:flex-start;";
  }
  return style;
});
</script>
<style lang="scss" scoped>
.card {
  width: 100%;
  min-height: 12.5rem;
  display: flex;
  flex-direction: column;
  justify-content: flex-start;
  align-items: flex-start;
  padding:0.625rem;
  h1,
  p {
    text-shadow: 0 0 5px #000, 0 0 3px #000, 0 0 1px #000;

    display: block;
    margin: 20px 20px 0 20px;
  }
  p {
    font-size: larger;
  }
}
</style>