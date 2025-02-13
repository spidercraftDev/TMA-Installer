<template v-else-if="!loggedIn">
    <div v-if="store.loginState === 'start'">
        <span>You are not logged in please input your key below</span>
        
        <form class="row login-form" @submit.prevent="login">
            <input type="password" name="whitelistKey" placeholder="Your whitelist key" class="whitelist-key-input">
            <button>Login</button>
        </form>
    </div>
    <div v-else-if="store.loginState === 'loading'">
        <span>Checking key...</span>
    </div>
    <div v-else>
        <span>{{ store.loginState }}</span>
        
        <footer>
            <button @click="store.loginState = 'start'">
                {{ strings.RETRY }}
            </button>
        </footer>
    </div>
</template>

<script setup lang="ts">
import fetch from '../fetch';
import { store } from '../store';
import strings from '../strings';

async function login(ev: Event) {
    store.loginState = 'loading';
    
    const formData = Object.fromEntries(new FormData(ev.target! as HTMLFormElement).entries());
    const trimValue = (formData.whitelistKey as string).trim()
    const resp = await fetch(`/checkKey?key=${trimValue}`);
    
    if (!resp.ok) {
        store.loginState = await resp.text();
        return;
    }
    
    store.jsonData = await resp.json();
    store.loggedIn = true;
    
    console.log(store);
    
}
</script>