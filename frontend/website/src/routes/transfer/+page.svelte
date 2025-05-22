<!-- src/routes/transfer/+page.svelte -->
<script>
    import { onMount } from 'svelte';

    let recipient = '';
    let amount = '';
    let balance = 0;
    let isLoading = false;
    let message = '';
    let messageType = '';
    let recentTransactions = [];

    async function callBlockchainAPI(action, params = {}) {
        const response = await fetch('/api/blockchain', {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ action, ...params })
        });
        return await response.json();
    }

    async function fetchBalance() {
        try {
            const result = await callBlockchainAPI('get_balance');
            if (result.success) {
                balance = result.balance;
            }
        } catch (error) {
            console.error('Error fetching balance:', error);
        }
    }

    async function handleTransfer() {
        if (isLoading) return;

        // Validation
        if (!recipient.trim()) {
            showMessage('Please enter a recipient address', 'error');
            return;
        }

        if (!amount || parseFloat(amount) <= 0) {
            showMessage('Please enter a valid amount', 'error');
            return;
        }

        if (parseFloat(amount) > balance) {
            showMessage('Insufficient balance', 'error');
            return;
        }

        isLoading = true;
        showMessage('Sending transaction...', 'info');

        try {
            const result = await callBlockchainAPI('send_transaction', {
                recipient: recipient.trim(),
                amount: parseFloat(amount)
            });

            if (result.success) {
                showMessage('Transaction sent successfully! 🎉', 'success');

                // Add to recent transactions
                recentTransactions = [{
                    to: recipient.trim(),
                    amount: parseFloat(amount),
                    timestamp: new Date().toLocaleString(),
                    status: 'Sent'
                }, ...recentTransactions.slice(0, 4)];

                // Clear form
                recipient = '';
                amount = '';

                // Refresh balance
                await fetchBalance();
            } else {
                showMessage(`Transaction failed: ${result.error}`, 'error');
            }
        } catch (error) {
            console.error('Transfer error:', error);
            showMessage('Transfer failed. Please ensure the blockchain node is running.', 'error');
        } finally {
            isLoading = false;
        }
    }

    function showMessage(text, type) {
        message = text;
        messageType = type;
        setTimeout(() => {
            message = '';
            messageType = '';
        }, 5000);
    }

    function setMaxAmount() {
        amount = balance.toString();
    }

    onMount(() => {
        fetchBalance();
    });
</script>

