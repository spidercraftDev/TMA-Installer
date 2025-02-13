<template>
  <div class="container">
    <div v-if="loading">
      <span class="loader"></span>
      <br>
      <span>{{ strings.LOADING }}</span>
    </div>
    
    <div v-else>
      <span class="tma-installer-label">TMA installer v{{ tmaInstallerVersion }}</span>
      <h1>{{ strings.TMA_VERSION(tmaVersion) }}</h1>
      
      <Update v-if="hasUpdater" />
      <Login v-else-if="!store.loggedIn" />
      <Install v-else />
    </div>
  </div>
</template>

<script setup async lang="ts">
import { onMounted, ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { getVersion } from "@tauri-apps/api/app"
import { check } from "@tauri-apps/plugin-updater";

import strings from "./strings";
import { store } from "./store";

import Login from "./components/Login.vue";
import Update from "./components/Update.vue";
import Install from "./components/Install.vue";
import fetch from "./fetch";

const tmaInstallerVersion = ref();
const tmaVersion = ref();

const loading = ref(true);
const hasUpdater = ref<boolean>();

onMounted(async () => {
  const updateRes = await check();

  if (updateRes) {
    hasUpdater.value = true;
    (window as any).updater = updateRes; // can't call updater from store so we have to store it globally
  }
  
  const resp = await fetch(`/updater/latest.json?v=${tmaInstallerVersion.value}`);
  
  tmaInstallerVersion.value = await getVersion();
  tmaVersion.value = (await resp.json()).tmaVersion;
  loading.value = false;
  
  try {
    store.gamePath = await invoke('get_game_location') as string;
    console.log('Found path', store.gamePath);    
  } catch (error) {
    console.log(error);
    console.log('Failed to find game location');
  }
});
</script>

<style>
body {
  margin: 0;
  padding: 0;
}

.row {
  display: flex;
  justify-content: center;
  gap: 0.5em;
}

.flex-col {
  display: flex;
  gap: 0.5em;
  flex-direction: column;
}

h1 {
  text-align: center;
}

.game-dir {
  background: #0f0f0f98;
  padding: 0.25em;
  border-radius: 0.3em;
}

footer {
  margin-top: 1em;
}

.login-form {
  margin-top: 1em;
  width: 90vw;
}

.whitelist-key-input {
  flex-grow: 0.5;
  text-align: center;
}

.tma-installer-label {
  position: fixed;
  right: 0.5em;
  bottom: 0px;
  font-size: 0.75em;
  opacity: 0.4;
  user-select: none;
  letter-spacing: 0.05em;
}
</style>