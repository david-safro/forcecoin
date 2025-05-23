<!-- src/routes/settings/+page.svelte -->
<script>
    import { onMount } from 'svelte';

    let nodeStatus = 'Unknown';
    let blockchainStats = null;
    let isLoadingStats = false;
    let isStartingNode = false;
    let message = '';

    /**
     * @param {string} action
     */
    async function callBlockchainAPI(action, params = {}) {
        const response = await fetch('/api/blockchain', {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ action, ...params })
        });
        return await response.json();
    }

    async function startNode() {
        isStartingNode = true;
        message = 'Starting blockchain node...';

        try {
            const result = await callBlockchainAPI('start_node');
            if (result.success) {
                //nodeStatus = 'Online';
                message = 'Node started successfully!';
                // Wait a moment then refresh stats
                setTimeout(() => {
                    fetchBlockchainStats();
                    message = '';
                }, 2000);
            } else {
                //nodeStatus = 'Offline';
                message = `Failed to start node: ${result.error}`;
            }
        } catch (error) {
            message = 'Failed to start node';
        } finally {
            isStartingNode = false;
            setTimeout(() => message = '', 5000);
        }
    }

    async function fetchBlockchainStats() {
        isLoadingStats = true;
        try {
            const result = await callBlockchainAPI('get_stats');
            if (result.success) {
                blockchainStats = result.stats;
                nodeStatus = 'Online';
            } else {
                nodeStatus = 'Offline';
                blockchainStats = null;
            }
        } catch (error) {
            console.error('Error fetching stats:', error);
            nodeStatus = 'Offline';
            blockchainStats = null;
        } finally {
            isLoadingStats = false;
        }
    }

    async function createUserWallet() {
        try {
            const result = await callBlockchainAPI('create_wallet');
            if (result.success) {
                message = 'Wallet initialized successfully!';
                setTimeout(() => message = '', 3000);
            }
        } catch (error) {
            console.error('Error creating wallet:', error);
        }
    }

    /**
     * @param {null} statsString
     */
    function parseStats(statsString) {
        if (!statsString || statsString === 'Node offline') return null;

        const lines = statsString.split('\n');
        const stats = {};

        lines.forEach(line => {
            const [key, value] = line.split(': ');
            if (key && value) {
                stats[key] = value;
            }
        });

        return stats;
    }

    onMount(() => {
        fetchBlockchainStats();
        createUserWallet();

        // Refresh stats every 30 seconds
        const interval = setInterval(fetchBlockchainStats, 30000);
        return () => clearInterval(interval);
    });

    $: parsedStats = parseStats(blockchainStats);
</script>

