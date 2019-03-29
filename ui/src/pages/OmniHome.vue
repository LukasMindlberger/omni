<template>
  <div class="ui center aligned container">
    <div class="ui horizontal divider">Menu</div>
    <div class="ui basic segment">
      <router-link
        class="ui primary button"
        exact
        :to="{ name: 'Submit Article', params: {} }"
        >Submit an article</router-link
      >
      <router-link
        class="ui button"
        exact
        :to="{ name: 'Repository', params: {} }"
        >Search articles</router-link
      >
      <router-link
        class="ui secondary button"
        :to="{ name: 'Profiles', params: {} }"
        >User profiles</router-link
      >
    </div>
  </div>
</template>

<script>
import OmniHeader from "@/components/OmniHeader";
export default {
  components: {
    OmniHeader
  },
  methods: {}
};
</script>

<style scoped></style>
