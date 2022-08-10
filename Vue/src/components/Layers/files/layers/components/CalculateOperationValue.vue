<template>
  <div id="mmContent">
    <LabelSelector
      v-model:value="operation"
      :list="operations"
      :label="`Operation`"
      class="BlockMargin"
    ></LabelSelector>
    <LabelSelector
      v-model:value="NormalizeType"
      :list="NormalizeTypes"
      :label="`Normalize Type`"
      v-show="operation == 'Normalize'"
      class="BlockMargin"
    ></LabelSelector>

    <LabelInput
      v-for="(item, index) in operation_list"
      type="number"
      class="BlockMargin"
      :label="operations_object[operation][`label`][index]"
      v-show="operation != 'Normalize'"
      v-model:value="calculate_value[index]"
      :key="item"
    />

    <div>
      <button
        style="padding: 0.625rem"
        @click="exec_calculate()"
        class="BlockMargin"
      >
        {{ operation }}
      </button>
    </div>
  </div>
</template>

<script setup>
import { computed, inject, ref } from "vue";
import LabelSelector from "./components/LabelSelector.vue";
import LabelInput from "./components/LabelInput.vue";
const props = defineProps(["layer"]);

//PARTS:
let calculate_value = ref([0, 1, 0, 1]);
let operation = ref("Select an operation");
let operations = [
  "Add",
  "Sub",
  "Mul",
  "Div",
  "Normalize",
  "Replace",
  "Abs",
  "Reflect to",
  "Range Reflect to",
];
const operations_object = {
  //details when we select an operation
  Add: { name: "Add", quantity: 1, label: ["Add Value"] },
  Sub: { name: "Sub", quantity: 1, label: ["Sub Value"] },
  Mul: { name: "Mul", quantity: 1, label: ["Mul Value"] },
  Div: { name: "Div", quantity: 1, label: ["Div Value"] },
  Normalize: { name: "Normalize", quantity: 1, label: ["Normalize Type"] },
  "Reflect to": {
    name: "Reflect to",
    quantity: 2,
    label: ["Target min", "Target max"],
  },
  "Range Reflect to": {
    name: "Range Reflect to",
    quantity: 4,
    label: ["Min", "Max", "Target min", "Target max"],
  },
  Replace: {
    name: "Replace",
    quantity: 3,
    label: ["Min", "Max", "Replace with"],
  },
  Abs: { name: "Abs", quantity: 0, label: ["Abs Value"] },
};
let operation_list = computed(() => {
  //make a array for v-for
  if (operations_object[operation.value]) {
    return Array.from(
      { length: operations_object[operation.value]["quantity"] },
      (_, i) => i
    );
  }
});

let NormalizeTypes = ["min_max", "z_score"];
let NormalizeType = ref("max_min");
const update_thumbnails = inject("update_thumbnails");
let exec_calculate = () => {
  // if(!Number(calculate_value.value)){
  //   alert("Please input a number");
  //   return;
  // }
  switch (operation.value) {
    case "Add":
      props.layer.add_value(calculate_value.value[0]);
      break;
    case "Sub":
      props.layer.sub_value(calculate_value.value[0]);
      break;
    case "Mul":
      props.layer.mul_value(calculate_value.value[0]);
      break;
    case "Div":
      props.layer.div_value(calculate_value.value[0]);
      break;
    case "Replace":
      if (calculate_value.value[0] >= calculate_value.value[1]) {
        alert("Min and Max should be in ascending order");
        return;
      }
      props.layer.replace(
        calculate_value.value[0],
        calculate_value.value[1],
        calculate_value.value[2]
      );
      break;
    case "Abs":
      props.layer.abs();
      break;
    case "Reflect to": {
      if (calculate_value.value[0] >= calculate_value.value[1]) {
        alert("Min and Max should be in ascending order");
        return;
      }
      let min = Number(calculate_value.value[0]);
      let max = Number(calculate_value.value[1]);
      if (isNaN(min) || isNaN(max) || max - min <= 0) {
        alert(
          "Please input valid numbers and value1(min) should less than value2(max). Please read Docs to determine how to input numbers."
        );
        return;
      }
      props.layer.reflect_to(min, max);
      break;
    }
    case "Range Reflect to": {
      if (
        calculate_value.value[0] >= calculate_value.value[1] ||
        calculate_value.value[2] >= calculate_value.value[3]
      ) {
        alert("Min and Max should be in ascending order");
        return;
      }
      let min = Number(calculate_value.value[0]);
      let max = Number(calculate_value.value[1]);
      let target_min = Number(calculate_value.value[2]);
      let target_max = Number(calculate_value.value[3]);
      props.layer.range_reflect(min, max, target_min, target_max);
      break;
    }
    case "Normalize":
      props.layer.normalize(NormalizeType.value);
      break;
  }

  if (update_thumbnails.if) {
    update_thumbnails.fn();
  }
};
</script>

<style lang="scss" scoped>
#mmContent {
  display: flex;
  flex-direction: row;
  align-items: center;
}
</style>