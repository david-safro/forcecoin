<!-- src/routes/dashboard/components/ActionButtons.svelte -->
<script>
    import { ShoppingCart, DollarSign, Pickaxe, Loader2, CheckCircle, AlertCircle } from 'lucide-svelte';

    let showMessage = '';
    let messageType = '';
    let isLoading = false;
    let loadingAction = '';

    async function callBlockchainAPI(action, params = {}) {
        const response = await fetch('/api/blockchain', {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ action, ...params })
        });
        return await response.json();
    }

    function showNotification(message, type = 'info') {
        showMessage = message;
        messageType = type;
        setTimeout(() => {
            showMessage = '';
            messageType = '';
        }, 5000);
    }

    async function handleMine() {
        if (isLoading) return;

        isLoading = true;
        loadingAction = 'mine';
        showNotification('Initializing mining process...', 'info');

        try {
            // Start node
            await callBlockchainAPI('start_node');

            // Create wallet if needed
            await callBlockchainAPI('create_wallet');

            showNotification('Mining block... This may take a few minutes.', 'info');

            // Mine block
            const result = await callBlockchainAPI('mine');

            if (result.success) {
                showNotification('Block mined successfully! 🎉 Your balance has been updated.', 'success');
                window.dispatchEvent(new CustomEvent('refresh-balance'));
            } else {
                showNotification(`Mining failed: ${result.error}`, 'error');
            }
        } catch (error) {
            console.error('Mining error:', error);
            showNotification('Mining failed. Please ensure the blockchain node is running.', 'error');
        } finally {
            isLoading = false;
            loadingAction = '';
        }
    }

    async function handleBuy() {
        if (isLoading) return;

        isLoading = true;
        loadingAction = 'buy';

        // Simulate loading for buy feature
        setTimeout(() => {
            showNotification('Buy feature coming soon! For now, try mining to get ForceCoin.', 'info');
            isLoading = false;
            loadingAction = '';
        }, 1500);
    }

    async function handleSell() {
        if (isLoading) return;

        isLoading = true;
        loadingAction = 'sell';

        // Simulate loading for sell feature
        setTimeout(() => {
            showNotification('Sell feature coming soon! You can transfer coins to other users.', 'info');
            isLoading = false;
            loadingAction = '';
        }, 1500);
    }
</script>

