<template>
  <article class="ui basic segment">
    <header>
      <h1 class="ui center aligned header">{{ article.title }}</h1>
      <p class="ui segment">{{ article.abstract }}</p>
    </header>
    <div class="ui basic segment">
      <p style="white-space: pre-wrap">{{ article.body }}</p>
    </div>
  </article>
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
