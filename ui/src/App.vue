<template>
  <div class="ui container">
    <home-header class="component"></home-header>
    <div class="ui two column doubling stackable grid container">
      <div class="column">
        <submit-article class="component"></submit-article>
      </div>
      <div class="column">
        <get-article class="component"></get-article>
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
