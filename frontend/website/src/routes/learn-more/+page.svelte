<script>
    export let forcecoinDetails = "We Will Add More Later";
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

    .info-container {
        background-color: white;
        padding: 40px;
        margin-top: 20px;
        border-radius: 10px;
        box-shadow: 0 4px 10px rgba(0, 0, 0, 0.1);
        max-width: 600px;
        margin-left: auto;
        margin-right: auto;
    }

    .description {
        margin-top: 20px;
        font-size: 1.1rem;
        color: #444;
        line-height: 1.6;
    }

    .btn {
        background-color: #0066cc;
        color: white;
        padding: 10px 20px;
        border-radius: 10px;
        text-decoration: none;
        font-weight: bold;
        display: inline-block;
        margin-top: 20px;
        transition: background-color 0.3s ease-in-out;
    }

    .btn:hover {
        background-color: #004d99;
    }

    footer {
        margin-top: 30px;
        padding: 15px;
        background-color: #0066cc;
        color: white;
    }
</style>

<main>
    <header>
        Learn More About Forcecoin
    </header>

    <div class="info-container">
        <h2>What is Forcecoin?</h2>
        <div class="description">
            {forcecoinDetails}
        </div>

        <a href="/" class="btn">Back to Homepage</a>
    </div>

    <footer>
        <p>&copy; 2025 Forcecoin Information</p>
    </footer>
</main>
