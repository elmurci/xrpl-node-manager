<template>
  <label class="inline-flex items-center" :for="id">
    <span class="ml-1 text-gray-700">
    <input
      :id="id"
      :name="name"
      :checked="checked"
      :value="value"
      :disabled="disabled"
      v-on="listeners"
      class="form-checkbox"
      type="checkbox">
    {{ this.label }}
    </span>
  </label>
</template>

<script lang="ts">
export default {
  name: 'UiErrorBox',
  props: {
    checked: Boolean,
    value: String,
    label: String,
    name: String,
    id: String,
    disabled: Boolean
  },
  methods: {
    getRandomInt () {
      const min = 1
      const max = 1000000000
      return Math.floor(Math.random() * (max - min + 1) + min)
    }
  }
}
</script>