<div class="action-section">
    <div class="section-header">
        <h3 class="section-title">Quick Actions</h3>
        <p class="section-subtitle">Manage your ForceCoin portfolio</p>
    </div>

    <div class="action-grid">
        <!-- Buy Button -->
        <button
                class="action-button buy-button card-hover"
                on:click={handleBuy}
                disabled={isLoading}
        >
            <div class="button-icon buy-icon">
                {#if isLoading && loadingAction === 'buy'}
                    <Loader2 size="24" class="animate-spin" />
                {:else}
                    <ShoppingCart size="24" />
                {/if}
            </div>
            <div class="button-content">
                <div class="button-title">Buy FCN</div>
                <div class="button-subtitle">
                    {#if isLoading && loadingAction === 'buy'}
                        Loading...
                    {:else}
                        Purchase coins
                    {/if}
                </div>
            </div>
            <div class="button-badge coming-soon">Soon</div>
        </button>

        <!-- Sell Button -->
        <button
                class="action-button sell-button card-hover"
                on:click={handleSell}
                disabled={isLoading}
        >
            <div class="button-icon sell-icon">
                {#if isLoading && loadingAction === 'sell'}
                    <Loader2 size="24" class="animate-spin" />
                {:else}
                    <DollarSign size="24" />
                {/if}
            </div>
            <div class="button-content">
                <div class="button-title">Sell FCN</div>
                <div class="button-subtitle">
                    {#if isLoading && loadingAction === 'sell'}
                        Loading...
                    {:else}
                        Convert to cash
                    {/if}
                </div>
            </div>
            <div class="button-badge coming-soon">Soon</div>
        </button>

        <!-- Mine Button -->
        <button
                class="action-button mine-button card-hover"
                on:click={handleMine}
                disabled={isLoading}
        >
            <div class="button-icon mine-icon">
                {#if isLoading && loadingAction === 'mine'}
                    <Loader2 size="24" class="animate-spin" />
                {:else}
                    <Pickaxe size="24" />
                {/if}
            </div>
            <div class="button-content">
                <div class="button-title">Mine FCN</div>
                <div class="button-subtitle">
                    {#if isLoading && loadingAction === 'mine'}
                        Mining...
                    {:else}
                        Earn rewards
                    {/if}
                </div>
            </div>
            <div class="button-badge active">Active</div>
        </button>
    </div>

    <!-- Notification -->
    {#if showMessage}
        <div class="notification card animate-fade-in" class:success={messageType === 'success'} class:error={messageType === 'error'} class:info={messageType === 'info'}>
            <div class="notification-icon">
                {#if messageType === 'success'}
                    <CheckCircle size="20" />
                {:else if messageType === 'error'}
                    <AlertCircle size="20" />
                {:else}
                    <AlertCircle size="20" />
                {/if}
            </div>
            <div class="notification-content">
                <p>{showMessage}</p>
            </div>
        </div>
    {/if}
</div>

<style>
    .action-section {
        margin-top: 1.5rem;
    }

    .section-header {
        text-align: center;
        margin-bottom: 2rem;
    }

    .section-title {
        font-size: 1.5rem;
        font-weight: 700;
        color: var(--text-primary);
        margin: 0 0 0.5rem 0;
    }

    .section-subtitle {
        font-size: 0.9375rem;
        color: var(--text-secondary);
        margin: 0;
    }

    .action-grid {
        display: grid;
        grid-template-columns: repeat(auto-fit, minmax(280px, 1fr));
        gap: 1.5rem;
        margin-bottom: 2rem;
    }

    .action-button {
        position: relative;
        display: flex;
        align-items: center;
        gap: 1rem;
        padding: 1.5rem;
        background: var(--bg-glass);
        backdrop-filter: var(--blur-md);
        -webkit-backdrop-filter: var(--blur-md);
        border: 1px solid var(--border-color);
        border-radius: var(--radius-2xl);
        cursor: pointer;
        transition: var(--transition-normal);
        text-align: left;
        width: 100%;
        box-shadow: var(--shadow-sm);
    }

    .action-button:hover:not(:disabled) {
        transform: translateY(-4px);
        box-shadow: var(--shadow-xl);
        border-color: var(--border-color-hover);
    }

    .action-button:disabled {
        opacity: 0.7;
        cursor: not-allowed;
        transform: none !important;
    }

    .button-icon {
        width: 3.5rem;
        height: 3.5rem;
        border-radius: var(--radius-xl);
        display: flex;
        align-items: center;
        justify-content: center;
        flex-shrink: 0;
        transition: var(--transition-fast);
    }

    .buy-icon {
        background: linear-gradient(135deg, #10b981, #059669);
        color: white;
    }

    .sell-icon {
        background: linear-gradient(135deg, #f59e0b, #d97706);
        color: white;
    }

    .mine-icon {
        background: linear-gradient(135deg, #3b82f6, #2563eb);
        color: white;
    }

    .button-content {
        flex: 1;
        min-width: 0;
    }

    .button-title {
        font-size: 1.125rem;
        font-weight: 600;
        color: var(--text-primary);
        margin-bottom: 0.25rem;
    }

    .button-subtitle {
        font-size: 0.875rem;
        color: var(--text-secondary);
    }

    .button-badge {
        position: absolute;
        top: 1rem;
        right: 1rem;
        padding: 0.25rem 0.75rem;
        border-radius: var(--radius-lg);
        font-size: 0.75rem;
        font-weight: 600;
        text-transform: uppercase;
        letter-spacing: 0.05em;
    }

    .button-badge.coming-soon {
        background: var(--accent-warning);
        color: var(--text-inverse);
    }

    .button-badge.active {
        background: var(--accent-success);
        color: var(--text-inverse);
    }

    .notification {
        display: flex;
        align-items: flex-start;
        gap: 1rem;
        padding: 1rem 1.5rem;
        border-radius: var(--radius-xl);
        border-left: 4px solid;
    }

    .notification.success {
        background: rgba(16, 185, 129, 0.1);
        border-left-color: var(--accent-success);
    }

    .notification.error {
        background: rgba(239, 68, 68, 0.1);
        border-left-color: var(--accent-error);
    }

    .notification.info {
        background: rgba(59, 130, 246, 0.1);
        border-left-color: var(--accent-primary);
    }

    .notification-icon {
        flex-shrink: 0;
        margin-top: 0.125rem;
    }

    .notification.success .notification-icon {
        color: var(--accent-success);
    }

    .notification.error .notification-icon {
        color: var(--accent-error);
    }

    .notification.info .notification-icon {
        color: var(--accent-primary);
    }

    .notification-content p {
        margin: 0;
        font-size: 0.9375rem;
        line-height: 1.5;
        color: var(--text-primary);
    }

    @media (max-width: 768px) {
        .action-grid {
            grid-template-columns: 1fr;
            gap: 1rem;
        }

        .action-button {
            padding: 1.25rem;
        }

        .button-icon {
            width: 3rem;
            height: 3rem;
        }

        .button-title {
            font-size: 1rem;
        }

        .button-subtitle {
            font-size: 0.8125rem;
        }

        .button-badge {
            position: static;
            align-self: flex-start;
            margin-left: auto;
        }

        .notification {
            padding: 1rem;
        }
    }

    @media (max-width: 480px) {
        .section-title {
            font-size: 1.25rem;
        }

        .action-button {
            flex-direction: column;
            text-align: center;
            gap: 1rem;
        }

        .button-badge {
            position: absolute;
            top: 1rem;
            right: 1rem;
        }
    }
</style>