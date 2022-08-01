<template>
  <div id="fileContent">
    <div>
    <label>FileName</label>
    <input type="text" v-model="props.file.name"/>
    </div>
    <div class="layers">
      
      <div
        class="layer"
        v-for="(item, index) in props.file.ChannelList"
        :key="index"
      >
        <Layer @click="trig(index)"  :layer="item" :canvansid="`${props.file.name}_${index}`"></Layer>
      </div>
    </div>
    <button @click="render_thumbnails_to_canvas()">render</button>
  </div>
</template>

<script setup>
import { reactive } from "@vue/reactivity";
import { provide } from "@vue/runtime-core";
import Layer from "./layers/layer.vue";
const props = defineProps(["file"]);

let render_thumbnails_to_canvas = () => {
  props.file.render_thumbnails_to_canvas();
};

let trig=(index)=>{props.file.render_to_main_canvas(index)}
</script>

<style lang="scss" scoped>
#fileContent {
  min-width: 100%;
      border: 1px solid rgba(0,90,40,0.5);
    border-radius: 1.25rem;
    h2{text-align: center;}
  .layers {
    margin:1.25rem;
    display: flex;
    flex-direction: column;
    justify-content: flex-start;
    align-items: flex-start;
    
    .layer{
      width: 100%;
      margin-top: 0.3125rem;
    }
    .layer:first-child{
      margin-top: 0;
    }
  }
  button{ margin-right: 20px;}
}
</style>