<script>
    export let forcecoinPrice;
    export let description;
</script>

<style>
    * {
        margin: 0;
        padding: 0;
        box-sizing: border-box;
    }

    main {
        text-align: center;
        padding: 20px;
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

    footer {
        margin-top: 30px;
        padding: 15px;
        background-color: #0066cc;
        color: white;
    }

    /* Buttons */
    .btn, .btnTwo {
        background-color: #0066cc;
        color: white;
        padding: 10px 20px;
        border-radius: 10px;
        text-decoration: none;
        font-weight: bold;
        display: inline-block;
        margin: 10px;
        transition: background-color 0.3s ease-in-out;
    }

    .btn:hover, .btnTwo:hover {
        background-color: #004d99;
    }
</style>

<main>
    <header>
        Forcecoin Homepage
    </header>

    <div class="forcecoin-section">
        <h2>Forcecoin Price</h2>
        <div class="price">{forcecoinPrice}</div>
        <h3>What is Forcecoin?</h3>
        <div class="description">
            {description}
        </div>

        <a href="/learn-more" class="btn">Learn More</a>
        <a href="/register" class="btnTwo">Register</a>
    </div>

    <footer>
        <p>&copy; 2025 Forcecoin Information</p>
    </footer>
</main>
