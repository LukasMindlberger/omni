<template>
  <div>
    <h3 class="ui large horizontal divider header">Commit a new article</h3>

    <div class="ui form" id="form-submit">
      <div class="field">
        <textarea
          rows="1"
          type="text"
          id="title"
          placeholder="Article title"
          v-model="submit_title"
          >{{ submit_title }}</textarea
        >
      </div>
      <div class="field">
        <textarea
          rows="3"
          type="text"
          id="abstract"
          placeholder="Article abstract"
          v-model="submit_abstract"
          >{{ submit_abstract }}</textarea
        >
      </div>
      <div class="field">
        <textarea
          type="text"
          id="body"
          placeholder="Article body"
          v-model="submit_body"
          >{{ submit_body }}</textarea
        >
      </div>
      <button
        class="ui primary button"
        type="submit"
        id="submitBtn"
        @click="submitForm()"
        :class="{ loading: is_loading }"
      >
        Submit
      </button>
      <button
        class="right floated ui secondary button"
        id="clearBtn"
        @click="clearForm()"
      >
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
      submit_title: "",
      submit_abstract: "",
      submit_body: "",
      is_zome_message: false,
      is_loading: false,
      submit_had_error: false,
      zome_message: ""
    };
  },
  methods: {
    clearForm() {
      (this.submit_title = ""),
        (this.submit_abstract = ""),
        (this.submit_body = "");
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
          title: this.submit_title,
          abst: this.submit_abstract,
          body: this.submit_body
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
