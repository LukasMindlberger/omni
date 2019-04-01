import Vue from "vue";
import Router from "vue-router";

import OmniHome from "@/views/OmniHome";
import OmniSubmitArticle from "@/views/OmniSubmit";
import OmniGetArticle from "@/views/OmniGet";
import OmniHeader from "@/components/OmniHeader";
import ZomeMessage from "@/components/common/ZomeMessage";

Vue.use(Router);

export default new Router({
  routes: [
    {
      path: "/",
      name: "Home",
      component: OmniHome
    },
    {
      path: "/submit",
      name: "Submit Article",
      component: OmniSubmitArticle
    },
    {
      path: "/repository",
      name: "Repository",
      component: OmniGetArticle
    }
  ]
});
