<template>
  <div class="ui basic segment">
    <h3 class="ui horizontal divider header">Get article from address</h3>
    <div class="ui form">
      <div class="field">
        <div class="ui icon input" :class="{ loading: is_loading }">
          <input
            type="text"
            placeholder="e.g. QmTe9kPbDs2YXs7KGJ4FQxeeTAZ1ssncZeYuXiPt9U5d5T"
            v-model="get_address"
            @keyup.enter="getArticle(get_address)"
          />
          <i
            class="link icon"
            :class="{ search: !searching, close: searching }"
            @click="
              get_address = '';
              clearArticleArea();
            "
          ></i>
        </div>
      </div>
    </div>
    <transition name="fade" mode="out-in">
      <loading-article-block v-if="is_loading"></loading-article-block>
      <article-block v-if="show_article" :article="article"></article-block>
      <zome-message
        class="negative"
        v-if="is_zome_message"
        :message="zome_message"
        @dismissed="clearMessage()"
      ></zome-message>
    </transition>
  </div>
</template>

<script>
import ArticleBlock from "@/components/articles/ArticleBlock";
import LoadingArticleBlock from "@/components/articles/LoadingArticleBlock";
import ZomeMessage from "@/components/common/ZomeMessage";

export default {
  data() {
    return {
      article: {
        title: "",
        abstract: "",
        body: ""
      },
      get_address: "",
      is_loading: false,
      show_article: false,
      is_zome_message: false,
      zome_message: ""
    };
  },
  computed: {
    searching: function() {
      if (this.get_address === "") {
        return false;
      } else {
        return true;
      }
    }
  },
  components: {
    LoadingArticleBlock,
    ArticleBlock,
    ZomeMessage
  },
  methods: {
    getArticle(get_address) {
      this.clearArticleArea();
      this.is_loading = true;
      if (get_address.length !== 46) {
        this.zome_message = "Invalid hash address";
        this.is_zome_message = true;
        this.is_loading = false;
        return;
      }
      this.$holochain.then(({ call, close }) => {
        const params = {
          instance_id: "test-instance",
          zome: "articles",
          function: "get_article",
          params: {
            article_addr: get_address
          }
        };
        call("call")(params)
          .then(response => {
            this.handleGetResponse(JSON.parse(response));
          })
          .catch(error => {
            console.error(error);
          });
      });
    },
    handleGetResponse(response) {
      if (response.Err) {
        this.is_loading = false;
        (this.zome_message = "Error: " + JSON.stringify(response.Err)),
          (this.is_zome_message = true);
      } else {
        let article = response.Ok;

        this.is_loading = false;

        if (article === null) {
          this.zome_message = "No article found";
          this.is_zome_message = true;
        } else {
          article = JSON.parse(article.App[1]);
          this.article.title = article.title;
          this.article.abstract = article.abst;
          this.article.body = article.body;
          this.show_article = true;
        }
      }
    },
    clearArticleArea() {
      this.clearMessage();
      this.show_article = false;
    },
    clearMessage() {
      this.is_zome_message = false;
    }
  },
  watch: {
    get_address: function() {
      if (this.get_address.length === 46) {
        this.getArticle(this.get_address);
      } else if (this.get_address.length === 0) {
        this.show_article = false;
      }
    }
  }
};
</script>
