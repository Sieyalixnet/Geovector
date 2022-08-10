<template>
  <div id="IntroductionContent">
    <ShowCard
      v-for="(item, index) in List"
      class="card"
      :key="item.title"
      :title="item.title"
      :text="item.text"
      :justify="`center`"
      :background-color="`rgba(89,${Math.random()*(255-170)+170},122,0.7)`"
      :style="`border: 2px solid rgba(89,${Math.random()*(255-170)+170},122,0.7);`"
      ><div><p><a v-if="item.title == `Guide`" href="www.sieyalix.net"
        >中文</a
      ></p><p style=" text-decoration:line-through;"><a v-if="item.title == `Guide`" 
        >English(provide after official edition publish)</a
      ></p></div></ShowCard
    >
  </div>
</template>
<script setup>
import ShowCard from "./component/ShowCard.vue";
let List = [
  {
    title: "Fast, Light",
    text: "With WASM basing on Rust, this application is extremly fast at computating 2-dimension data. It is of ~1MB(<2MB) after packing up.",
  },
  {
    title: "Strong",
    text: "It can be used for matrix calculation(image processing) and image display. ALL PROCESSING WILL BE DONE ON YOUR DEVICE!",
  },
  { title: "Guide", text: "Please read the guide on the website" },
  {
    title: "Firstly...",
    text: "Browse your Image(PNG/JPEG) or JSON(Please read the guide).",
  },
];
</script>

<style lang="scss" scoped>
#IntroductionContent {
  width: 100%;
  p{margin-top:0.625rem;text-align: center;}
  a,
  a:link,
  a:visited,
  a:hover,
  a:active {
    text-decoration: none;
    color: rgb(1, 100, 20);
    
  }
  a:hover {
    text-shadow: 0 0 2px rgb(200, 255, 187), 0 0 1px rgb(100, 155, 80);
  }
  .card{
    width: 100%;
    margin:0.625rem 0;
  }
}
</style>