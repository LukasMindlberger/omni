<template>
  <div id="app">
    <div class="ui grid text container focus">
      <div class="column">
        <div class="grid">
          <div class="row">
            <div class="ui basic segment">
              <omni-header></omni-header>
            </div>
          </div>
        </div>
        <transition name="slide-fade" mode="out-in">
          <router-view />
        </transition>
      </div>
    </div>
  </div>
</template>

<script>
import OmniHeader from "@/components/OmniHeader.vue";
export default {
  components: {
    OmniHeader
  },
  name: "App"
};
</script>

<style scoped>
.page-move {
  transition: transform 1s;
}
</style>
<style scoped src="@/assets/css/global-animations.css"></style>
<style scoped src="@/assets/css/master.css"></style>
