<script>
    let email = '';
    let password = '';
    let error = '';
    let isLoading = false;

    export let form;
    if (form?.error) {
        error = form.error;
    }
</script>

<main>
    <header>
        Login
    </header>

    <div class="login-container">
        <h2>Welcome Back</h2>
        <p>Please enter your credentials</p>

        <form method="POST">
            <input type="email" name="email" placeholder="Email" bind:value={email} required />
            <input type="password" name="password" placeholder="Password" bind:value={password} required />

            {#if error}
                <p class="error">{error}</p>
            {/if}

            <button class="btn" type="submit" disabled={isLoading}>
                {isLoading ? 'Logging in...' : 'Login'}
            </button>
        </form>

        <p>Don't have an account? <a href="/register">Register here</a></p>
    </div>
    <div>
        <a href="/" class="btn">Back to Homepage</a>
    </div>
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

    .login-container {
        background-color: white;
        padding: 40px;
        margin-top: 20px;
        border-radius: 10px;
        box-shadow: 0 4px 10px rgba(0, 0, 0, 0.1);
        max-width: 400px;
        margin-left: auto;
        margin-right: auto;
    }

    input {
        width: 100%;
        padding: 10px;
        margin: 10px 0;
        border: 1px solid #ccc;
        border-radius: 5px;
    }

    .btn {
        background-color: #0066cc;
        color: white;
        padding: 10px 20px;
        border-radius: 10px;
        text-decoration: none;
        font-weight: bold;
        display: inline-block;
        margin: 10px;
        cursor: pointer;
        border: none;
        transition: background-color 0.3s ease-in-out;
    }

    .btn:hover {
        background-color: #004d99;
    }

    .error {
        color: red;
        margin-top: 10px;
    }
</style>