<main>
    <div class="settings-container">
        <header class="page-header">
            <h1>⚙️ Settings & Node Status</h1>
            <p>Manage your ForceCoin node and view blockchain information</p>
        </header>

        {#if message}
            <div class="message-banner" class:success={message.includes('successfully')} class:error={message.includes('Failed')}>
                {message}
            </div>
        {/if}

        <!-- Node Status Section -->
        <div class="settings-section">
            <h2>🌐 Blockchain Node Status</h2>
            <div class="status-card">
                <div class="status-row">
                    <span class="status-label">Node Status:</span>
                    <span class="status-value" class:online={nodeStatus === 'Online'} class:offline={nodeStatus === 'Offline'}>
                        {nodeStatus}
                        {#if nodeStatus === 'Online'}
                            🟢
                        {/if}
                    </span>
                </div>

                <div class="node-actions">
                    <button
                            on:click={startNode}
                            disabled={isStartingNode || nodeStatus === 'Online'}
                            class="action-btn primary"
                    >
                        {#if isStartingNode}
                            Starting...
                        {:else}
                            Start Node
                        {/if}
                    </button>

                    <button
                            on:click={fetchBlockchainStats}
                            disabled={isLoadingStats}
                            class="action-btn secondary"
                    >
                        {#if isLoadingStats}
                            Refreshing...
                        {:else}
                            Refresh Status
                        {/if}
                    </button>
                </div>
            </div>
        </div>

        <!-- Blockchain Stats Section -->
        <div class="settings-section">
            <h2>📊 Blockchain Statistics</h2>
            {#if isLoadingStats}
                <div class="loading-card">
                    <div class="spinner"></div>
                    <p>Loading blockchain statistics...</p>
                </div>
            {:else if parsedStats}
                <div class="stats-grid">
                    <div class="stat-card">
                        <div class="stat-icon">🔗</div>
                        <div class="stat-info">
                            <div class="stat-label">Total Blocks</div>
                            <div class="stat-value">{parsedStats['Blocks'] || '0'}</div>
                        </div>
                    </div>

                    <div class="stat-card">
                        <div class="stat-icon">💳</div>
                        <div class="stat-info">
                            <div class="stat-label">Transactions</div>
                            <div class="stat-value">{parsedStats['Transactions'] || '0'}</div>
                        </div>
                    </div>

                    <div class="stat-card">
                        <div class="stat-icon">⛏️</div>
                        <div class="stat-info">
                            <div class="stat-label">Mining Difficulty</div>
                            <div class="stat-value">{parsedStats['Difficulty'] || '0'}</div>
                        </div>
                    </div>

                    <div class="stat-card">
                        <div class="stat-icon">⏱️</div>
                        <div class="stat-info">
                            <div class="stat-label">Avg Block Time</div>
                            <div class="stat-value">{parsedStats['Average Block Time'] || 'N/A'}</div>
                        </div>
                    </div>

                    <div class="stat-card">
                        <div class="stat-icon">⏳</div>
                        <div class="stat-info">
                            <div class="stat-label">Pending Transactions</div>
                            <div class="stat-value">{parsedStats['Pending Transactions'] || '0'}</div>
                        </div>
                    </div>
                </div>
            {:else}
                <div class="offline-card">
                    <p>📡 Blockchain node is offline. Start the node to view statistics.</p>
                </div>
            {/if}
        </div>

        <!-- User Info Section -->
        <div class="settings-section">
            <h2>👤 Account Information</h2>
            <div class="info-card">
                <div class="info-row">
                    <span class="info-label">Wallet ID:</span>
                    <span class="info-value">user_{window.location.pathname.includes('dashboard') ? 'active' : 'guest'}</span>
                </div>
                <div class="info-row">
                    <span class="info-label">Node Address:</span>
                    <span class="info-value">127.0.0.1:7001</span>
                </div>
                <div class="info-row">
                    <span class="info-label">Network:</span>
                    <span class="info-value">ForceCoin Testnet</span>
                </div>
            </div>
        </div>

        <!-- Help Section -->
        <div class="settings-section">
            <h2>❓ Getting Started</h2>
            <div class="help-card">
                <div class="help-item">
                    <div class="help-number">1</div>
                    <div class="help-text">
                        <strong>Start the node:</strong> Click "Start Node" to begin participating in the ForceCoin network
                    </div>
                </div>
                <div class="help-item">
                    <div class="help-number">2</div>
                    <div class="help-text">
                        <strong>Mine coins:</strong> Go to Dashboard and click "Mine" to earn your first ForceCoin
                    </div>
                </div>
                <div class="help-item">
                    <div class="help-number">3</div>
                    <div class="help-text">
                        <strong>Transfer funds:</strong> Use the Transfer page to send coins to other users
                    </div>
                </div>
            </div>
        </div>
    </div>
</main>

<style>
    main {
        padding: 1rem;
        font-family: system-ui, sans-serif;
        max-width: 800px;
        margin: 0 auto;
    }

    .settings-container {
        display: flex;
        flex-direction: column;
        gap: 2rem;
    }

    .page-header {
        text-align: center;
        margin-bottom: 1rem;
    }

    .page-header h1 {
        margin: 0 0 0.5rem 0;
        color: #0066cc;
    }

    .page-header p {
        margin: 0;
        color: #666;
    }

    .message-banner {
        padding: 1rem;
        border-radius: 0.5rem;
        text-align: center;
        font-weight: 500;
        margin-bottom: 1rem;
    }

    .message-banner.success {
        background-color: #d4edda;
        color: #155724;
        border: 1px solid #c3e6cb;
    }

    .message-banner.error {
        background-color: #f8d7da;
        color: #721c24;
        border: 1px solid #f5c6cb;
    }

    .settings-section {
        background: white;
        border-radius: 1rem;
        padding: 1.5rem;
        box-shadow: 0 4px 12px rgba(0,0,0,0.1);
    }

    .settings-section h2 {
        margin: 0 0 1rem 0;
        color: #333;
        font-size: 1.25rem;
    }

    .status-card, .info-card {
        border: 2px solid #e1e1e1;
        border-radius: 0.75rem;
        padding: 1.5rem;
    }

    .status-row, .info-row {
        display: flex;
        justify-content: space-between;
        align-items: center;
        margin-bottom: 1rem;
    }

    .status-row:last-child, .info-row:last-child {
        margin-bottom: 0;
    }

    .status-label, .info-label {
        font-weight: 600;
        color: #333;
    }

    .status-value {
        font-weight: bold;
        padding: 0.25rem 0.75rem;
        border-radius: 1rem;
        font-size: 0.9rem;
    }

    .status-value.online {
        background-color: #d4edda;
        color: #155724;
    }

    .status-value.offline {
        background-color: #f8d7da;
        color: #721c24;
    }

    .info-value {
        font-family: monospace;
        background-color: #f8f9fa;
        padding: 0.25rem 0.5rem;
        border-radius: 0.25rem;
        font-size: 0.9rem;
    }

    .node-actions {
        display: flex;
        gap: 1rem;
        margin-top: 1rem;
        justify-content: center;
    }

    .action-btn {
        padding: 0.75rem 1.5rem;
        border: none;
        border-radius: 0.5rem;
        font-weight: bold;
        cursor: pointer;
        transition: all 0.2s;
    }

    .action-btn.primary {
        background-color: #0066cc;
        color: white;
    }

    .action-btn.primary:hover:not(:disabled) {
        background-color: #004d99;
    }

    .action-btn.secondary {
        background-color: #6c757d;
        color: white;
    }

    .action-btn.secondary:hover:not(:disabled) {
        background-color: #545b62;
    }

    .action-btn:disabled {
        background-color: #cccccc;
        cursor: not-allowed;
    }

    .loading-card, .offline-card {
        text-align: center;
        padding: 2rem;
        border: 2px dashed #e1e1e1;
        border-radius: 0.75rem;
        color: #666;
    }

    .spinner {
        width: 2rem;
        height: 2rem;
        border: 3px solid #e1e1e1;
        border-top: 3px solid #0066cc;
        border-radius: 50%;
        animation: spin 1s linear infinite;
        margin: 0 auto 1rem;
    }

    @keyframes spin {
        from { transform: rotate(0deg); }
        to { transform: rotate(360deg); }
    }

    .stats-grid {
        display: grid;
        grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
        gap: 1rem;
    }

    .stat-card {
        background: linear-gradient(135deg, #f8f9fa 0%, #e9ecef 100%);
        border-radius: 0.75rem;
        padding: 1.25rem;
        display: flex;
        align-items: center;
        gap: 1rem;
        border: 1px solid #dee2e6;
    }

    .stat-icon {
        font-size: 2rem;
        width: 3rem;
        height: 3rem;
        display: flex;
        align-items: center;
        justify-content: center;
        background: white;
        border-radius: 50%;
        box-shadow: 0 2px 4px rgba(0,0,0,0.1);
    }

    .stat-info {
        flex: 1;
    }

    .stat-label {
        font-size: 0.85rem;
        color: #666;
        margin-bottom: 0.25rem;
    }

    .stat-value {
        font-size: 1.5rem;
        font-weight: bold;
        color: #333;
    }

    .help-card {
        border: 2px solid #e1e1e1;
        border-radius: 0.75rem;
        padding: 1.5rem;
    }

    .help-item {
        display: flex;
        align-items: flex-start;
        gap: 1rem;
        margin-bottom: 1.5rem;
    }

    .help-item:last-child {
        margin-bottom: 0;
    }

    .help-number {
        background-color: #0066cc;
        color: white;
        width: 2rem;
        height: 2rem;
        border-radius: 50%;
        display: flex;
        align-items: center;
        justify-content: center;
        font-weight: bold;
        flex-shrink: 0;
    }

    .help-text {
        flex: 1;
        padding-top: 0.25rem;
    }

    @media (max-width: 768px) {
        main {
            padding: 0.5rem;
        }

        .settings-section {
            padding: 1rem;
        }

        .node-actions {
            flex-direction: column;
        }

        .stats-grid {
            grid-template-columns: 1fr;
        }

        .status-row, .info-row {
            flex-direction: column;
            align-items: flex-start;
            gap: 0.5rem;
        }
    }
</style>