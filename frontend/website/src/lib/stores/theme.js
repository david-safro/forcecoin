// src/lib/stores/theme.js
import { writable } from 'svelte/store';
import { browser } from '$app/environment';

function createThemeStore() {
    // Initialize with system preference or default to dark
    const getInitialTheme = () => {
        if (!browser) return 'dark';

        const stored = localStorage.getItem('theme');
        if (stored) return stored;

        return window.matchMedia('(prefers-color-scheme: dark)').matches ? 'dark' : 'light';
    };

    const { subscribe, set, update } = writable(getInitialTheme());

    return {
        subscribe,
        toggle: () => update(theme => {
            const newTheme = theme === 'light' ? 'dark' : 'light';
            if (browser) {
                localStorage.setItem('theme', newTheme);
                document.documentElement.setAttribute('data-theme', newTheme);
            }
            return newTheme;
        }),
        set: (theme) => {
            if (browser) {
                localStorage.setItem('theme', theme);
                document.documentElement.setAttribute('data-theme', theme);
            }
            set(theme);
        }
    };
}

export const theme = createThemeStore();