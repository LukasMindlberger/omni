<template>
  <transition name="slide-fade">
    <div class="ui message" id="message">
      {{ message }}<i class="close icon" @click="$emit('dismissed')"></i>
    </div>
  </transition>
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
