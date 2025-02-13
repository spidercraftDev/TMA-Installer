<template>
    <div v-if="store.installState === 'installing'">
        <span>{{ strings.INSTALLING }}</span>
    </div>
    
    <div v-else-if="store.installState === 'error'">
        <div class="col">
            <span v-for="text in strings.INSTALL_ERRORS">{{ text }}</span>
        </div>
        
        <span>Error reason: {{ store.installError }}</span>
        
        <footer>
            <button @click="store.installState = 'start'">{{ strings.RETRY }}</button>
        </footer>
    </div>
    
    <div v-else-if="store.installState === 'installed'" class="col">
        <div v-for="html in strings.INSTALL_SUCCESS">
            <span v-html="html"></span>
        </div>
        
        <footer class="row">
            <button @click="invoke('close')" danger>
                {{ strings.CLOSE }}
            </button>
        </footer>
    </div>
    
    <div v-else-if="store.installState === 'uninstalled'" class="col">
        <span>{{ strings.UNINSTALLED }}</span>
        <br>
        
        <footer class="row">
            <button @click="invoke('close')" danger>{{strings.CLOSE}}</button>
        </footer>
    </div>
    
    <div v-else>
        <div class="flex-col" v-if="store.exePath">
            <div>
                <div class="game-dir">{{store.exePath}}</div>
                {{ strings.PATH_SELECTED }}
            </div>
            
            <div class="row">
                <button @click="installTMA" v-if="store.exePath">
                    {{store.foundTmaInstallation ? strings.UPDATE : strings.INSTALL}}
                </button>
                
                <button v-if="store.foundTmaInstallation" @click="uninstallTMA" danger>
                    {{strings.UNINSTALL}}
                </button>
                
                <button @click="promptGameDir" v-if="!store.foundTmaInstallation">
                    {{ store.exePath ? strings.CHANGE_PATH : strings.SET_PATH }}
                </button>
            </div>
        </div>

        <div v-else class="flex-col">
            <span>
                {{ strings.CANT_FIND_GAME }}
            </span>
            
            <button @click="promptGameDir">
                {{ strings.SET_PATH }}
            </button>
        </div>
    </div>
</template>

<script setup lang="ts">
import { invoke } from '@tauri-apps/api/core';
import { store } from '../store';
import strings from '../strings';
import { open } from '@tauri-apps/plugin-dialog';

async function installTMA() {
    store.installState = 'installing';
    
    const jsonData = store.jsonData!;
    
    try {
        await invoke("install_bepinex", {
            downloadLink: jsonData.link,
            keyName: jsonData.files.key,
            nonceName: jsonData.files.nonce,
            folderName: jsonData.files.folder,
            dllName: jsonData.files.dll
        });
        
        store.installState = 'installed';    
    } catch (error: any) {
        store.installError = error.toString();
        store.installState = 'error';
    }
}

async function uninstallTMA() {
    await invoke('uninstall', {folderName: store.jsonData!.files.folder});
    
    store.foundTmaInstallation = false;
    store.installState = 'uninstalled';
}

async function promptGameDir() {
    const pickedExe = await open({
        multiple: false,
        directory: false,
        defaultPath: store.gamePath,
        filters: [
        {
            name: 'Executable File',
            extensions: ['exe']
        }
        ],
        title: 'Gtag executable file'
    });
    
    const jsonData = store.jsonData!;
    
    if (!pickedExe) return;
    
    try {
        await invoke('set_info', {
            exePath: pickedExe,
            nonce: jsonData.nonce,
            whitelistKey: jsonData.key
        });
        
        store.exePath = pickedExe;
        store.foundTmaInstallation = await invoke('has_tma_installed', {folderName: jsonData.files.folder}).catch((err) => console.error(err)) as boolean;
    } catch (error: any) {
        console.log(error);
        
        store.installState = 'error';
        store.installError = error.toString();
    }
};
</script>