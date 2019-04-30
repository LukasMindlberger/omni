<template>
  <div class="ui center aligned container">
    <div class="row">
      <div class="ui basic segment">
        <!-- Navigation at root address -->
        <div class="ui horizontal divider" v-if="this.$route.name === 'Home'">
          Menu
        </div>
        <nav
          class="ui compact secondary stackable menu"
          v-if="this.$route.name === 'Home'"
        >
          <router-link
            class="item"
            v-if="showMoon"
            exact
            :to="{ name: 'Home', params: {} }"
          >
            <img src="@/assets/logo.png" alt="" />
          </router-link>
          <router-link class="item" exact :to="{ name: 'Submit', params: {} }"
            >Submit</router-link
          >
          <router-link
            class="item"
            exact
            :to="{ name: 'Repository', params: {} }"
            >Repository</router-link
          >
          <router-link
            class="item"
            exact
            :to="{ name: 'Communities', params: {} }"
            >Communities</router-link
          >
          <router-link
            class="item"
            exact
            :to="{ name: 'Astrophysics', params: {} }"
            >Example Community</router-link
          >
        </nav>

        <!-- Typical navigation -->
        <nav
          id="navbar"
          class="ui fluid borderless compact menu"
          v-if="this.$route.name !== 'Home'"
        >
          <router-link
            class="item"
            v-if="showMoon"
            exact
            :to="{ name: 'Home', params: {} }"
          >
            <img src="@/assets/logo.png" alt="" />
          </router-link>
          <div class="right menu">
            <div class="ui simple dropdown item">
              {{ navText }}
              <i class="dropdown icon"></i>
              <div class="menu">
                <router-link
                  class="item"
                  :class="{ active: this.$route.name === 'Submit' }"
                  exact
                  :to="{ name: 'Submit', params: {} }"
                  >Submit</router-link
                >
                <router-link
                  class="item"
                  :class="{ active: this.$route.name === 'Repository' }"
                  exact
                  :to="{ name: 'Repository', params: {} }"
                  >Repository</router-link
                >
                <router-link
                  class="item"
                  :class="{ active: this.$route.name === 'Communities' }"
                  exact
                  :to="{ name: 'Communities', params: {} }"
                  >Communities</router-link
                >
              </div>
            </div>
          </div>
        </nav>
      </div>
    </div>
  </div>
</template>

<script>
import { mapGetters } from "vuex";
export default {
  computed: {
    showMoon: function() {
      if (this.$route.name === "Home") {
        return false;
      } else {
        return true;
      }
    },
    ...mapGetters({
      navText: "navText"
    })
  }
};
</script>

<style scoped>
#navbar {
  border: none;
  box-shadow: none;
}
</style>
