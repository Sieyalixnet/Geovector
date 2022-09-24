<template>
<div>
    <input :value="props.value" @input="handleInput($event)" v-bind="$attrs">
</div>
</template>

<script setup>
const props = defineProps(["value"])
let handleInput=(e)=>{
emits("update:value",e.target.value)
}
const emits = defineEmits(["update:value"]);
</script>