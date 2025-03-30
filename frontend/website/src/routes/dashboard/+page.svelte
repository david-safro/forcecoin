<script>
    import BalanceCard from './components/BalanceCard.svelte';
    import ActionButtons from './components/ActionButtons.svelte';
    import { Home, Send, Settings as Gear } from 'lucide-svelte';
    import { page } from '$app/stores';
</script>

<main>
    <BalanceCard />
    <ActionButtons />
</main>

<nav class="bottom-nav">
    <a href="/dashboard" class:selected={$page.url.pathname === '/dashboard'}>
        <Home size="20" /> Dashboard
    </a>
    <a href="/transfer" class:selected={$page.url.pathname === '/transfer'}>
        <Send size="20" /> Transfer
    </a>
    <a href="/settings" class:selected={$page.url.pathname === '/settings'}>
        <Gear size="20" /> Settings
    </a>
</nav>

<style>
    main {
        flex: 1;
        padding: 1rem;
        padding-bottom: 6rem;
    }

    .bottom-nav {
        position: fixed;
        bottom: 0;
        width: 100%;
        background-color: white;
        display: flex;
        justify-content: space-around;
        padding: 0.5rem 0;
        border-top: 1px solid #ccc;
    }

    .bottom-nav a {
        text-decoration: none;
        color: #0066cc;
        font-weight: bold;
        display: flex;
        flex-direction: column;
        align-items: center;
        font-size: 0.9rem;
    }

    .bottom-nav a.selected {
        color: black;
        border-bottom: 2px solid #0066cc;
    }
</style>
