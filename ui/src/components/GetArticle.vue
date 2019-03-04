<template>
  <div>
    <h3 class="ui large horizontal divider header">Get article from address</h3>
    <div class="ui form" id="form-retrieve">
      <div class="field">
        <input
          type="text"
          id="address"
          placeholder="e.g. QmTe9kPbDs2YXs7KGJ4FQxeeTAZ1ssncZeYuXiPt9U5d5T"
          v-model="get_address"
        />
      </div>
    </div>
    <loading-article-block v-if="is_loading"></loading-article-block>
    <article-block v-if="is_article" :article="article"></article-block>
    <zome-message
      class="negative"
      v-if="is_zome_message"
      :message="zome_message"
      @dismissed="clearMessage()"
    ></zome-message>
  </div>
</template>

<script>
import { connect } from "@holochain/hc-web-client";
import LoadingArticleBlock from "./LoadingArticleBlock";
import ArticleBlock from "./ArticleBlock";
import ZomeMessage from "./ZomeMessage";

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
      is_article: false,
      is_zome_message: false,
      zome_message: ""
    };
  },
  components: {
    LoadingArticleBlock,
    ArticleBlock,
    ZomeMessage
  },
  methods: {
    getArticle(get_address) {
      this.is_zome_message = false;
      this.is_loading = true;
      this.is_article = false;
      connect("ws:localhost:8888").then(({ call, close }) => {
        const params = {
          article_addr: get_address
        };

        call("test-instance/articles/get_article")(params)
          .then(response => {
            this.handleGetResponse(JSON.parse(response));
          })
          .catch(error => console.error(error));
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
          this.is_article = true;
        }
      }
    },
    clearMessage() {
      this.is_zome_message = false;
    }
  },
  watch: {
    get_address() {
      if (this.get_address.length === 46) {
        this.getArticle(this.get_address);
      }
    }
  }
};
</script>

<style scoped>
.divider {
  padding: 1em;
}
</style>
