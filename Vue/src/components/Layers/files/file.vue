<template>
  <div id="FileContent">
    <div class="filename" @click="showLayers = !showLayers">
      <label>FileName</label>
      <input type="text" v-model="props.file.name" v-on:click.stop />
    </div>
    <Transition name="slide-fade">
      <div class="layers" v-show="showLayers">
        <div
          class="layer"
          v-for="(item, index) in props.file.ChannelList"
          :key="index"
        >
          <Layer
            @renderMain="render_main"
            @LayerOperation_exec="exec_operation"
            :layer="item"
            :canvansid="`${props.file.name}_${index}`"
            :LayerIndex="index"
          ></Layer>
        </div>
      </div>
    </Transition>
    <button @click="render_thumbnails_to_canvas()">Update Thumbnails</button>
  </div>
</template>

<script setup>
import { reactive, ref } from "@vue/reactivity";
import { onMounted, provide,inject } from "@vue/runtime-core";
import Layer from "./layers/Layer.vue";
const props = defineProps(["file"]);
let ImageFileList = inject("ImageFileList");
const { List } = ImageFileList;
let showLayers = ref(true);

let exec_operation = (option, index) => {
  const {operation,SelectedFile} = option;
  console.log(operation, SelectedFile,index);
  switch (operation) {
    case "Copy":
      props.file.copy(index);
      break;
    case "Upward":
      props.file.upward(index);
      break;
    case "Downward":
      props.file.downward(index);
      break;
    case "Delete":
      props.file.delete(index);
      break;
    case "Move To":
      console.log(SelectedFile);
      console.log(List.find(item => item.name === SelectedFile))
      List.find(item => item.name === SelectedFile).add_List(props.file.get_Vector(index));
      props.file.delete(index);
      break;
  }
};

let render_thumbnails_to_canvas = () => {
  props.file.render_thumbnails_to_canvas();
};
onMounted(() => {
  render_thumbnails_to_canvas();
});
let render_main = (index) => {
  props.file.render_to_main_canvas(index);
};
</script>

<style lang="scss" scoped>
#FileContent {
  min-width: 100%;
  border: 1px solid rgba(0, 90, 40, 0.5);
  border-radius: 0.3125rem;

  h2 {
    text-align: center;
  }
  .filename {
    margin-top: 10px;
    text-align: center;
    cursor: pointer;
    label {
      margin-right: 10px;
      font-size: larger;
      cursor: pointer;
    }
    input {
      height: 30px;
      font-size: medium;
      cursor: text;
    }
  }
  .layers {
    margin: 10px;
    display: flex;
    flex-direction: column;
    justify-content: flex-start;
    align-items: flex-start;
    overflow: hidden;

    .layer {
      width: 100%;
      margin-top: 0.3125rem;
    }
    .layer:first-child {
      margin-top: 0;
    }
  }
  button {
    margin-right: 20px;
  }
}
</style>