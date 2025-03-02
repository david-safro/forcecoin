import { writable } from 'svelte/store';

export const user = writable(null);

export async function checkSession() {
    const res = await fetch('/api/check-session');
    const data = await res.json();
    user.set(data.loggedIn ? data.session : null);
}
