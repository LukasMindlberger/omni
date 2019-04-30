import Vue from "vue";
import Router from "vue-router";

import OmniSubmitArticle from "@/views/OmniSubmit";
import OmniGetArticle from "@/views/OmniGet";
import Communities from "@/views/communities/Communities";
import Community from "@/views/communities/Community";
import Dummy from "@/components/common/Dummy";
import Lost from "@/components/common/Lost";

Vue.use(Router);

export default new Router({
  routes: [
    {
      path: "/",
      name: "Home",
      component: Dummy
    },
    {
      path: "/submit",
      name: "Submit",
      component: OmniSubmitArticle
    },
    {
      path: "/repository",
      name: "Repository",
      component: OmniGetArticle
    },
    {
      path: "/communities",
      name: "Communities",
      component: Communities
    },
    {
      path: "/c/astrophysics",
      name: "Astrophysics",
      component: Community
    },
    {
      path: "*",
      name: "Lost",
      component: Lost
    }
  ]
});
