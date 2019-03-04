<template>
  <div class="ui container">
    <h3 class="ui horizontal divider header">Commit a new article</h3>

    <div class="ui form">
      <div class="field">
        <textarea
          rows="1"
          type="text"
          placeholder="Title"
          v-model="article.title"
          >{{ article.title }}</textarea
        >
      </div>
      <div class="field">
        <textarea
          rows="3"
          type="text"
          placeholder="Abstract"
          v-model="article.abstract"
          >{{ article.abstract }}</textarea
        >
      </div>
      <div class="field">
        <textarea type="text" placeholder="Body" v-model="article.body">{{
          article.body
        }}</textarea>
      </div>
      <button
        class="ui primary button"
        type="submit"
        @click="submitForm()"
        :class="{ loading: is_loading }"
      >
        Submit
      </button>
      <button class="right floated ui secondary button" @click="clearForm()">
        Clear
      </button>
      <zome-message
        class="positive"
        v-if="is_zome_message"
        :class="{ negative: submit_had_error }"
        :message="zome_message"
        @dismissed="clearMessage()"
      ></zome-message>
    </div>
  </div>
</template>

<script>
import { connect } from "@holochain/hc-web-client";
import ZomeMessage from "./ZomeMessage.vue";

export default {
  components: {
    ZomeMessage
  },
  data() {
    return {
      article: {
        title: "",
        abstract: "",
        body: ""
      },
      is_zome_message: false,
      is_loading: false,
      submit_had_error: false,
      zome_message: ""
    };
  },
  methods: {
    clearForm() {
      (this.article.title = ""),
        (this.article.abstract = ""),
        (this.article.body = "");
    },
    clearMessage() {
      (this.is_zome_message = false),
        (this.submit_had_error = false),
        (this.zome_message = "");
    },
    submitForm() {
      this.is_loading = true;
      this.is_zome_message = false;
      connect("ws:localhost:8888").then(({ call, close }) => {
        const params = {
          title: this.article.title,
          abst: this.article.abstract,
          body: this.article.body
        };

        call("test-instance/articles/create_article")(params)
          .then(response => {
            this.handleSubmitResponse(JSON.parse(response));
          })
          .catch(error => console.error(error));
      });
    },
    handleSubmitResponse(response) {
      this.is_loading = false;
      this.is_zome_message = true;
      if (response.Err) {
        (this.zome_message = "Error: " + JSON.stringify(response.Err)),
          (this.submit_had_error = true);
      } else {
        (this.zome_message = "Success: " + response.Ok),
          (this.submit_had_error = false);
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
