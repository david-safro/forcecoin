<script>
    let showMessage = '';
    function handleClick(action) {
        showMessage = `Forcecoin ${action} (coming soon)`;
        setTimeout(() => showMessage = '', 2000);
    }
</script>

<div class="action-buttons">
    <button on:click={() => handleClick('Buy')}>
        Buy
    </button>
    <button on:click={() => handleClick('Sell')}>
        Sell
    </button>
    <button on:click={() => handleClick('Mine')}>
        Mine
    </button>
    {#if showMessage}
        <div class="popup">{showMessage}</div>
    {/if}
</div>

<style>
    .action-buttons {
        display: flex;
        justify-content: center;
        gap: 1rem;
        margin-top: 1.5rem;
        position: relative;
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
    }

    button:hover {
        transform: translateY(-2px);
        box-shadow: 0 4px 8px rgba(0, 0, 0, 0.2);
        background-color: #004d99;
    }

    .popup {
        position: absolute;
        top: -2rem;
        background-color: #f0f0f0;
        color: #333;
        padding: 0.5rem 1rem;
        border-radius: 6px;
        box-shadow: 0 2px 6px rgba(0,0,0,0.15);
        animation: fade 0.3s ease;
    }

    @keyframes fade {
        from { opacity: 0; transform: translateY(-5px); }
        to { opacity: 1; transform: translateY(0); }
    }
</style>
