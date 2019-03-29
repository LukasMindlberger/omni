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
          <router-view v-title="title" />
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
  computed: {
    title: function() {
      if (this.$route.name === "Home") {
        return "Omni";
      }
      return "Omni | " + this.$route.name;
    }
  },
  name: "App",
  beforeCreate() {
    // Init
    this.$holochain.then(({ call, close }) => {
      const params = {
        instance_id: this.DNA_OMNI,
        zome: "users",
        function: "register_self",
        params: {
          anchor: "all_users"
        }
      };
      call("call")(params)
        .then(response => {
          this.$store.dispatch("anchor_users", JSON.parse(response).Ok);
        })
        .catch(error => {
          console.error(error);
        });
    });
  }
};
</script>

<style scoped></style>
<style scoped src="@/assets/css/global-animations.css"></style>
<style scoped src="@/assets/css/master.css"></style>
