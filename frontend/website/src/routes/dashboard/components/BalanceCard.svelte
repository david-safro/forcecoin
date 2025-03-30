<script>
    export let balance = 0.00;
    export let price = 0.00;
    export let change = 0.0;
</script>

<div class="balance-card">
    <div class="balance-top">
        <h2>{balance} FC</h2>
        <p>${(balance * price).toFixed(2)} USD</p>
    </div>
    <div class="price-info">
        <p>1 FC = ${price}</p>
        <p class={change >= 0 ? 'positive' : 'negative'}>
            {change >= 0 ? '+' : ''}{change}% today
        </p>
    </div>
</div>

<style>
    .balance-card {
        background: white;
        border-radius: 1rem;
        padding: 1rem;
        box-shadow: 0 4px 12px rgba(0,0,0,0.1);
        margin-bottom: 1rem;
    }

    .balance-top {
        display: flex;
        justify-content: space-between;
        align-items: center;
    }

    .price-info {
        display: flex;
        justify-content: space-between;
        margin-top: 0.5rem;
        font-size: 0.9rem;
    }

    .positive {
        color: green;
    }

    .negative {
        color: red;
    }
</style>