<main>
    <div class="transfer-container">
        <header class="page-header">
            <h1>💸 Transfer ForceCoin</h1>
            <p>Send FCN to other wallet addresses</p>
        </header>

        <div class="balance-display">
            <h3>Available Balance</h3>
            <div class="balance-amount">
                {balance.toFixed(2)} FC
                <button class="max-btn" on:click={setMaxAmount} disabled={balance === 0}>
                    MAX
                </button>
            </div>
        </div>

        <form on:submit|preventDefault={handleTransfer} class="transfer-form">
            <div class="form-group">
                <label for="recipient">Recipient Address</label>
                <input
                        id="recipient"
                        type="text"
                        bind:value={recipient}
                        placeholder="Enter wallet address or user ID (e.g., user_2)"
                        disabled={isLoading}
                        required
                />
                <small>💡 Tip: Use format "user_X" where X is the recipient's user ID</small>
            </div>

            <div class="form-group">
                <label for="amount">Amount (FC)</label>
                <input
                        id="amount"
                        type="number"
                        step="0.01"
                        min="0.01"
                        max={balance}
                        bind:value={amount}
                        placeholder="0.00"
                        disabled={isLoading}
                        required
                />
            </div>

            {#if message}
                <div class="message" class:success={messageType === 'success'} class:error={messageType === 'error'} class:info={messageType === 'info'}>
                    {message}
                </div>
            {/if}

            <button
                    type="submit"
                    class="transfer-btn"
                    disabled={isLoading || balance === 0}
            >
                {#if isLoading}
                    <span class="spinner"></span>
                    Sending...
                {:else}
                    Send ForceCoin
                {/if}
            </button>
        </form>

        {#if balance === 0}
            <div class="zero-balance-hint">
                <p>💡 <strong>No balance:</strong> Mine some ForceCoin first to have funds to transfer!</p>
                <a href="/dashboard" class="mine-link">Go Mine →</a>
            </div>
        {/if}

        {#if recentTransactions.length > 0}
            <div class="recent-transactions">
                <h3>Recent Transfers</h3>
                <div class="transaction-list">
                    {#each recentTransactions as tx}
                        <div class="transaction-item">
                            <div class="tx-info">
                                <div class="tx-amount">-{tx.amount} FC</div>
                                <div class="tx-details">
                                    <div class="tx-to">To: {tx.to}</div>
                                    <div class="tx-time">{tx.timestamp}</div>
                                </div>
                            </div>
                            <div class="tx-status">{tx.status}</div>
                        </div>
                    {/each}
                </div>
            </div>
        {/if}
    </div>
</main>

<style>
    main {
        padding: 1rem;
        font-family: system-ui, sans-serif;
        max-width: 600px;
        margin: 0 auto;
    }

    .transfer-container {
        background: white;
        border-radius: 1rem;
        padding: 2rem;
        box-shadow: 0 4px 12px rgba(0,0,0,0.1);
    }

    .page-header {
        text-align: center;
        margin-bottom: 2rem;
    }

    .page-header h1 {
        margin: 0 0 0.5rem 0;
        color: #0066cc;
    }

    .page-header p {
        margin: 0;
        color: #666;
    }

    .balance-display {
        background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
        color: white;
        padding: 1.5rem;
        border-radius: 0.75rem;
        text-align: center;
        margin-bottom: 2rem;
    }

    .balance-display h3 {
        margin: 0 0 0.5rem 0;
        font-weight: 500;
        opacity: 0.9;
    }

    .balance-amount {
        font-size: 2rem;
        font-weight: bold;
        display: flex;
        align-items: center;
        justify-content: center;
        gap: 1rem;
    }

    .max-btn {
        background: rgba(255,255,255,0.2);
        color: white;
        border: none;
        padding: 0.25rem 0.75rem;
        border-radius: 0.5rem;
        font-size: 0.8rem;
        font-weight: bold;
        cursor: pointer;
        transition: background-color 0.2s;
    }

    .max-btn:hover:not(:disabled) {
        background: rgba(255,255,255,0.3);
    }

    .max-btn:disabled {
        opacity: 0.5;
        cursor: not-allowed;
    }

    .transfer-form {
        display: flex;
        flex-direction: column;
        gap: 1.5rem;
    }

    .form-group {
        display: flex;
        flex-direction: column;
        gap: 0.5rem;
    }

    .form-group label {
        font-weight: 600;
        color: #333;
    }

    .form-group input {
        padding: 0.75rem;
        border: 2px solid #e1e1e1;
        border-radius: 0.5rem;
        font-size: 1rem;
        transition: border-color 0.2s;
    }

    .form-group input:focus {
        outline: none;
        border-color: #0066cc;
    }

    .form-group input:disabled {
        background-color: #f5f5f5;
        cursor: not-allowed;
    }

    .form-group small {
        color: #666;
        font-size: 0.85rem;
    }

    .message {
        padding: 1rem;
        border-radius: 0.5rem;
        text-align: center;
        font-weight: 500;
    }

    .message.success {
        background-color: #d4edda;
        color: #155724;
        border: 1px solid #c3e6cb;
    }

    .message.error {
        background-color: #f8d7da;
        color: #721c24;
        border: 1px solid #f5c6cb;
    }

    .message.info {
        background-color: #d1ecf1;
        color: #0c5460;
        border: 1px solid #bee5eb;
    }

    .transfer-btn {
        background-color: #0066cc;
        color: white;
        padding: 1rem;
        border: none;
        border-radius: 0.5rem;
        font-size: 1.1rem;
        font-weight: bold;
        cursor: pointer;
        transition: background-color 0.2s;
        display: flex;
        align-items: center;
        justify-content: center;
        gap: 0.5rem;
    }

    .transfer-btn:hover:not(:disabled) {
        background-color: #004d99;
    }

    .transfer-btn:disabled {
        background-color: #cccccc;
        cursor: not-allowed;
    }

    .spinner {
        width: 1rem;
        height: 1rem;
        border: 2px solid transparent;
        border-top: 2px solid white;
        border-radius: 50%;
        animation: spin 1s linear infinite;
    }

    @keyframes spin {
        from { transform: rotate(0deg); }
        to { transform: rotate(360deg); }
    }

    .zero-balance-hint {
        background-color: #fff3cd;
        color: #856404;
        padding: 1rem;
        border-radius: 0.5rem;
        text-align: center;
        margin-top: 1rem;
        border: 1px solid #ffeaa7;
    }

    .zero-balance-hint p {
        margin: 0 0 0.5rem 0;
    }

    .mine-link {
        color: #0066cc;
        text-decoration: none;
        font-weight: bold;
    }

    .mine-link:hover {
        text-decoration: underline;
    }

    .recent-transactions {
        margin-top: 2rem;
        padding-top: 2rem;
        border-top: 1px solid #e1e1e1;
    }

    .recent-transactions h3 {
        margin: 0 0 1rem 0;
        color: #333;
    }

    .transaction-list {
        display: flex;
        flex-direction: column;
        gap: 0.75rem;
    }

    .transaction-item {
        display: flex;
        justify-content: space-between;
        align-items: center;
        padding: 1rem;
        background-color: #f8f9fa;
        border-radius: 0.5rem;
        border-left: 4px solid #dc3545;
    }

    .tx-info {
        display: flex;
        align-items: center;
        gap: 1rem;
    }

    .tx-amount {
        font-weight: bold;
        color: #dc3545;
        font-size: 1.1rem;
    }

    .tx-details {
        display: flex;
        flex-direction: column;
        gap: 0.25rem;
    }

    .tx-to {
        font-weight: 500;
        color: #333;
    }

    .tx-time {
        font-size: 0.85rem;
        color: #666;
    }

    .tx-status {
        color: #28a745;
        font-weight: 500;
        font-size: 0.9rem;
    }

    @media (max-width: 480px) {
        main {
            padding: 0.5rem;
        }

        .transfer-container {
            padding: 1rem;
        }

        .balance-amount {
            font-size: 1.5rem;
            flex-direction: column;
            gap: 0.5rem;
        }

        .transaction-item {
            flex-direction: column;
            align-items: flex-start;
            gap: 0.5rem;
        }

        .tx-info {
            width: 100%;
        }
    }
</style>