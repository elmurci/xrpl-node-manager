import { defineComponent } from "vue";
import AppHeader from '@/components/layout/AppHeader.vue'
import AppFooter from '@/components/layout/AppFooter.vue'

export default defineComponent({
    name: 'AppLayout',
    components: {
        AppHeader,
        AppFooter,
    }
});
