import { reactive, Ref, ref } from "vue";

interface Store {
    loggedIn: boolean;
    loginState: 'start' | 'loading' | string;
    
    jsonData?: JSONData;

    updating: boolean;
    
    installState: 'start' | 'error' | 'installing' | 'installed' | 'uninstall' | 'uninstalled';
    installError: string;
    
    foundTmaInstallation: boolean;
    
    gamePath?: string;
    exePath?: string;
}

interface TMAFiles {
    key: string;
    nonce: string;
    folder: string;
    dll: string;
}

interface JSONData {
    link: string;
    files: TMAFiles;
    nonce: string;
    key: string;
}

export const store = reactive<Store>({
    loginState: 'start',
    loggedIn: false,
    updating: false,
    
    installState: 'start',
    installError: '',

    foundTmaInstallation: false
});