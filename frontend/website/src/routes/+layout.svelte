<script>
    import { page } from '$app/stores';

    export let data;

    async function logout() {
        await fetch('/api/logout', { method: 'POST' });
        window.location.reload();
    }
</script>

<header>
    {#if data.user}
        <p>Welcome, {data.user.email}</p>
        <button on:click={logout}>Logout</button>
    {:else}

        {#if $page.url.pathname !== '/login' && $page.url.pathname !== '/register'}
            <a href="/login">Login</a>
        {/if}
    {/if}
</header>

<slot />

