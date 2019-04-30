import Vue from "vue";
import Vuex from "vuex";

Vue.use(Vuex);

export default new Vuex.Store({
  state: {
    page_title: "Omni",
    nav_text: "Navigation"
  },
  getters: {
    pageTitle: state => {
      return state.page_title;
    },
    navText: state => {
      return state.nav_text;
    }
  },
  mutations: {
    updateTitle: (state, payload) => {
      state.page_title = payload;
    },
    updateNavText: (state, payload) => {
      state.nav_text = payload;
    }
  },
  actions: {
    updateTitle: ({ commit }, payload) => {
      commit("updateTitle", payload);
    },
    updateNavText: ({ commit }, payload) => {
      commit("updateNavText", payload);
    }
  }
});
