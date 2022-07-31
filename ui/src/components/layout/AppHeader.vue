<template>
  <nav class="bg-gray-800">
    <div class="mx-auto px-8">
      <div class="relative flex items-center justify-between h-16">
        <div class="absolute inset-y-0 left-0 flex items-center sm:hidden">
          <!-- Mobile menu button-->
          <button type="button" class="inline-flex items-center justify-center p-2 rounded-md text-gray-400 hover:text-white hover:bg-gray-700 focus:outline-none focus:ring-2 focus:ring-inset focus:ring-white" aria-controls="mobile-menu" aria-expanded="false">
            <span class="sr-only">Open main menu</span>
            <!--
              Icon when menu is closed.

              Heroicon name: outline/menu

              Menu open: "hidden", Menu closed: "block"
            -->
            <svg class="block h-6 w-6" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" aria-hidden="true">
              <path stroke-linecap="round" stroke-linejoin="round" d="M4 6h16M4 12h16M4 18h16" />
            </svg>
            <!--
              Icon when menu is open.

              Heroicon name: outline/x

              Menu open: "block", Menu closed: "hidden"
            -->
            <svg class="hidden h-6 w-6" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" aria-hidden="true">
              <path stroke-linecap="round" stroke-linejoin="round" d="M6 18L18 6M6 6l12 12" />
            </svg>
          </button>
        </div>
        <div class="flex-1 flex items-center justify-center sm:items-stretch sm:justify-start">
          <div class="flex-shrink-0 flex items-center">
            <h1 class="text-md font-bold leading-7 text-white sm:leading-9 sm:truncate float-left">
              <svg width="36" viewBox="0 0 512 424" class="inline -mt-1" version="1.1" xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink">
                <g id="Canvas" fill="none">
                  <g id="xrp-symbol">
                    <path id="Vector" d="M 436.043 0L 510.096 0L 356.029 152.475C 300.258 207.67 209.837 207.67 154.067 152.475L 2.51594e-08 0L 74.0526 0L 191.093 115.832C 226.414 150.788 283.68 150.788 319.002 115.832L 436.043 0Z" transform="translate(0.943848 0)" fill="#2A6DB0"/>
                    <path id="Vector_2" d="M 74.0527 194.813L 0 194.813L 155.019 41.3955C 210.789 -13.7985 301.21 -13.7985 356.981 41.3955L 512 194.813L 437.947 194.813L 319.953 78.0385C 284.632 43.0821 227.366 43.0821 192.045 78.0385L 74.0527 194.813Z" transform="translate(0 229.187)" fill="#2A6DB0"/>
                  </g>
                </g>
              </svg>
              {{ t('header.title') }}
            </h1>
          </div>
          <div class="hidden sm:block sm:ml-6">
            <div class="flex space-x-4">
              <!-- Current: "bg-gray-900 text-white", Default: "text-gray-300 hover:bg-gray-700 hover:text-white" -->
              <router-link :to="{ path: '/' }" @click="selectOption('dashboard')" :class="getOptionClass('dashboard')" class="px-3 py-2 rounded-md text-sm font-medium">Dashboard</router-link>
              <router-link :to="{ path: '/configuration' }" @click="selectOption('configuration')" :class="getOptionClass('configuration')" class="px-3 py-2 rounded-md text-sm font-mediumm">Configuration</router-link>
              <router-link :to="{ path: '/logs' }" @click="selectOption('logs')" :class="getOptionClass('logs')" class="px-3 py-2 rounded-md text-sm font-medium">Logs</router-link>
            </div>
          </div>
        </div>
        <div class="absolute inset-y-0 right-0 flex items-center pr-2 sm:static sm:inset-auto sm:ml-6 sm:pr-0">
            <div class="float-right">
              <span v-if="wsConnected && status && info.complete_ledgers === 'empty'" class="bg-yellow-100 text-yellow-800 text-xs font-semibold mr-2 px-2.5 py-0.5 rounded dark:bg-yellow-200 dark:text-yellow-900">Node is syncing</span> 
              <span v-if="wsConnected && status && info && info.closed_ledger" class="bg-blue-100 text-blue-800 text-xs font-semibold mr-2 px-2.5 py-0.5 rounded dark:bg-blue-200 dark:text-blue-800">Ledger Sequence: {{info && info.closed_ledger && info.closed_ledger.seq}}</span>
              <svg v-if="!wsConnected" xmlns="http://www.w3.org/2000/svg" class="h-5 w-5 fill-red-400 float-right mt-1" viewBox="0 0 20 20" fill="currentColor">
                  <path d="M3.707 2.293a1 1 0 00-1.414 1.414l6.921 6.922c.05.062.105.118.168.167l6.91 6.911a1 1 0 001.415-1.414l-.675-.675a9.001 9.001 0 00-.668-11.982A1 1 0 1014.95 5.05a7.002 7.002 0 01.657 9.143l-1.435-1.435a5.002 5.002 0 00-.636-6.294A1 1 0 0012.12 7.88c.924.923 1.12 2.3.587 3.415l-1.992-1.992a.922.922 0 00-.018-.018l-6.99-6.991zM3.238 8.187a1 1 0 00-1.933-.516c-.8 3-.025 6.336 2.331 8.693a1 1 0 001.414-1.415 6.997 6.997 0 01-1.812-6.762zM7.4 11.5a1 1 0 10-1.73 1c.214.371.48.72.795 1.035a1 1 0 001.414-1.414c-.191-.191-.35-.4-.478-.622z" />
              </svg>
              <svg v-if="wsConnected" xmlns="http://www.w3.org/2000/svg" class="h-5 w-5 fill-green-400 float-right mt-1" viewBox="0 0 20 20" fill="currentColor">
                <path fill-rule="evenodd" d="M5.05 3.636a1 1 0 010 1.414 7 7 0 000 9.9 1 1 0 11-1.414 1.414 9 9 0 010-12.728 1 1 0 011.414 0zm9.9 0a1 1 0 011.414 0 9 9 0 010 12.728 1 1 0 11-1.414-1.414 7 7 0 000-9.9 1 1 0 010-1.414zM7.879 6.464a1 1 0 010 1.414 3 3 0 000 4.243 1 1 0 11-1.415 1.414 5 5 0 010-7.07 1 1 0 011.415 0zm4.242 0a1 1 0 011.415 0 5 5 0 010 7.072 1 1 0 01-1.415-1.415 3 3 0 000-4.242 1 1 0 010-1.415zM10 9a1 1 0 011 1v.01a1 1 0 11-2 0V10a1 1 0 011-1z" clip-rule="evenodd" />
              </svg>
            </div>
            <div class="float-right">
              <button v-if="wsConnected && !status" id="startBtn" :disabled="disable" @click="send('start')" class="inline-flex px-4 py-2 font-bold text-white bg-blue-500 border-b-4 border-blue-700 rounded hover:bg-blue-700 hover:border-blue-800 ml-4" :class="(disable) ? 'disabled:opacity-50 cursor-not-allowed' : ''">
                <svg v-if="isWorking" role="status" class="w-5 h-5 text-white animate-spin" viewBox="0 0 100 101" fill="none" xmlns="http://www.w3.org/2000/svg">
                  <path d="M100 50.5908C100 78.2051 77.6142 100.591 50 100.591C22.3858 100.591 0 78.2051 0 50.5908C0 22.9766 22.3858 0.59082 50 0.59082C77.6142 0.59082 100 22.9766 100 50.5908ZM9.08144 50.5908C9.08144 73.1895 27.4013 91.5094 50 91.5094C72.5987 91.5094 90.9186 73.1895 90.9186 50.5908C90.9186 27.9921 72.5987 9.67226 50 9.67226C27.4013 9.67226 9.08144 27.9921 9.08144 50.5908Z" fill="#E5E7EB"/>
                  <path d="M93.9676 39.0409C96.393 38.4038 97.8624 35.9116 97.0079 33.5539C95.2932 28.8227 92.871 24.3692 89.8167 20.348C85.8452 15.1192 80.8826 10.7238 75.2124 7.41289C69.5422 4.10194 63.2754 1.94025 56.7698 1.05124C51.7666 0.367541 46.6976 0.446843 41.7345 1.27873C39.2613 1.69328 37.813 4.19778 38.4501 6.62326C39.0873 9.04874 41.5694 10.4717 44.0505 10.1071C47.8511 9.54855 51.7191 9.52689 55.5402 10.0491C60.8642 10.7766 65.9928 12.5457 70.6331 15.2552C75.2735 17.9648 79.3347 21.5619 82.5849 25.841C84.9175 28.9121 86.7997 32.2913 88.1811 35.8758C89.083 38.2158 91.5421 39.6781 93.9676 39.0409Z" fill="currentColor"/>
                </svg>
                <svg v-if="!isWorking" xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" viewBox="0 0 20 20" fill="currentColor">
                  <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zM9.555 7.168A1 1 0 008 8v4a1 1 0 001.555.832l3-2a1 1 0 000-1.664l-3-2z" clip-rule="evenodd" />
                </svg>
              </button>
              <button v-if="wsConnected && info.complete_ledgers && status" id="stopBtn" :disabled="disable" @click="send('stop')" class="inline-flex px-4 py-2 font-bold text-white bg-blue-500 border-b-4 border-blue-700 rounded hover:bg-blue-700 hover:border-blue-800 ml-4" :class="(disable) ? 'disabled:opacity-50 cursor-not-allowed' : ''">
                <svg v-if="isWorking" role="status" class="w-5 h-5 text-white animate-spin" viewBox="0 0 100 101" fill="none" xmlns="http://www.w3.org/2000/svg">
                  <path d="M100 50.5908C100 78.2051 77.6142 100.591 50 100.591C22.3858 100.591 0 78.2051 0 50.5908C0 22.9766 22.3858 0.59082 50 0.59082C77.6142 0.59082 100 22.9766 100 50.5908ZM9.08144 50.5908C9.08144 73.1895 27.4013 91.5094 50 91.5094C72.5987 91.5094 90.9186 73.1895 90.9186 50.5908C90.9186 27.9921 72.5987 9.67226 50 9.67226C27.4013 9.67226 9.08144 27.9921 9.08144 50.5908Z" fill="#E5E7EB"/>
                  <path d="M93.9676 39.0409C96.393 38.4038 97.8624 35.9116 97.0079 33.5539C95.2932 28.8227 92.871 24.3692 89.8167 20.348C85.8452 15.1192 80.8826 10.7238 75.2124 7.41289C69.5422 4.10194 63.2754 1.94025 56.7698 1.05124C51.7666 0.367541 46.6976 0.446843 41.7345 1.27873C39.2613 1.69328 37.813 4.19778 38.4501 6.62326C39.0873 9.04874 41.5694 10.4717 44.0505 10.1071C47.8511 9.54855 51.7191 9.52689 55.5402 10.0491C60.8642 10.7766 65.9928 12.5457 70.6331 15.2552C75.2735 17.9648 79.3347 21.5619 82.5849 25.841C84.9175 28.9121 86.7997 32.2913 88.1811 35.8758C89.083 38.2158 91.5421 39.6781 93.9676 39.0409Z" fill="currentColor"/>
                </svg>
                <svg v-if="!isWorking" xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" viewBox="0 0 20 20" fill="currentColor">
                  <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zM8 7a1 1 0 00-1 1v4a1 1 0 001 1h4a1 1 0 001-1V8a1 1 0 00-1-1H8z" clip-rule="evenodd" />
                </svg>
              </button>
            </div> 
        </div>
      </div>
    </div>

  </nav>

  <header class="bg-white shadow">
    <div class="mx-auto py-3 px-4 sm:px-6">
      <h1 class="text-3xl font-bold text-gray-900">Dashboard</h1>
    </div>
  </header>
