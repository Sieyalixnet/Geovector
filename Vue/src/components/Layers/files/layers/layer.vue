<template>
  <div
    class="layerContent"
    :class="`${layer.OptionalAttributes.name[0]}_layer`"
  ><Transition name="fade">
    <div v-show="showDetail == false" style="padding:0.1875rem; width: 100%;">
      <div class="OmitInput">
        <button @click="showDetail = !showDetail">+</button>
        <input type="text" v-model="layer.OptionalAttributes.name" />
      </div>
   
   
   </div>
   </Transition>
   <Transition name="fade">
    <div v-show="showDetail == true" class="layerContentDetail">
      <div class="canvas_block" @click="$emit('renderMain', props.LayerIndex)">
        <canvas height="100" width="100" :id="props.canvansid"></canvas>
      </div>

      <div class="layerTitle">
        <div class="OmitInput">
          <button @click="showDetail = !showDetail">-</button>
          <input type="text" v-model="layer.OptionalAttributes.name" />
        </div>

        <select v-model="select_option">
          <option
            v-for="(item, index) in operation"
            :key="index"
            :value="item.component"
          >
            {{ item.name }}
          </option>
        </select>
      </div>
      
      <div class="operation_component">
        <Transition name="fade">
        <component :is="select_option" :layer="props.layer" @LayerOperation_exec="LayerOperation_exec"></component>
        </Transition>
        <!-- <Padding
          v-if="select_option == 'padding'"
          :layer="props.layer"
        ></Padding>
        <conv2d v-if="select_option == 'conv2d'" :layer="props.layer"></conv2d>
        <cal-operation
          v-if="select_option == 'cal_operation'"
          :layer="props.layer"
        ></cal-operation>
        <cal-operation-value
          v-if="select_option == 'cal_operation_value'"
          :layer="props.layer"
        ></cal-operation-value>
        <layer-operation
          v-if="select_option == 'layer_operation'"
          :layer="props.layer"
          @LayerOperation_exec="LayerOperation_exec"
        ></layer-operation>
         -->
      </div>
    </div>
    </Transition>
  </div>
</template>

<script setup>
import { reactive, ref,shallowRef } from "@vue/reactivity";
import { computed, provide } from "@vue/runtime-core";
import Padding from "./components/padding.vue";
import conv2d from "./components/conv2d.vue";
import CalOperation from "./components/CalculateOperation.vue";
import CalOperationValue from "./components/CalculateOperationValue.vue";
import LayerOperation from "./components/LayerOperation.vue";
const emits = defineEmits(["renderMain", "LayerOperation_exec"]);
const props = defineProps(["layer", "canvansid", "file", "LayerIndex"]);

//omit controller
let showDetail = ref(true);

//emit the layer Operation
let LayerOperation_exec = (option) => {
  select_option.value = Padding;
  emits("LayerOperation_exec", option, props.LayerIndex);
};

//manage the selectbox and loaded Components
let select_option = shallowRef(Padding);
let operation = [
  {
    name: "Padding",
    component: Padding,
  },
  {
    name: "Conv2d",
    component: conv2d,
  },
  {
    name: "Calculate",
    component: CalOperation,
  },
  {
    name: "Calculate(Value)",
    component: CalOperationValue,
  },
  {
    name: "Layer Operation",
    component: LayerOperation,
  },
];
</script>

<style lang="scss" scoped>
.layerContent {
  width: 100%;
  border: 1px rgba(0, 90, 30, 0.5) solid;
  @media screen and (orientation: portrait) {
    overflow: scroll;
  }
  .OmitInput {
    input {
      width: 6.5625rem;
    }
    button {
      border-radius: 0 !important;
      border-right: 0px !important;
    }
  }
  .layerContentDetail {
    display: flex;
    flex-direction: row;
    justify-content: flex-start;
    align-items: center;

    .layerTitle {
      padding: 0 0.625rem;
      height: 110px;
      display: flex;
      flex-direction: column;
      align-items: center;
      justify-content: center;
      box-shadow: 0 0 0.3125rem rgba(0, 0, 0, 0);
      border-right: 1px rgba(90, 160, 80, 0.5) solid;
      * {
        margin: 0.1875rem 0;
        max-width: 7.5rem;
      }
    }

    .canvas_block {
      height: 110px;
      border-right: 1px rgba(90, 160, 80, 0.5) solid;
    }
    .canvas_block:hover {
      cursor: pointer;
    }
    .operation_component {
      height: 100%;
      margin: 0 0.625rem;

      display: flex;
      flex-direction: row;
      justify-content: flex-start;
      align-items: center;
    }
  }
}
.R_layer {
  background-color: rgba(255, 200, 200, 0.2);
}
.G_layer {
  background-color: rgba(191, 255, 170, 0.2);
}
.B_layer {
  background-color: rgba(162, 161, 255, 0.2);
}
.A_layer {
  background-color: rgba(207, 248, 255, 0.2);
}
</style>

<style lang="scss">
.BlockMargin {
  margin: 0 10px;
}
</style>