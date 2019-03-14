<template>
  <div class="ui center aligned container">
    <div class="ui horizontal divider">Menu</div>
    <div class="ui basic segment">
      <router-link
        class="ui button"
        exact
        :to="{ name: 'SubmitArticle', params: {} }"
      >
        Submit an article
      </router-link>
      <router-link
        class="ui button"
        exact
        :to="{ name: 'GetArticle', params: {} }"
      >
        Search articles
      </router-link>
    </div>
  </div>
</template>

<script>
import OmniHeader from "@/components/OmniHeader";
export default {
  components: {
    OmniHeader
  }
};
</script>

<style scoped></style>
