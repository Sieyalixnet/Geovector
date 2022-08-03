<template>
  <div id="transposeContent">
    <div>
    <button style="padding:0.625rem" class="BlockMargin" @click="exec_transpose()">Transpose!</button>
    </div>
  </div>
</template>

<script setup>
const props = defineProps(["layer"]);
let exec_transpose = () => {
    props.layer.transpose();
};
</script>

