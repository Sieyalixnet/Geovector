<template>
  <div style="padding:0.3125rem;border: 0.0625rem solid rgba(0,255,0,0.1);">
    <p>{{ props.label }}</p>
    <input :value="props.value" @input="handleInput($event)" v-bind="$attrs" />
  </div>
</template>

<script setup>
const props = defineProps(["value", "label"]);
let handleInput = (e) => {
  emits("update:value", e.target.value);
};
const emits = defineEmits(["update:value"]);
</script>

<style lang="scss" scoped>
p {
  text-align: center;
}
input {
  width: 100px;
}
</style>