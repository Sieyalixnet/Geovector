<template>
  <div id="mergeRGBContent">
    <LabelLayerSelector
      v-model:value="selectedLayersName[0]"
      :list="ALL_Channel_List"
      :label="`Red Channel`"
      class="BlockMargin"
    ></LabelLayerSelector>
    <LabelLayerSelector
      v-model:value="selectedLayersName[1]"
      :list="ALL_Channel_List"
      :label="`Green Channel`"
      class="BlockMargin"
    ></LabelLayerSelector>
    <LabelLayerSelector
      v-model:value="selectedLayersName[2]"
      :list="ALL_Channel_List"
      :label="`Blue Channel`"
      class="BlockMargin"
    ></LabelLayerSelector>
    <LabelLayerSelector
      v-model:value="selectedLayersName[3]"
      :list="ALL_Channel_List"
      :label="`Alpha Channel`"
      class="BlockMargin"
    ></LabelLayerSelector>
    <div>
      <button
        style="padding: 0.625rem"
        @click="exec_calculate()"
        class="BlockMargin"
      >
        Merge
      </button>
    </div>
  </div>
</template>

<script setup>
import { inject, ref } from "vue";
import LabelSelector from "../layers/components/components/LabelSelector.vue";
import LabelLayerSelector from "../layers/components/components/LabelLayerSelector.vue";
let ImageFileList = inject("ImageFileList");
let lastRenderedLayer = inject("lastRenderedLayer");
const { ALL_Channel_List } = ImageFileList;
//let selectedMatrixName = ref("");
let selectedLayersName = ref([null, null, null, null]);
let WASM_Module = inject("WASM_Module");
const { WASM } = WASM_Module;
let exec_calculate = () => {
  // console.log(WASM);
  let R = ALL_Channel_List.find(
    (item) => item.OptionalAttributes.name === selectedLayersName.value[0]
  );
  let G = ALL_Channel_List.find(
    (item) => item.OptionalAttributes.name === selectedLayersName.value[1]
  );
  let B = ALL_Channel_List.find(
    (item) => item.OptionalAttributes.name === selectedLayersName.value[2]
  );
  let A = ALL_Channel_List.find(
    (item) => item.OptionalAttributes.name === selectedLayersName.value[3]
  );
  if (R && G && B) {
    try{
    WASM.renderRGBA("main_canvas", { R, G, B, A });}
    catch(_){
      alert("These layers's length are not equal.");
    }
    lastRenderedLayer.file = undefined;
    lastRenderedLayer.index = undefined;
    if (R && G && B && A) {
      lastRenderedLayer.layer = [R, G, B, A];
    } else if (R && G && B) {
      lastRenderedLayer.layer = [R, G, B];
    } 
  } else {alert("Please select R,G,B channels at least.");}
};
</script>

<style lang="scss" scoped>
#mergeRGBContent {
  display: flex;
  flex-direction: row;
  align-items: center;
}
</style>