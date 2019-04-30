<template>
  <div id="app">
    <div class="ui grid text container" :class="{ focus: true }">
      <!-- TODO: dynamic focus class global variable -->
      <div class="column">
        <div class="grid">
          <omni-header v-if="this.$route.name === 'Home'"></omni-header>
        </div>
        <omni-nav-bar></omni-nav-bar>
        <div class="ui basic segment">
          <router-view v-title="title" />
        </div>
      </div>
    </div>
  </div>
</template>

<script>
import OmniHeader from "@/components/common/OmniHeader.vue";
import OmniNavBar from "@/components/common/OmniNavBar";
import { mapGetters } from "vuex";

export default {
  components: {
    OmniHeader,
    OmniNavBar
  },
  computed: {
    // title: function() {
    //   if (this.$route.path === "/") {
    //     return "Omni";
    //   }
    //   return "Omni | " + this.$route.name;
    // }
    ...mapGetters({
      title: "pageTitle"
    })
  },
  name: "App"
};
</script>

<style scoped></style>
<style scoped src="@/assets/css/global-animations.css"></style>
<style scoped src="@/assets/css/master.css"></style>
