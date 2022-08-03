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
    <div>
      <button style="padding:0.625rem;" class="BlockMargin" @click="exec_conv2d()">conv2d!</button>
    </div>
    <!-- {{ kernel }} -->
  </div>
</template>

<script setup>
import { onMounted, reactive, ref } from "vue";
import LabelInput from "./components/LabelInput.vue";
const props = defineProps(["layer"]);

let kernel = reactive({
  kernel_size: 3,
  kernel_size_list: [],
  kernel: [],
});
let stride = ref(1);

let exec_conv2d = () => {
  let kernel_final = kernel.kernel.map((x) => Number(x));
  if (stride.value > 0) {
    props.layer.conv2d(kernel_final, stride.value);
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
  console.log(kernel_set);
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