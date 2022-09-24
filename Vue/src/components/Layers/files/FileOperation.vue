<template>
  <div id="FileOperationContent">
    <div class="filename" @click="showFileOperation = !showFileOperation">
      <label>File Operation</label>
    </div>
    <Transition name="slide-fade">
      <div style="width: 100%" v-if="showFileOperation">
        <select v-model="select_option" class="FileOperationSelector">
          <option
            v-for="(item, index) in operation"
            :key="item.name"
            :value="item.component"
          >
            {{ item.name }}
          </option>
        </select>
        <Transition name="fade">
          <div class="FileOperationOption">
            <component :is="select_option"></component>
          </div>
        </Transition>
      </div>
    </Transition>
  </div>
</template>

<script setup>
import { reactive, ref, shallowRef } from "@vue/reactivity";
import { onMounted, provide } from "@vue/runtime-core";
import LabelSelector from "./layers/components/components/LabelSelector.vue";
import MergeRGB from "./components/MergeRGB.vue";
import DownloadData from "./components/DownloadData.vue";
import DownloadImage from "./components/DownloadImage.vue";
import CreateFile from "./components/CreateFile.vue";
import CreateLayer from "./components/CreateLayer.vue";
import RemoveFile from "./components/RemoveFile.vue";
let showFileOperation = ref(false);
let select_option = shallowRef(MergeRGB);
let operation = [
  {
    name: "Merge to RGBA Image",
    component: MergeRGB,
  },

  {
    name: "Download Main Image",
    component: DownloadImage,
  },
  {
    name: "Download JSON data",
    component: DownloadData,
  },
  {
    name: "Create a New Empty File",
    component: CreateFile,
  },
  {
    name: "Create a New Layer",
    component: CreateLayer,
  },
  {
    name: "Remove File",
    component: RemoveFile,
  },
];
</script>

<style lang="scss" scoped>
#FileOperationContent {
  width: 100%;
  border: 1px solid rgba(0, 90, 40, 0.5);
  border-radius: 0.3125rem;
  transition: height 10s;
  .FileOperationSelector {
    margin-left: 5px;

    border: 1px solid rgba(0, 90, 40, 0.5);
  }
  .FileOperationOption {
    border: 1px solid rgba(0, 90, 40, 0.5);
    display: flex;
    flex-direction: row;
    justify-content: center;
    align-items: center;
    padding: 5px;
    margin: 5px;
    @media screen and (orientation: portrait) {
      justify-content: flex-start;
      overflow: scroll;
    }
  }
  .filename {
    margin-top: 10px;
    text-align: center;
    cursor: pointer;
    min-height: 30px;
    label {
      font-size: larger;
      cursor: pointer;
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
.BlockMargin {
  margin: 0 10px;
}
</style>