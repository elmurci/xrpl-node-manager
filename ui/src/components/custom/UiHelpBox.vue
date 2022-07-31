<template>
  <div>
    <h3>
      <i class="fas fa-info-circle text-blue-500"></i>
      {{ t('helpBox.title') }}
    </h3>
    <div class="mt-2 bg-gray-200 rounded-md flex flex-1 items-center p-4 transition duration-500 ease-in-out transform hover:-translate-y-1 hover:shadow-lg">
      <div class="flex-1 pl-1" v-html="content"></div>
    </div>
  </div>
</template>

<script lang="ts">
import axios from 'axios'
import { useI18n } from 'vue-i18n'

export default {
  name: 'UIHelpBox',
  props: {
    contentKey: String
  },
  setup() {
    const { t, locale } = useI18n({
      useScope: 'global'
    })
    return { t, locale }
  },
  data() {
    return {
      content: null,
    }
  },
  watch: {
    contentKey (newVal: string) {
      console.log("*********", newVal);
      axios.get(`content/html/${newVal}_${this.locale}.html`).then(response => {
        this.content = response.data
      })
        .catch(error => {
          this.content = error
        })
    }
  }
}
</script>

