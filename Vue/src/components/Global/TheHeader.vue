<template>
  <div class="header">
    <div class="content">
      <div class="title">
        <a src="/">Geovector</a>
      </div>
    </div>


  </div>
</template>

<style lang="scss" scoped>
.header {
  box-shadow: 0 0 5px rgba(0, 0, 0, 0.8);
  background-color: #fff;
  margin-bottom: 20px;
  .content {
    .title {
      margin: 0 0 10px 60px;
      a {
        font-size: 300%;
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