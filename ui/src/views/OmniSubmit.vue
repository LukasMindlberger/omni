<template>
  <div class="ui basic segment">
    <h3 class="ui horizontal divider header">Submit article to Omni</h3>
    <div class="ui form">
      <div class="field">
        <textarea
          rows="1"
          type="text"
          placeholder="Title"
          v-model.lazy="article.title"
          >{{ article.title }}</textarea
        >
      </div>
      <div class="field">
        <textarea
          rows="3"
          type="text"
          placeholder="Abstract"
          v-model.lazy="article.abstract"
          >{{ article.abstract }}</textarea
        >
      </div>
      <div class="field">
        <textarea type="text" placeholder="Body" v-model.lazy="article.body">
          {{ article.body }}</textarea
        >
      </div>
      <zome-message
        class="positive"
        v-if="is_zome_message"
        :class="{ negative: submit_had_error }"
        :message="zome_message"
        @dismissed="clearMessage()"
      >
      </zome-message>
      <button
        class="ui primary button"
        @click.prevent="submitForm()"
        :class="{ loading: is_loading }"
      >
        Submit
      </button>
      <button
        class="right floated ui secondary button"
        @click.prevent="
          clearMessage();
          clearForm();
        "
      >
        Clear
      </button>
    </div>
  </div>
</template>

<script>
import OmniHeader from "@/components/OmniHeader";
import ZomeMessage from "@/components/common/ZomeMessage";

export default {
  components: {
    OmniHeader,
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
      response: "",
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
      (this.zome_message = ""),
        (this.submit_had_error = false),
        (this.is_zome_message = false);
    },
    submitForm() {
      this.is_loading = true;
      this.is_zome_message = false;
      this.$holochain.then(({ call, close }) => {
        const params = {
          instance_id: this.DNA_OMNI,
          zome: "articles",
          function: "create_article",
          params: {
            title: this.article.title,
            abst: this.article.abstract,
            body: this.article.body
          }
        };
        call("call")(params)
          .then(response => {
            this.handleSubmitResponse(JSON.parse(response));
          })
          .catch(error => {
            console.error(error);
          });
      });
    },
    handleSubmitResponse(response) {
      this.response = response;
      this.show_copied = false;
      this.is_loading = false;
      if (response.Err) {
        (this.zome_message = "Error: " + JSON.stringify(response.Err)),
          (this.submit_had_error = true);
      } else {
        (this.zome_message = "Success: " + response.Ok),
          (this.submit_had_error = false);
      }
      this.is_zome_message = true;
    },
    toClipboard(content) {
      const dummy = document.createElement("input");
      document.body.appendChild(dummy);
      dummy.setAttribute("value", content);
      dummy.select();
      document.execCommand("copy");
      document.body.removeChild(dummy);
    }
  }
};
</script>

<style scoped></style>
<style scoped src="@/assets/css/global-animations.css"></style>
