<template>
  <div class="header">
    <div class="content">
      <div class="title">
        <a src="/">GeoVector</a>
      </div>
    </div>

    <!-- <div class="nav">
      <ul>
        <li><a to="/paleocal">PaleoCal</a></li>
        <p style="display: inline-block; font-size: 120%; user-select: none">
          /
        </p>
        <li><a to="/dev">Dev</a></li>
        <p style="display: inline-block; font-size: 120%; user-select: none">
          /
        </p>     

        <li><a to="/about">About Me</a></li>
        <p style="display: inline-block; font-size: 120%; user-select: none">
          /
        </p>
   
        <li><a to="/gallery">Photos Gallery</a></li>
      </ul>
    </div> -->
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
  .nav {
    display: flex;
    justify-content: flex-end;
    ul {
      li {
        margin: 0 1.25rem;
        display: inline-block;

        a {
          padding: 0.3125rem;
          transition: color 0.5s, background-color 0.5s, text-shadow 0.5s,
            border 0.5s !important;
          font-size: 200%;
        }
        a:hover {
          color: rgb(1, 155, 87);
          text-shadow: 0rem 0rem 0.3125rem rgba(103, 103, 255, 0.5);
        }
      }
    }
  }
  .nav li:last-child {
    margin-right: 10px;
  }
}
</style>