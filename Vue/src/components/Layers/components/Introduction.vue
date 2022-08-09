<template>
  <div id="IntroductionContent">
    <ShowCard
      v-for="(item, index) in List"
      :key="index"
      :title="item.title"
      :text="item.text"
      :justify="`center`"
      ><a v-if="item.title == `Guide`" href="www.sieyalix.net"
        >www.sieyalix.net</a
      ></ShowCard
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
    text: "It can be used for matrix calculation(image processing) and image display. ALL PROCESSING WILL BE DONE ON YOU OWN DEVICE!",
  },
  { title: "Guide", text: "Please read the guide on the website" },
  {
    title: "Firstly...",
    text: "Browse your Image(PNG/JPEG) or JSON(You should arrange them in a proper way please follow the guide).",
  },
];
</script>

<style lang="scss" scoped>
#IntroductionContent {
  width: 100%;
  a,
  a:link,
  a:visited,
  a:hover,
  a:active {
    text-decoration: none;
    color: rgb(1, 155, 87);
  }
  a:hover {
    text-shadow: 0 0 2px rgb(200, 255, 187), 0 0 1px rgb(100, 155, 80);
  }
}
</style>