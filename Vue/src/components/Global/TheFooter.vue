<template>
        <div class="bottom">
            <div class="content">
                <div><h2 id="copyright">© 2021-2024 Sieyalix.Net 版权所有</h2></div>
                <div><h2 id="name">网站名:XY的编程</h2></div>
                <div><h2 id="law_label">ICP证:</h2><a href="https://beian.miit.gov.cn/" id="law">粤ICP备2021160450号</a></div>
                <!-- <div><h2 id="email">sieyalixnet@163.com</h2></div> -->
            </div>
        </div>
</template>

<style lang="scss" scoped >

  .bottom {
    margin-top: 10px;
    background-image: linear-gradient(
      rgba(141, 246, 183, 0.4),
      rgba(225, 254, 245, 0.8)
    );
    display: flex;
    justify-content: center;
    align-items: center;
    .content {
      width: 70%;
        display: flex;
        flex-direction: row;
        justify-content: space-around;
        align-items: center;
      div{

      h2,
      ul{
        display: inline-block;
        margin-right: 10px;
        font-size: 14px;
        line-height: 14px;
      }
      #law_label{
        margin-right: 0px;
      }

      }
      div {
        border: 1px solid rgba(0, 0, 0, 0);
        transition: color 0.5s, background-color 0.5s, text-shadow 0.5s,
          border 0.5s;
      }
      div:hover {
        //border: 1px solid rgba(0, 0, 0, 0.2);
        //background-color: rgba(255, 255, 255, 0.4);
        color: rgb(1, 155, 87);
        text-shadow: 0px 0px 5px rgba(103, 255, 103, 0.5);
      }    
    }
  }
</style>