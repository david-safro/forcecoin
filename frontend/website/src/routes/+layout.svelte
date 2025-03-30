<script>
    import { page } from '$app/stores';
    import { Home, Send, Settings } from 'lucide-svelte';

    export let data;

    async function logout() {
        await fetch('/api/logout', { method: 'POST' });
        window.location.href = '/';
    }
</script>

<div class="app-container">
    {#if data?.user}
        <div class="user-header">
            <p>Hello, {data.user.name}</p>
            <button on:click={logout}>Logout</button>
        </div>
    {/if}

    <main>
        <slot />
    </main>

    {#if data?.user}
        <nav class="bottom-nav">
            <a href="/dashboard" class:selected={$page.url.pathname === '/dashboard'}>
                <Home size="18" /> Dashboard
            </a>
            <a href="/transfer" class:selected={$page.url.pathname === '/transfer'}>
                <Send size="18" /> Transfer
            </a>
            <a href="/settings" class:selected={$page.url.pathname === '/settings'}>
                <Settings size="18" /> Settings
            </a>
        </nav>
    {/if}
</div>

<style>
    .app-container {
        display: flex;
        flex-direction: column;
        min-height: 100vh;
        font-family: system-ui, sans-serif;
    }

    .user-header {
        background-color: #f8f9fa;
        padding: 0.75rem 1rem;
        display: flex;
        justify-content: space-between;
        align-items: center;
        font-weight: bold;
        border-bottom: 1px solid #ddd;
    }

    .user-header button {
        background-color: #d23b3b;
        color: white;
        border: none;
        padding: 0.4rem 0.8rem;
        border-radius: 0.4rem;
        cursor: pointer;
    }

    .user-header button:hover {
        background-color: #a52b2b;
    }

    main {
        flex: 1;
        padding: 1rem;
        padding-bottom: 5rem;
    }

    .bottom-nav {
        display: flex;
        justify-content: space-around;
        padding: 1rem;
        background-color: #f1f1f1;
        border-top: 1px solid #ccc;
        position: sticky;
        bottom: 0;
    }

    .bottom-nav a {
        text-decoration: none;
        color: #0066cc;
        font-weight: bold;
        display: flex;
        align-items: center;
        gap: 0.4rem;
    }

    .bottom-nav a.selected {
        color: black;
        border-bottom: 2px solid #0066cc;
        padding-bottom: 2px;
    }

    .bottom-nav a:hover {
        color: #004b99;
    }
</style>
