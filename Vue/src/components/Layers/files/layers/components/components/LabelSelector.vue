<template>
  <div style="padding:0.3125rem;border: 0.0625rem solid rgba(0,255,0,0.1);background-color: rgba(255,255,255,0.7);">
    <p>{{ props.label }}</p>
    <select :value="props.value" @change="handleChange($event)" v-bind="$attrs">
        <option v-for="(item,index) in list" :key="index" :value="item">{{item}}</option>

    </select>
  </div>
</template>

<script setup>
const props = defineProps(["value","list", "label"]);
let handleChange = (e) => {
  emits("update:value", e.target.value);
};
const emits = defineEmits(["update:value"]);
</script>

<style lang="scss" scoped>
p {
  text-align: center;
}
select {
  min-width: 100px;
  max-width: 150px;
}
</style>