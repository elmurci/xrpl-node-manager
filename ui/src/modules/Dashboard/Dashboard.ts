import DefaultLayout from '@/layouts/default/index.vue';
import { computed, defineComponent, inject } from 'vue';
import { useStore } from 'vuex';
import moment from 'moment';
import { useI18n } from 'vue-i18n';
import UiTickOrX from '@/components/custom/UiTickOrX.vue';
import { Topic } from '@/enums';

export default defineComponent({
  name: 'Dashboard',
  components: {
    DefaultLayout,
    UiTickOrX,
  },
  inject: ['worker'],
  setup() {
    const store = useStore();
    const { t, locale } = useI18n({
      useScope: 'global'
    });
    const wsConnected = computed(() => store.state.ws.open);
    const status = computed(() => store.state.ws.status);
    const info = computed(() => store.state.ws.info);
    const stats = computed(() => store.state.ws.stats);
    const config = computed(() => store.state.ws.config);
    const features = computed(() => store.state.ws.features);
    return {
      store,
      wsConnected,
      status,
      stats,
      features,
      info,
      config,
      moment,
      t,
      locale,
    };
  },
  data() {
    return {
      worker: this.worker,
      isLoadingData: false,
      errorText: '',
      error: false,
    };
  },
  mounted () {
    console.log('this.worker', this.worker);
    // this.worker.postMessage('PERIQUITO!!!');
  }
});
