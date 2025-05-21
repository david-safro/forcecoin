<script>
    export let data;

    async function logout() {
        await fetch('/api/logout', { method: 'POST' });
        window.location.reload();
    }
</script>

<main>
    <header>
        Forcecoin
    </header>

    {#if data.user}
        <div class="top-left">
            <p>Hello, {data.user.username}</p>
            <button on:click={logout}>Logout</button>
        </div>
    {/if}

    <div class="forcecoin-section">
        <h2>Forcecoin Price</h2>
        <div class="price">{data.forcecoinPrice}</div>
        <h3>What is Forcecoin?</h3>
        <div class="description">
            {data.description}
        </div>

        <div class="homepage-buttons">
            <a href="/learn-more" class="btn">Learn More</a>

            {#if !data.user}
                <a href="/register" class="btn">Register</a>
                <a href="/login" class="btn">Login</a>
            {/if}
        </div>
    </div>

    <footer>
        <p>&copy; 2025 Forcecoin Information</p>
    </footer>
</main>

<style>
    * {
        margin: 0;
        padding: 0;
        box-sizing: border-box;
    }

    main {
        text-align: center;
        padding: 20px;
        font-family: system-ui, sans-serif;
    }

    header {
        background-color: #0066cc;
        color: white;
        padding: 20px 0;
        font-size: 32px;
        font-weight: bold;
    }

    .forcecoin-section {
        background-color: white;
        padding: 40px;
        margin-top: 20px;
        border-radius: 10px;
        box-shadow: 0 4px 10px rgba(0, 0, 0, 0.1);
        max-width: 600px;
        margin-left: auto;
        margin-right: auto;
    }

    .price {
        font-size: 2rem;
        font-weight: bold;
        margin-top: 10px;
        color: #0066cc;
    }

    .description {
        margin-top: 20px;
        font-size: 1.1rem;
        color: #444;
        line-height: 1.6;
    }

    .homepage-buttons {
        display: flex;
        justify-content: center;
        flex-wrap: wrap;
        gap: 1rem;
        margin-top: 2rem;
    }

    .btn {
        background-color: #0066cc;
        color: white;
        padding: 10px 20px;
        border-radius: 10px;
        text-decoration: none;
        font-weight: bold;
        transition: background-color 0.3s ease-in-out;
    }

    .btn:hover {
        background-color: #004d99;
    }

    .top-left {
        position: absolute;
        top: 1rem;
        left: 1rem;
        display: flex;
        flex-direction: column;
        align-items: flex-start;
        gap: 0.5rem;
    }

    .top-left button {
        padding: 0.4rem 0.8rem;
        background-color: #d23b3b;
        color: white;
        border: none;
        border-radius: 0.3rem;
        cursor: pointer;
    }

    .top-left button:hover {
        background-color: #a52b2b;
    }

    footer {
        margin-top: 30px;
        padding: 15px;
        background-color: #0066cc;
        color: white;
    }
</style>
