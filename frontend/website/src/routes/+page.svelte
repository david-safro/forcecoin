<script>
    let forcecoinPrice = "$0";
    let description = `Crypto users and Charter School of Wilmington students will be utilizing this.
										No age restriction.
										The product allows users to sell and buy cryptocurrency, learning how it works.`;
</script>

<style>
    * {
        margin: 0;
        padding: 0;
        box-sizing: border-box;
    }

    body {
        font-family: Arial, sans-serif;
        background-color: #f0f4f8; /* Light blue background */
        color: #333;
        line-height: 1.6;
    }

    main {
        text-align: center;
        padding: 10px;
    }

    header {
        background-color: #0066cc; /* Blue header */
        color: white;
        padding: 20px 0px;
        font-size: 32px;
        font-weight: bold;
    }

    .forcecoin-section {
        background-color: white; /* White background */
        padding: 40px;
        margin-top: 20px;
        border-radius: 10px;
        box-shadow: 0px 4px 10px rgba(0, 0, 0, 0.1);
    }

    .price {
        font-size: 2rem;
        font-weight: bold;
        margin-top: 5px;
        color: #0066cc; /* Blue price */
    }

    .description {
        margin-top: 20px;
        font-size: 1.1rem;
        color: #444;
        line-height: 1.6;
    }

    footer {
        margin-top: 30px;
        padding: 10px;
        background-color: #0066cc; /* Blue footer */
        color: white;
    }

    /* Button */
    .btn {
        background-color: #0066cc;
        color: white;
        padding: 10px 20px;
        border-radius: 10px;
        text-decoration: none ;
        font-weight: bold;
    }

    .btn:hover {
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

        <a href="#" class="btn">Learn More</a>
    </div>

    <footer>
        <p>&copy; 2025 Forcecoin Information</p>
    </footer>
</main>

