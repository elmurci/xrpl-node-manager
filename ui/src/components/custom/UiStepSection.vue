<template>
  <div @mouseenter="focus()" class="hover:bg-teal-100 pl-4 pr-4 py-2" :class="additionalClasses">
    <slot></slot>
  </div>
</template>

<script lang="ts">
export default {
  name: 'UiStepSection',
  props: {
    additionalClasses: String,
    helpBox: String
  },
  methods: {
    focus () {
      this.$emit('item-focus', this.helpBox)
    }
  }
}
</script>
