<!-- src/routes/+layout.svelte -->
<script>
    import { page } from '$app/stores';
    import { onMount } from 'svelte';
    import { Home, Send, Settings, Sun, Moon, User, LogOut } from 'lucide-svelte';
    import { theme } from '$lib/stores/theme.js';
    import '../global.css';

    export let data;

    let showUserMenu = false;

    onMount(() => {
        // Initialize theme on mount
        if (typeof document !== 'undefined') {
            document.documentElement.setAttribute('data-theme', $theme);
        }
    });

    // Update theme attribute when theme changes
    $: if (typeof document !== 'undefined') {
        document.documentElement.setAttribute('data-theme', $theme);
    }

    async function logout() {
        await fetch('/api/logout', { method: 'POST' });
        window.location.href = '/';
    }

    function toggleUserMenu() {
        showUserMenu = !showUserMenu;
    }

    function closeUserMenu() {
        showUserMenu = false;
    }

    // Close user menu when clicking outside
    function handleClickOutside(event) {
        if (!event.target.closest('.user-menu-container')) {
            showUserMenu = false;
        }
    }

    onMount(() => {
        document.addEventListener('click', handleClickOutside);
        return () => document.removeEventListener('click', handleClickOutside);
    });
</script>

<svelte:head>
    <title>ForceCoin - Professional Cryptocurrency Platform</title>
    <meta name="description" content="Modern cryptocurrency platform with advanced blockchain technology" />
</svelte:head>

