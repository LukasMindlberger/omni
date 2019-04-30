<template>
  <div class="ui relaxed list">
    <div class="item">
      <router-link class="content" :to="{ name: 'community', params: {} }"
        >Astrophysics</router-link
      >
    </div>
    <div class="item">
      <router-link class="content" :to="{ name: 'community', params: {} }"
        >Biomechanics</router-link
      >
    </div>
  </div>
</template>

<script>
export default {
  mounted() {
    this.$store.commit("updateTitle", "Omni | Communities");
    this.$store.commit("updateNavText", "Communities");
  },
  beforeDestroy() {
    this.$store.commit("updateTitle", "Omni");
    this.$store.commit("updateNavText", "Navigation");
  },
  data() {
    return {
      communities: []
    };
  }
};
</script>

<style scoped></style>
