<template>
  <div id="LayerOperationContent">
    <LabelSelector
      v-model:value="operation"
      :list="operations"
      :label="`Operation`"
      class="BlockMargin"
    ></LabelSelector>
        <LabelFileSelector
        v-show="operation=='Move To'"
      v-model:value="SelectedFile"
      :list="List"
      :label="operation"
      class="BlockMargin"
    ></LabelFileSelector>
    <div>
      <button
        style="padding: 0.625rem"
        class="BlockMargin"
        @click="exec_operation()"
      >
        {{operation}}
      </button>
    </div>
  </div>
</template>

<script setup>
import LabelSelector from "./components/LabelSelector.vue";
import LabelFileSelector from "./components/LabelFileSelector.vue";
import { ref,inject} from "vue";
let ImageFileList = inject("ImageFileList");
const file = inject("file")
const update_thumbnails=inject("update_thumbnails");
const { List } = ImageFileList;
let SelectedFile = ref("");
let operation = ref("Select an operation");
let operations = ["Copy", "Upward", "Downward", "Delete","Move To","Download Image","Download JSON Data"];
const props = defineProps(["layer","LayerIndex"]);

let exec_operation = async () => {
  //exec_operation for LayerOperation
  let op = operation.value
  let index = props.LayerIndex
  switch (op) {
    case "Copy":
      await file.copy(index);
      break;
    case "Upward":
      file.upward(index);
      break;
    case "Downward":
      file.downward(index);
      break;
    case "Delete":
      file.delete(index);
      break;
    case "Move To":
      //console.log(SelectedFile);
      //console.log(List.find(item => item.name === SelectedFile))
      // console.log(List)
      // console.log(SelectedFile.value)
      let targetList = List.find((item) => item.name === SelectedFile.value)
      targetList.add_List(
       file.__copy__(index)
      );
      file.delete(index);
      break;
    case "Download Image":
      props.layer.render_to_downlaod();
      break
    case "Download JSON Data":
      props.layer.json_to_download(file.name);
      break
  }
    if(update_thumbnails.if){
      update_thumbnails.fn()
    } 
};


// let exec_operation = () => {
//   if (operations.includes(operation.value)) {
    
//     emits("LayerOperation_exec", {operation:operation.value,SelectedFile:SelectedFile.value});
//   }
// };
</script>

<style lang="scss" scoped>
#LayerOperationContent{
    display:flex;
    flex-direction:row;
    align-items:center;
    justify-content:flex-start;
}
</style>