</template>

<script lang="ts">
import WsClient from '@/util/WsClient';
import { defineComponent } from 'vue';
import { useI18n } from 'vue-i18n'
import { useStore } from 'vuex';

export default defineComponent({
  name: 'App',
  inject: ['wsClient'],
  computed: {
    wsConnected (): boolean {
      return this.store.state.ws.open;
    },
    info (): Record<string, unknown> {
      return this.store.state.ws.info;
    },
    status (): string {
      return this.store.state.ws.status;
    },
    option (): string {
      return this.store.state.option;
    }
  },
  watch: {
    status(value, oldValue) {
     this.isWorking = false;
    }
  },
  methods: {
    getOptionClass (option: string): string {
      return option === this.option ? 'bg-gray-900 text-white' : 'text-gray-300 hover:bg-gray-700 hover:text-white';
    },
    send(topic: string) {
      this.isWorking = true;
      this.wsClient.send(topic);
    },
    selectOption(path: string) {
      this.store.commit('option', path);
    }
  },
  data() {
    return {
      wsClient: this.wsClient as unknown as WsClient,
      isWorking: false,
    }
  },
  setup() {
    const store = useStore();
    const { t, locale } = useI18n({
      useScope: 'global'
    })
    return {
      store,
      t,
      locale,
    };
  },
  mounted() {
    //
  }
});
</script>