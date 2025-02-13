<template v-else-if="outdated">
    <div class="flex-col">
        <span>{{ strings.OUTDATED }}</span>
        
        <div class="row">
            <button @click="finishUpdate">
                {{store.updating ? downloadProgress : strings.UPDATE}}
            </button>
        </div>
    </div>
</template>

<script setup lang="ts">
import { Update } from '@tauri-apps/plugin-updater';
import { store } from '../store';
import strings from '../strings';
import { computed, ref } from 'vue';

let contentLength = ref(0);
let downloaded = ref(0);

const downloadProgress = computed(() => {
    const progress = (downloaded.value / contentLength.value);
    const percent = Math.floor(progress * 100);

    return `${percent} %`;
});

async function finishUpdate() {
    if (store.updating) return;
    
    store.updating = true;

    const updater = (window as any).updater as Update;
 
    await updater.download((ev) => {
        if (ev.event === 'Started') {
            contentLength.value = ev.data.contentLength!;
        } else if (ev.event === 'Progress') {
            downloaded.value += ev.data.chunkLength!;
        } else if (ev.event === 'Finished') {
            console.log('install time!');
        }
    });

    updater.install();
}
</script>