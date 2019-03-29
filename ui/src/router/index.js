import Vue from "vue";
import Router from "vue-router";

import OmniHome from "@/pages/OmniHome";
import OmniSubmitArticle from "@/pages/OmniSubmit";
import OmniGetArticle from "@/pages/OmniGet";
import OmniProfile from "@/pages/OmniProfile";
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
    },
    {
      path: "/profiles",
      name: "Profiles",
      component: OmniProfile
    }
  ]
});
