<template>
  <div class="ui center aligned container">
    <div class="ui basic segment">
      <button class="ui button" @click="get_all_users()">
        Get users
      </button>
      <div v-if="users" class="ui header">
        Registered Users
      </div>
      <div class="ui middle aligned divided list">
        <div class="item" v-for="user in users">{{ user }}</div>
      </div>
      <h3 class="ui horizontal divider header">Get profile from address</h3>
      <div class="ui form">
        <div class="field">
          <div class="ui icon input" :class="{ loading: is_loading }">
            <input
              type="text"
              placeholder="e.g. HcScjN8wBwrn3tuyg89aab3a69xsIgdzmX5P9537BqQZ5A7TEZu7qCY4Xzzjhma"
              v-model="get_address"
            />
            <i
              class="link icon"
              :class="{ search: !searching, close: searching }"
              @click="get_address = ''"
            ></i>
          </div>
        </div>
      </div>
      <transition name="slide-fade" mode="out-in">
        <profile-articles></profile-articles>
      </transition>
    </div>
  </div>
</template>

<script>
import ProfileArticles from "@/components/profiles/ProfileArticles";
export default {
  data() {
    return {
      is_loading: false,
      get_address: "",
      show_profile: false
    };
  },
  computed: {
    searching: function() {
      if (this.get_address === "") {
        return false;
      } else {
        return true;
      }
    },
    user_anchor: function() {
      return this.$store.getters.user_anchor;
    },
    users: function() {
      return this.$store.getters.users;
    }
  },
  methods: {
    getProfile(address) {
      this.is_loading = true;
      this.$holochain.then(({ call, close }) => {
        const params = {
          instance_id: "test-instance",
          zome: "articles",
          function: "get_authored_articles",
          params: {
            agent_addr: this.get_address
          }
        };
        call("call")(params)
          .then(response => {
            console.log(response);
            this.is_loading = false;
            this.show_profile = true;
          })
          .catch(error => {
            console.error(error);
          });
      });
    },
    get_all_users() {
      this.$holochain.then(({ call, close }) => {
        const params = {
          instance_id: this.DNA_OMNI,
          zome: "users",
          function: "get_users",
          params: {
            anchor_address: this.user_anchor
          }
        };
        call("call")(params)
          .then(response => {
            this.$store.dispatch("users", JSON.parse(response).Ok.addresses);
          })
          .catch(error => {
            console.error(error);
          });
      });
    }
  },
  components: {
    ProfileArticles
  },
  watch: {
    get_address: function() {
      if (this.get_address.length !== 0) {
        this.getProfile(this.get_address);
      } else if (this.get_address.length === 0) {
        this.show_profile = false;
      }
    }
  }
};
</script>

<style scoped></style>
