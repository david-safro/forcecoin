<script>
    const navItems = [
        { name: 'Dashboard', href: '/dashboard' },
        { name: 'Insights', href: '/insights' },
        { name: 'Transfer', href: '/transfer' },
        { name: 'Settings', href: '/settings' }
    ];
</script>

<nav class="bottom-nav">
    {#each navItems as item}
        <a href={item.href}>{item.name}</a>
    {/each}
</nav>

<style>
    .bottom-nav {
        position: fixed;
        bottom: 0;
        left: 0;
        right: 0;
        background: #ffffff;
        border-top: 1px solid #ddd;
        display: flex;
        justify-content: space-around;
        padding: 0.75rem 0;
        z-index: 10;
    }

    a {
        color: #0056d2;
        text-decoration: none;
        font-weight: 600;
        font-size: 0.95rem;
    }

    a:hover {
        text-decoration: underline;
    }
</style>
