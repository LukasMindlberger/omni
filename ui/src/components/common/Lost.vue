<template>
  <div>
    <h1 class="ui center aligned header">
      Looks like you're lost. There's nothing here!
    </h1>
    <div class="ui centered grid">
      <div class="row">
        <router-link class="" :to="{ name: 'Home', params: {} }"
          >Return to Home</router-link
        >
      </div>
    </div>
  </div>
</template>

<script>
export default {
  mounted() {
    this.$store.commit("updateTitle", "Omni | Page Not Found");
  },
  beforeDestroy() {
    this.$store.commit("updateTitle", "Omni");
  }
};
</script>

<style scoped></style>
