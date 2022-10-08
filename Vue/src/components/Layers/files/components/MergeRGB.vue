<template>
  <div id="mergeRGBContent">
    <div v-if="List.length==1 && (List[0].ChannelList.length==3 ||List[0].ChannelList.length==4)">
      <div @click="shortcut_merge = !shortcut_merge">
            <input type="checkbox" :checked="shortcut_merge" /><label
              >Shorcut Merge</label
            >
          </div>
    </div>
    <div class="mergeRGBBlocks" v-if="shortcut_merge==false">
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
    </div>
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
import { inject, nextTick, ref, watch } from "vue";
import LabelSelector from "../layers/components/components/LabelSelector.vue";
import LabelLayerSelector from "../layers/components/components/LabelLayerSelector.vue";
let ImageFileList = inject("ImageFileList");
let lastRenderedLayer = inject("lastRenderedLayer");
const { List,ALL_Channel_List } = ImageFileList;
//let selectedMatrixName = ref("");
let selectedLayersName = ref([null, null, null, null]);
let WASM_Module = inject("WASM_Module");
const { WASM } = WASM_Module;
let exec_calculate = () => {
  // console.log(WASM);
  let R,G,B,A;

  if(shortcut_merge.value==true){
    R = List[0].ChannelList[0]
    G = List[0].ChannelList[1]
    B = List[0].ChannelList[2]
    if(List[0].ChannelList[3]){
      A = List[0].ChannelList[3]
    }
  }else{
  R = ALL_Channel_List.find(
    (item) => item.OptionalAttributes.name === selectedLayersName.value[0]
  );
  G = ALL_Channel_List.find(
    (item) => item.OptionalAttributes.name === selectedLayersName.value[1]
  );
  B = ALL_Channel_List.find(
    (item) => item.OptionalAttributes.name === selectedLayersName.value[2]
  );
  A = ALL_Channel_List.find(
    (item) => item.OptionalAttributes.name === selectedLayersName.value[3]
  );}
  console.log(R,G,B,A)
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

let shortcut_merge = ref(true)
watch(
  () =>List,
  () => {
    if(List.length>1){
      shortcut_merge.value=false
    } else{shortcut_merge.value=true}
  },
  { deep: true }
)
</script>

<style lang="scss" scoped>
#mergeRGBContent {
  display: flex;
  flex-direction: row;
  align-items: center;
  .mergeRGBBlocks{
  display: flex;
  flex-direction: row;
  align-items: center;}
}
</style>