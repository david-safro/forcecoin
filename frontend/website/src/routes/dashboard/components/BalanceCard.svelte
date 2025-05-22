<!-- src/routes/dashboard/components/BalanceCard.svelte -->
<script>
    import { onMount } from 'svelte';

    export let balance = 0.00;
    export let price = 1.00; // Default ForceCoin price in USD
    export let change = 0.0;

    let isLoading = true;
    let lastUpdated = '';

    async function callBlockchainAPI(action, params = {}) {
        const response = await fetch('/api/blockchain', {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ action, ...params })
        });
        return await response.json();
    }

    async function fetchBalance() {
        isLoading = true;
        try {
            // Ensure wallet exists
            await callBlockchainAPI('create_wallet');

            // Get balance
            const result = await callBlockchainAPI('get_balance');
            if (result.success) {
                balance = result.balance;
                lastUpdated = new Date().toLocaleTimeString();
            }
        } catch (error) {
            console.error('Error fetching balance:', error);
        } finally {
            isLoading = false;
        }
    }

    async function refreshBalance() {
        await fetchBalance();
    }

    // Listen for balance refresh events (triggered after mining)
    onMount(() => {
        fetchBalance();

        // Refresh balance every 30 seconds
        const interval = setInterval(fetchBalance, 30000);

        // Listen for custom refresh events
        window.addEventListener('refresh-balance', refreshBalance);

        return () => {
            clearInterval(interval);
            window.removeEventListener('refresh-balance', refreshBalance);
        };
    });
</script>

<div class="balance-card">
    <div class="balance-header">
        <h1>Your Balance</h1>
        <button class="refresh-btn" on:click={refreshBalance} disabled={isLoading}>
            <span class="refresh-icon" class:spinning={isLoading}>🔄</span>
        </button>
    </div>

    <div class="balance-main">
        {#if isLoading}
            <div class="loading">
                <div class="spinner"></div>
                <p>Loading balance...</p>
            </div>
        {:else}
            <div class="balance-top">
                <h2 class="balance-amount">{balance.toFixed(2)} FC</h2>
                <p class="usd-value">${(balance * price).toFixed(2)} USD</p>
            </div>

            <div class="price-info">
                <div class="price-row">
                    <span>1 FC = ${price.toFixed(2)}</span>
                    <span class={change >= 0 ? 'positive' : 'negative'}>
                        {change >= 0 ? '+' : ''}{change.toFixed(1)}% today
                    </span>
                </div>

                {#if lastUpdated}
                    <p class="last-updated">Last updated: {lastUpdated}</p>
                {/if}
            </div>
        {/if}
    </div>

    {#if balance === 0 && !isLoading}
        <div class="zero-balance-hint">
            <p>💡 <strong>Get started:</strong> Mine your first ForceCoin block to earn rewards!</p>
        </div>
    {/if}
</div>

<style>
    .balance-card {
        background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
        color: white;
        border-radius: 1rem;
        padding: 1.5rem;
        box-shadow: 0 8px 32px rgba(0,0,0,0.1);
        margin-bottom: 1rem;
        position: relative;
        overflow: hidden;
    }

    .balance-card::before {
        content: '';
        position: absolute;
        top: 0;
        left: 0;
        right: 0;
        bottom: 0;
        background: linear-gradient(45deg, rgba(255,255,255,0.1), transparent);
        pointer-events: none;
    }

    .balance-header {
        display: flex;
        justify-content: space-between;
        align-items: center;
        margin-bottom: 1rem;
    }

    .balance-header h1 {
        margin: 0;
        font-size: 1.25rem;
        font-weight: 600;
    }

    .refresh-btn {
        background: rgba(255,255,255,0.2);
        border: none;
        border-radius: 50%;
        width: 40px;
        height: 40px;
        cursor: pointer;
        display: flex;
        align-items: center;
        justify-content: center;
        transition: background-color 0.2s;
    }

    .refresh-btn:hover:not(:disabled) {
        background: rgba(255,255,255,0.3);
    }

    .refresh-btn:disabled {
        cursor: not-allowed;
        opacity: 0.6;
    }

    .refresh-icon {
        font-size: 1.2rem;
        transition: transform 0.3s;
    }

    .refresh-icon.spinning {
        animation: spin 1s linear infinite;
    }

    @keyframes spin {
        from { transform: rotate(0deg); }
        to { transform: rotate(360deg); }
    }

    .loading {
        text-align: center;
        padding: 2rem 0;
    }

    .spinner {
        border: 3px solid rgba(255,255,255,0.3);
        border-radius: 50%;
        border-top: 3px solid white;
        width: 40px;
        height: 40px;
        animation: spin 1s linear infinite;
        margin: 0 auto 1rem;
    }

    .balance-top {
        text-align: center;
        margin-bottom: 1rem;
    }

    .balance-amount {
        font-size: 2.5rem;
        font-weight: bold;
        margin: 0;
        text-shadow: 0 2px 4px rgba(0,0,0,0.3);
    }

    .usd-value {
        font-size: 1.1rem;
        opacity: 0.9;
        margin: 0.5rem 0 0 0;
    }

    .price-info {
        border-top: 1px solid rgba(255,255,255,0.2);
        padding-top: 1rem;
    }

    .price-row {
        display: flex;
        justify-content: space-between;
        align-items: center;
        font-size: 0.9rem;
    }

    .positive {
        color: #4ade80;
        font-weight: 600;
    }

    .negative {
        color: #f87171;
        font-weight: 600;
    }

    .last-updated {
        font-size: 0.8rem;
        opacity: 0.7;
        margin: 0.5rem 0 0 0;
        text-align: center;
    }

    .zero-balance-hint {
        background: rgba(255,255,255,0.1);
        border-radius: 0.5rem;
        padding: 1rem;
        margin-top: 1rem;
        text-align: center;
        border: 1px solid rgba(255,255,255,0.2);
    }

    .zero-balance-hint p {
        margin: 0;
        font-size: 0.9rem;
    }

    @media (max-width: 480px) {
        .balance-amount {
            font-size: 2rem;
        }

        .price-row {
            flex-direction: column;
            gap: 0.5rem;
            align-items: center;
        }
    }
</style>