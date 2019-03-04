<template>
  <div class="ui segment">
    <div class="ui center aligned header">{{ article.title }}</div>
    <div class="ui segment">{{ article.abstract }}</div>
    <div class="ui basic segment">
      <div style="white-space: pre-wrap">{{ article.body }}</div>
    </div>
  </div>
</template>

<script>
export default {
  props: {
    article: {
      title: String,
      abstract: String,
      body: String
    }
  }
};
</script>

<style scoped></style>
