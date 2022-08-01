import DefaultLayout from "@/layouts/default/index.vue";
import { computed, defineComponent, inject } from "vue";
import { useStore } from "vuex";
import moment from "moment";
import { useI18n } from 'vue-i18n'

export default defineComponent({
  name: "Stats",
  components: {
    DefaultLayout,
  },
  setup() {
    const store = useStore();
    const { t, locale } = useI18n({
      useScope: 'global'
    })
    const wsClient: any = inject('wsClient')
    const wsConnected = computed(() => store.state.ws.open);
    const status = computed(() => store.state.ws.status);
    const info = computed(() => store.state.ws.info);
    const stats = computed(() => store.state.ws.stats);
    const config = computed(() => store.state.ws.config);
    wsClient.send('config');
    return {
      store,
      wsConnected,
      status,
      stats,
      info,
      config,
      moment,
      t,
      locale,
    };
  },
  data() {
    return {
      isLoadingData: false,
      errorText: "",
      error: false,
    };
  },
});
