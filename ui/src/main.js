// The Vue build version to load with the `import` command
// (runtime-only or standalone) has been set in webpack.base.conf with an alias.
import Vue from "vue";
import App from "./App";
import router from "./router";
import { store } from "./store/store";

import { connect } from "@holochain/hc-web-client";
Vue.prototype.$holochain = connect("ws:localhost:8888");

Vue.prototype.DNA_OMNI = "test-instance";

// Custom directive to change browser window title
Vue.directive("title", {
  inserted: (el, binding) => (document.title = binding.value),
  update: (el, binding) => (document.title = binding.value)
});

Vue.config.productionTip = false;

/* eslint-disable no-new */
new Vue({
  el: "#app",
  router,
  store,
  components: { App },
  template: "<App/>"
});
