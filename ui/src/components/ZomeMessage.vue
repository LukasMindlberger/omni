<template>
  <div class="ui message">
    {{ message }}<i class="close icon" @click="$emit('dismissed')"></i>
  </div>
</template>

<script>
export default {
  props: {
    message: String
  }
};
</script>

<style scoped>
#message {
  word-wrap: break-word;
}
</style>
