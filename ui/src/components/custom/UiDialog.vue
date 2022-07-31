<template>
  <component
    :is="$modal.context.componentName"
    :name="name"
    :height="height"
    :classes="['vue-dialog', this.params.class]"
    :width="width"
    :json="json"
    :id="id"
    :shift-y="0.3"
    :adaptive="true"
    :focus-trap="true"
    :clickToClose="clickToClose"
    :transition="transition"
    @before-open="beforeOpened"
    @before-close="beforeClosed"
    @opened="$emit('opened', $event)"
    @closed="$emit('closed', $event)"
  >
    <div class="text-left vue-dialog-content">
      <div class="vue-dialog-content-title" v-if="params.title" v-html="params.title || ''" />
      <component v-if="params.component" v-bind="params.props" :is="params.component" />
      <div v-else v-html="params.text || ''" />
        <textarea class="p-4 m-4 jsonta" :id="`${id}_textarea`" v-model="json"></textarea>
      </div>
  </component>
</template>

<script lang="ts">
export default {
  name: 'UiDialog',
  data() {
    return {
      params: {},
    }
  },
  props: {
    width!: String,
    height!: String,
    clickToClose!: Boolean,
    transition!: Boolean,
    json!: String,
    name!: String,
    id!: String
  }
}
</script>
<style>
.jsonta {
    width: 97%;
    height: 45vh;
    background: #f9f9f9;
}
</style>
