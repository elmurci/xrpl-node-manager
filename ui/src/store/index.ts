import { createStore } from "vuex";

export const store = createStore({
  state () {
    return {
      ws: {
        open: false,
        info: {},
        stats: {},
        status: false,
        config: {},
      },
      option: '',
    }
  },
  mutations: {
    info (state, info) {
      state.ws.info = info;
    },
    config (state, config) {
      state.ws.config = config;
    },
    option (state, option) {
      state.option = option;
    },
    status (state, status) {
      state.ws.status = status;
    },
    stats (state, stats) {
      state.ws.stats = stats;
    },
    wsConnected (state) {
      state.ws.open = true;
    },
    wsDisconnected (state) {
      state.ws.open = false;
    }
  }
})
