<template>
  <div class="header">
    <div class="content">
      <div class="title">
        <a src="/">Geovector</a>
      </div>
      <button @click="toggleDark()">{{isDark?"Dark":"Light"}}</button>
    </div>


  </div>
</template>
<script setup>
import { useDark, useToggle } from '@vueuse/core'

const isDark = useDark({
  selector: 'body',
  attribute: 'class',
  valueDark: 'dark',
  valueLight: 'light',
})
const toggleDark = useToggle(isDark)
</script>
<style lang="scss" scoped>
.header {
  box-shadow: 0 0 5px rgba(0, 0, 0, 0.8);
  background-color: #fff;
  margin-bottom: 20px;
  position: relative;
  .content {
    button{position: absolute;
    right:30px;
    bottom: 20px;
  }
    .title {
      margin: 0 0 10px 60px;
      a {
        font-size: 300%;
        width: max-content;
        display: block;
        cursor: pointer;
        color: rgb(0, 106, 42);
      }
      @media screen and (orientation: portrait) {
        margin: 0 0 0 0;
        text-align: center;
      }
    }
  }
}
</style>