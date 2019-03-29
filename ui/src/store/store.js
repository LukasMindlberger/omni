import Vue from "vue";
import Vuex from "vuex";

Vue.use(Vuex);

export const store = new Vuex.Store({
  state: {
    user_anchor: "",
    users: ""
  },
  getters: {
    user_anchor: state => {
      return state.user_anchor;
    },
    users: state => {
      return state.users;
    }
  },
  mutations: {
    anchor_users: (state, payload) => {
      state.user_anchor = payload;
    },
    users: (state, payload) => {
      state.users = payload;
    }
  },
  actions: {
    anchor_users: ({ commit }, payload) => {
      commit("anchor_users", payload);
    },
    users: ({ commit }, payload) => {
      commit("users", payload);
    }
  }
});