<div class="app-container">
    <!-- Top Navigation -->
    {#if data?.user}
        <nav class="top-nav glass">
            <div class="nav-content">
                <div class="nav-brand">
                    <h1 class="brand-text">ForceCoin</h1>
                    <span class="brand-badge">PRO</span>
                </div>

                <div class="nav-actions">
                    <!-- Theme Toggle -->
                    <button class="theme-toggle btn-glass" on:click={theme.toggle} aria-label="Toggle theme">
                        {#if $theme === 'light'}
                            <Moon size="18" />
                        {:else}
                            <Sun size="18" />
                        {/if}
                    </button>

                    <!-- User Menu -->
                    <div class="user-menu-container">
                        <button class="user-button btn-glass" on:click={toggleUserMenu}>
                            <User size="18" />
                            <span class="user-name">{data.user.name}</span>
                        </button>

                        {#if showUserMenu}
                            <div class="user-menu card animate-fade-in">
                                <div class="user-info">
                                    <div class="user-avatar">
                                        <User size="20" />
                                    </div>
                                    <div class="user-details">
                                        <div class="user-name-large">{data.user.name}</div>
                                        <div class="user-email">{data.user.email}</div>
                                    </div>
                                </div>
                                <hr class="menu-divider" />
                                <button class="menu-item" on:click={logout}>
                                    <LogOut size="16" />
                                    <span>Sign Out</span>
                                </button>
                            </div>
                        {/if}
                    </div>
                </div>
            </div>
        </nav>
    {:else}
        <!-- Guest Navigation -->
        <nav class="top-nav glass">
            <div class="nav-content">
                <div class="nav-brand">
                    <h1 class="brand-text">ForceCoin</h1>
                    <span class="brand-badge">PRO</span>
                </div>

                <div class="nav-actions">
                    <button class="theme-toggle btn-glass" on:click={theme.toggle} aria-label="Toggle theme">
                        {#if $theme === 'light'}
                            <Moon size="18" />
                        {:else}
                            <Sun size="18" />
                        {/if}
                    </button>
                </div>
            </div>
        </nav>
    {/if}

    <!-- Main Content -->
    <main class="main-content">
        <slot />
    </main>

    <!-- Bottom Navigation (only for authenticated users) -->
    {#if data?.user}
        <nav class="bottom-nav glass">
            <a
                    href="/dashboard"
                    class="nav-item"
                    class:active={$page.url.pathname === '/dashboard'}
                    on:click={closeUserMenu}
            >
                <Home size="20" />
                <span>Dashboard</span>
            </a>
            <a
                    href="/transfer"
                    class="nav-item"
                    class:active={$page.url.pathname === '/transfer'}
                    on:click={closeUserMenu}
            >
                <Send size="20" />
                <span>Transfer</span>
            </a>
            <a
                    href="/settings"
                    class="nav-item"
                    class:active={$page.url.pathname === '/settings'}
                    on:click={closeUserMenu}
            >
                <Settings size="20" />
                <span>Settings</span>
            </a>
        </nav>
    {/if}
</div>

<style>
    .app-container {
        display: flex;
        flex-direction: column;
        min-height: 100vh;
        background:
                radial-gradient(ellipse at top left, rgba(59, 130, 246, 0.15) 0%, transparent 50%),
                radial-gradient(ellipse at bottom right, rgba(139, 92, 246, 0.15) 0%, transparent 50%),
                var(--bg-primary);
        background-attachment: fixed;
    }

    /* Top Navigation */
    .top-nav {
        position: sticky;
        top: 0;
        z-index: 50;
        padding: 1rem 0;
        margin: 1rem 1rem 0 1rem;
        border-radius: var(--radius-2xl);
    }

    .nav-content {
        display: flex;
        align-items: center;
        justify-content: space-between;
        max-width: 1200px;
        margin: 0 auto;
        padding: 0 1.5rem;
    }

    .nav-brand {
        display: flex;
        align-items: center;
        gap: 0.75rem;
    }

    .brand-text {
        font-size: 1.5rem;
        font-weight: 700;
        background: var(--bg-gradient);
        -webkit-background-clip: text;
        -webkit-text-fill-color: transparent;
        background-clip: text;
        margin: 0;
    }

    .brand-badge {
        background: var(--bg-gradient-alt);
        color: var(--text-inverse);
        font-size: 0.625rem;
        font-weight: 600;
        padding: 0.25rem 0.5rem;
        border-radius: var(--radius-sm);
        letter-spacing: 0.05em;
    }

    .nav-actions {
        display: flex;
        align-items: center;
        gap: 1rem;
    }

    .theme-toggle {
        width: 2.5rem;
        height: 2.5rem;
        border-radius: 50%;
        display: flex;
        align-items: center;
        justify-content: center;
    }

    /* User Menu */
    .user-menu-container {
        position: relative;
    }

    .user-button {
        gap: 0.5rem;
        padding: 0.5rem 1rem;
        border-radius: var(--radius-xl);
    }

    .user-name {
        font-weight: 500;
        max-width: 120px;
        overflow: hidden;
        text-overflow: ellipsis;
        white-space: nowrap;
    }

    .user-menu {
        position: absolute;
        top: calc(100% + 0.5rem);
        right: 0;
        min-width: 280px;
        padding: 1.5rem;
        z-index: 60;
    }

    .user-info {
        display: flex;
        align-items: center;
        gap: 1rem;
        margin-bottom: 1rem;
    }

    .user-avatar {
        width: 3rem;
        height: 3rem;
        background: var(--bg-gradient);
        border-radius: 50%;
        display: flex;
        align-items: center;
        justify-content: center;
        color: var(--text-inverse);
    }

    .user-details {
        flex: 1;
    }

    .user-name-large {
        font-weight: 600;
        color: var(--text-primary);
        margin-bottom: 0.25rem;
    }

    .user-email {
        font-size: 0.8125rem;
        color: var(--text-secondary);
    }

    .menu-divider {
        border: none;
        height: 1px;
        background: var(--border-color);
        margin: 1rem 0;
    }

    .menu-item {
        width: 100%;
        display: flex;
        align-items: center;
        gap: 0.75rem;
        padding: 0.75rem;
        background: transparent;
        border: none;
        border-radius: var(--radius-lg);
        color: var(--text-primary);
        cursor: pointer;
        transition: var(--transition-fast);
        font-size: 0.875rem;
    }

    .menu-item:hover {
        background: var(--bg-glass-hover);
    }

    /* Main Content */
    .main-content {
        flex: 1;
        padding: 2rem 1rem;
        max-width: 1200px;
        margin: 0 auto;
        width: 100%;
    }

    /* Bottom Navigation */
    .bottom-nav {
        position: sticky;
        bottom: 1rem;
        margin: 0 1rem 1rem 1rem;
        padding: 1rem;
        border-radius: var(--radius-2xl);
        display: flex;
        justify-content: center;
        gap: 2rem;
    }

    .nav-item {
        display: flex;
        flex-direction: column;
        align-items: center;
        gap: 0.5rem;
        padding: 0.75rem 1.5rem;
        text-decoration: none;
        color: var(--text-secondary);
        border-radius: var(--radius-xl);
        transition: var(--transition-fast);
        font-size: 0.8125rem;
        font-weight: 500;
        min-width: 80px;
    }

    .nav-item:hover {
        color: var(--text-primary);
        background: var(--bg-glass-hover);
        transform: translateY(-2px);
    }

    .nav-item.active {
        color: var(--accent-primary);
        background: var(--bg-glass);
        box-shadow: var(--shadow-md);
    }

    /* Responsive Design */
    @media (max-width: 768px) {
        .nav-content {
            padding: 0 1rem;
        }

        .user-name {
            display: none;
        }

        .user-menu {
            min-width: 240px;
            right: -1rem;
        }

        .bottom-nav {
            gap: 1rem;
        }

        .nav-item {
            padding: 0.5rem 1rem;
            min-width: 70px;
        }

        .main-content {
            padding: 1rem 0.5rem;
        }

        .top-nav,
        .bottom-nav {
            margin: 0.5rem;
        }
    }

    @media (max-width: 480px) {
        .brand-text {
            font-size: 1.25rem;
        }

        .nav-item span {
            font-size: 0.75rem;
        }

        .bottom-nav {
            gap: 0.5rem;
        }

        .nav-item {
            padding: 0.5rem 0.75rem;
            min-width: 60px;
        }
    }
</style>