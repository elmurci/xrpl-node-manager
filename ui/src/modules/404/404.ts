import { defineComponent } from "vue";

export default defineComponent({
  name: "404",
  data() {
    return {
      context: process.env.VUE_APP_CONTEXT,
    };
  },
});
