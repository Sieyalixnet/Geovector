<template>
  <div class="conv2dContent">
      <LabelInput
        type="number"
        class="BlockMargin"
        label="Kernel Size"
        v-model:value="kernel.kernel_size"
        @change="change_list()"
      />
    <LabelInput type="number" v-model:value="stride" label="Stride" class="BlockMargin"/>


    <div class="kernel_block BlockMargin">
      <label>Kernel</label>
      <div :style="`width:${kernel.kernel_size * 23}px;`" class="kernel">
        <span v-for="(_, index) in kernel.kernel_size_list" :key="index">
          <input
            type="text"
            v-for="(_, inner_index) in kernel.kernel_size_list"
            :key="index"
            v-model="kernel.kernel[index * kernel.kernel_size + inner_index]"
          />
        </span>
       
      </div>
    </div>

    <div class="BlockMargin" v-if="kernel.kernel_size==3">
      <LabelSelector :label="`Kernel Mode 3x3`" :list="Object.keys(conv2DMode)" v-model:value="conv2DMode_Selected" @change="modeChanged()"></LabelSelector>

    </div>
    <div>
      <button style="padding:0.625rem;" class="BlockMargin" @click="exec_conv2d()">conv2d!</button>
    </div>
    <!-- {{ kernel }} -->
  </div>
</template>

<script setup>
import { onMounted, reactive, ref,inject, watch } from "vue";
import LabelInput from "./components/LabelInput.vue";
import LabelSelector from "./components/LabelSelector.vue";
const props = defineProps(["layer"]);
const update_thumbnails=inject("update_thumbnails");
let kernel = reactive({
  kernel_size: 3,
  kernel_size_list: [],
  kernel: [],
});
let stride = ref(1);
let conv2DMode ={
    Normal: [
      0, 0, 0,
      0, 1, 0,
      0, 0, 0
    ],
    GaussianBlur: [
      0.045, 0.122, 0.045,
      0.122, 0.332, 0.122,
      0.045, 0.122, 0.045
    ],
    GaussianBlur2: [
      1, 2, 1,
      2, 4, 2,
      1, 2, 1
    ],
    GaussianBlur3: [
      0, 1, 0,
      1, 1, 1,
      0, 1, 0
    ],
    Unsharpen: [
      -1, -1, -1,
      -1,  9, -1,
      -1, -1, -1
    ],
    Sharpness: [
       0,-1, 0,
      -1, 5,-1,
       0,-1, 0
    ],
    Sharpen: [
       -1, -1, -1,
       -1, 16, -1,
       -1, -1, -1
    ],
    EdgeDetect: [
       -0.125, -0.125, -0.125,
       -0.125,  1,     -0.125,
       -0.125, -0.125, -0.125
    ],
    EdgeDetect2: [
       -1, -1, -1,
       -1,  8, -1,
       -1, -1, -1
    ],
    EdgeDetect3: [
       -5, 0, 0,
        0, 0, 0,
        0, 0, 5
    ],
    EdgeDetect4: [
       -1, -1, -1,
        0,  0,  0,
        1,  1,  1
    ],
    EdgeDetect5: [
       -1, -1, -1,
        2,  2,  2,
       -1, -1, -1
    ],
   EdgeDetect6: [
       -5, -5, -5,
       -5, 39, -5,
       -5, -5, -5
    ],
    SobelHorizontal: [
        1,  2,  1,
        0,  0,  0,
       -1, -2, -1
    ],
    SobelVertical: [
        1,  0, -1,
        2,  0, -2,
        1,  0, -1
    ],
    PrevitHorizontal: [
        1,  1,  1,
        0,  0,  0,
       -1, -1, -1
    ],
    PrevitVertical: [
        1,  0, -1,
        1,  0, -1,
        1,  0, -1
    ],
    BoxBlur: [
        0.111, 0.111, 0.111,
        0.111, 0.111, 0.111,
        0.111, 0.111, 0.111
    ],
    TriangleBlur: [
        0.0625, 0.125, 0.0625,
        0.125,  0.25,  0.125,
        0.0625, 0.125, 0.0625
    ],
    Emboss: [
       -2, -1,  0,
       -1,  1,  1,
        0,  1,  2
    ]
  }
  let conv2DMode_Selected = ref("");
let modeChanged = ()=>{
    if(kernel.kernel_size==3 && conv2DMode_Selected.value!=""){
      kernel.kernel=conv2DMode[conv2DMode_Selected.value]
    }
  }

let exec_conv2d = () => {

  if (stride.value > 0) {
    props.layer.conv2d(kernel.kernel, stride.value);
        if(update_thumbnails.if){
      update_thumbnails.fn()
    }
  } else{
    alert("Stride should > 0.")
  }
};

let change_list = () => {
  kernel.kernel_size_list = [];
  for (let i = 0; i < kernel.kernel_size; i++) {
    kernel.kernel_size_list.push(i);
  }

  //if user input all of them, it should be a default value.
  let kernel_value = 0;

  let kernel_set = Array.from(new Set(kernel.kernel));
  if (kernel_set.length == 1) {
    kernel_value = kernel_set[0];
  }
  kernel.kernel = [];
  for (let i = 0; i < kernel.kernel_size * kernel.kernel_size; i++) {
    kernel.kernel.push(kernel_value);
  }
};

onMounted(() => {
  change_list();
});
</script>

<style lang="scss" scoped>
.conv2dContent {
  display: flex;
  flex-direction: row;
  justify-content: flex-start;
  align-items: center;


  .kernel_block {
    max-width: 100px;
    max-height: 100px;
    overflow: scroll;
    .kernel {
      text-align: center;
      border: 1px solid rgba(0, 90, 30, 0.5);
      input {
        width: 20px;
        margin: 0.0625rem;
        border: none !important;
        outline: none;
        background: none;
        background-color: rgba(170, 255, 180, 0.2);
      }
      input:focus {
        border: none;
      }
    }
  }
}
</style>