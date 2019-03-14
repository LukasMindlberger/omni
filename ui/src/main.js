import Vue from "vue";
import App from "./App.vue";
import { connect } from "@holochain/hc-web-client";

new Vue({
  el: "#app",
  render: h => h(App)
});
