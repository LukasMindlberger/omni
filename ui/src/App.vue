<template>
  <div>
    <home-header class="component"></home-header>
    <div class="ui container">
      <div class="ui horizontal segments">
        <div class="ui segment">
          <submit-article class="component"></submit-article>
        </div>
        <div class="ui segment">
          <get-article class="component"></get-article>
        </div>
      </div>
    </div>
  </div>
</template>

<script>
import HomeHeader from "./components/HomeHeader.vue";
import SubmitArticle from "./components/SubmitArticle.vue";
import GetArticle from "./components/GetArticle.vue";

export default {
  components: {
    HomeHeader,
    SubmitArticle,
    GetArticle
  }
};
</script>

<style scoped>
.component {
  padding: 1em;
}
</style>
