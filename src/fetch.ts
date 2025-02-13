const BASE_URL = 'https://toomanyaccounts.org';

export default (path: string, options?: RequestInit) => {
    const resp = fetch(`${BASE_URL}${path}`, {
        cache: 'no-cache',
        ...options
    });
    
    return resp;
}