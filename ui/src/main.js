import Vue from "vue";
import "semantic-ui-css/semantic.min.css";
import App from "./App.vue";
import router from "./router";
import store from "./store";

import { connect } from "@holochain/hc-web-client";
Vue.prototype.$holochain = connect("ws:localhost:8888");

Vue.prototype.DNA_OMNI = "test-instance";

// Custom directive to change browser window title
Vue.directive("title", {
  inserted: (el, binding) => (document.title = binding.value),
  update: (el, binding) => (document.title = binding.value)
});

Vue.config.productionTip = false;

new Vue({
  router,
  store,
  render: h => h(App)
}).$mount("#app");
