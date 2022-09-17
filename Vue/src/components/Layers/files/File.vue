<template>
  <div id="FileContent">
    <div class="filename" @click="showLayers = !showLayers">
      <label>File Name</label>
      <input type="text" v-model="props.file.name" v-on:click.stop />
    </div>
    <Transition name="slide-fade">
      <div class="layers" v-show="showLayers">
        <div
          class="layer"
          v-for="(item, index) in props.file.ChannelList"
          :key="item.OptionalAttributes.name"
        >
          <Layer
            @renderMain="render_main"
            :layer="item"
            :canvansid="`${props.file.name}_${index}`"
            :LayerIndex="index"
          ></Layer>
        </div>
        <div class="convenient">
          <div @click="render_reflect_click()">
            <input type="checkbox" :checked="render_reflect" /><label
              >Auto Reflect</label
            >
          </div>
          <div @click="auto_render_thumbails = !auto_render_thumbails">
            <input type="checkbox" :checked="auto_render_thumbails" /><label
              >Auto Update Thumbails</label
            >
          </div>
          <div>
            <button
              v-if="!auto_render_thumbails"
              @click="render_thumbnails_to_canvas()"
            >
              Update Thumbnails
            </button>
          </div>
        </div>
      </div>
    </Transition>
  </div>
</template>

<script setup>
import {
  onMounted,
  provide,
  inject,
  reactive,
  ref,
  watch,
  nextTick,
} from "vue";
import Layer from "./layers/Layer.vue";
const props = defineProps(["file"]);
let ImageFileList = inject("ImageFileList");
let lastRenderedLayer = inject("lastRenderedLayer");
const { List } = ImageFileList;
let showLayers = ref(true);
let auto_render_thumbails = ref(true);
let render_reflect = ref(true);

provide("file", props.file);

let render_thumbnails_to_canvas = () => {
  props.file.render_thumbnails_to_canvas(render_reflect.value);
};
onMounted(() => {
  render_thumbnails_to_canvas();
});
provide("update_thumbnails", {
  fn: render_thumbnails_to_canvas,
  if: auto_render_thumbails.value,
});

let render_reflect_click = () => {
  //update the thumbnails when the reflect checkbox is clicked
  render_reflect.value = !render_reflect.value;
  render_thumbnails_to_canvas();
};

let render_main = (index) => {
  props.file.render_to_main_canvas(index, render_reflect.value);
  lastRenderedLayer.file = props.file;
  lastRenderedLayer.layer = props.file.get_Vector(index);
  lastRenderedLayer.index = index;
};

watch(
  () => List,
  (_) => {
    nextTick(() => {
      if (auto_render_thumbails.value) {
        render_thumbnails_to_canvas();
      }
    });
  },
  { deep: true }
);
</script>

<style lang="scss" scoped>
#FileContent {
  min-width: 100%;
  border: 1px solid rgba(0, 90, 40, 0.5);
  border-radius: 0.3125rem;
  min-height: 3.125rem;
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
    .convenient {
      display: flex;
      justify-content: flex-start;
      align-items: center;
      min-height: 1.875rem;
      div {
        margin: 0 0.625rem;

        label {
          margin-left: 0.125rem;
          user-select: none;
        }
      }
    }
  }
  button {
    margin-right: 20px;
  }
}
</style>