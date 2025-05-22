<!-- src/routes/dashboard/components/ActionButtons.svelte -->
<script>
    let showMessage = '';
    let isLoading = false;
    let showMineDialog = false;

    async function callBlockchainAPI(action, params = {}) {
        const response = await fetch('/api/blockchain', {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ action, ...params })
        });
        return await response.json();
    }

    async function handleMine() {
        if (isLoading) return;

        isLoading = true;
        showMessage = 'Mining block...';

        try {
            // First ensure node is running
            await callBlockchainAPI('start_node');

            // Create wallet if needed
            await callBlockchainAPI('create_wallet');

            // Mine a block
            const result = await callBlockchainAPI('mine');

            if (result.success) {
                showMessage = 'Block mined successfully! 🎉';
                // Refresh balance after mining
                window.dispatchEvent(new CustomEvent('refresh-balance'));
            } else {
                showMessage = `Mining failed: ${result.error}`;
            }
        } catch (error) {
            console.error('Mining error:', error);
            showMessage = 'Mining failed. Make sure the blockchain node is running.';
        } finally {
            isLoading = false;
            setTimeout(() => showMessage = '', 5000);
        }
    }

    async function handleBuy() {
        showMessage = 'Buy feature coming soon! For now, try mining to get ForceCoin.';
        setTimeout(() => showMessage = '', 3000);
    }

    async function handleSell() {
        showMessage = 'Sell feature coming soon! You can transfer coins to other users.';
        setTimeout(() => showMessage = '', 3000);
    }
</script>

<div class="action-buttons">
    <button on:click={handleBuy} disabled={isLoading}>
        💰 Buy
    </button>
    <button on:click={handleSell} disabled={isLoading}>
        💸 Sell
    </button>
    <button on:click={handleMine} disabled={isLoading}>
        {isLoading ? '⛏️ Mining...' : '⛏️ Mine'}
    </button>

    {#if showMessage}
        <div class="popup" class:success={showMessage.includes('successfully')} class:error={showMessage.includes('failed')}>
            {showMessage}
        </div>
    {/if}
</div>

<style>
    .action-buttons {
        display: flex;
        justify-content: center;
        gap: 1rem;
        margin-top: 1.5rem;
        position: relative;
        flex-wrap: wrap;
    }

    button {
        background-color: #0066cc;
        color: white;
        padding: 0.75rem 1.5rem;
        border: none;
        border-radius: 8px;
        font-weight: bold;
        cursor: pointer;
        box-shadow: 0 2px 4px rgba(0, 0, 0, 0.2);
        transition: transform 0.1s ease, box-shadow 0.2s;
        min-width: 100px;
    }

    button:hover:not(:disabled) {
        transform: translateY(-2px);
        box-shadow: 0 4px 8px rgba(0, 0, 0, 0.2);
        background-color: #004d99;
    }

    button:disabled {
        background-color: #cccccc;
        cursor: not-allowed;
        transform: none;
    }

    .popup {
        position: absolute;
        top: -3rem;
        left: 50%;
        transform: translateX(-50%);
        background-color: #f0f0f0;
        color: #333;
        padding: 0.75rem 1rem;
        border-radius: 6px;
        box-shadow: 0 2px 6px rgba(0,0,0,0.15);
        animation: fade 0.3s ease;
        max-width: 300px;
        text-align: center;
        font-size: 0.9rem;
        z-index: 1000;
    }

    .popup.success {
        background-color: #d4edda;
        color: #155724;
        border: 1px solid #c3e6cb;
    }

    .popup.error {
        background-color: #f8d7da;
        color: #721c24;
        border: 1px solid #f5c6cb;
    }

    @keyframes fade {
        from { opacity: 0; transform: translateX(-50%) translateY(-5px); }
        to { opacity: 1; transform: translateX(-50%) translateY(0); }
    }

    @media (max-width: 480px) {
        .action-buttons {
            flex-direction: column;
            align-items: center;
        }

        button {
            width: 200px;
        }
    }
</style>