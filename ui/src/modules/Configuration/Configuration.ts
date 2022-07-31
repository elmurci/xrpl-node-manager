import DefaultLayout from "@/layouts/default/index.vue";
import { defineComponent } from "vue";
import { useStore } from "vuex";
import moment from "moment";
import UiStepSection from '@/components/custom/UiStepSection.vue'
import UiCheckbox from '@/components/custom/UiCheckbox.vue'
import UiDropdown from '@/components/custom/UiDropdown.vue'
import UiErrorBox from '@/components/custom/UiErrorBox.vue'
import UiHelpBox from '@/components/custom/UiHelpBox.vue'
import { required, ipAddress, between, numeric } from 'vuelidate/lib/validators'
import HttpClient from "@/util/HttpClient";
import routes from "@/routes";
import useVuelidate from '@vuelidate/core'
import { useI18n } from "vue-i18n";

const openedRows: string[] = [];

export default defineComponent({
  name: "Dashboard",
  components: {
    DefaultLayout,
    UiStepSection,
    UiCheckbox,
    UiDropdown,
    UiErrorBox,
    UiHelpBox,
  },
  setup() {
    const store = useStore();
    const { t, locale } = useI18n({
      useScope: 'global'
    })
    return {
      store,
      moment,
      v$: useVuelidate(),
      t,
      locale,
    };
  },
  data() {
    return {
      helpBoxContentKey:'',
    };
  },
  async mounted() {
  },
  methods: {
    itemFocus (value: string): void {
      this.helpBoxContentKey = value;
    },
    itemSelect (value: any): void {
    }
  },
});
