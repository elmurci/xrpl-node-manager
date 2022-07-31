<template>
  <div class="flex justify-center bg-white border-t-2 footer">
    <div class="flex w-full lg:ml-4">
      <div class="w-3/5 py-1 text-left">
        <!-- Language -->
        <ul class="inline list-inline">
          <li class="float-left">
            <button :class="(currentLocale == 'es') ? 'text-teal-500' : 'text-gray-500'" @click="setLocale('es')" :title="t('language.es.title')" class="border-0 btn btn-default focus:outline-none">{{ t('language.es.button') }}</button>
          </li>
          <li class="float-left ml-3">
            <button :class="(currentLocale == 'en') ? 'text-teal-500' : 'text-gray-500'" @click="setLocale('en')" :title="t('language.en.title')" class="border-0 btn btn-default focus:outline-none">{{ t('language.en.button') }}</button>
          </li>
        </ul>
        <a href="schema/schema.json" target="_blank" class="text-blue-400 ml-3" title="JSON Schema">
          {{ t('header.schema') }}
        </a>
      </div>
    </div>
  </div>
</template>

<script lang="ts">
import { useI18n } from 'vue-i18n'
export default {
  name: 'AppFooter',
  setup() {
    const { t, locale } = useI18n({
      useScope: 'global'
    })
    return { t, locale }
  }
}
</script>

<style>
.footer {
  position: fixed;
  bottom: 0px;
  left: 0px;
  right: 0px;
  margin-bottom: 0px;
}
</style>
