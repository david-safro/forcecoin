<script>
    let name = '';
    let email = '';
    let password = '';
    let error = '';
    let isLoading = false;

    async function handleRegister() {
        isLoading = true;

        const res = await fetch('/api/register', {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ name, email, password })
        });

        const data = await res.json();
        if (res.ok) {
            window.location.href = '/login';
        } else {
            error = data.error;
            isLoading = false;
        }
    }
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

    .register-container {
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

    .btn:disabled {
        background-color: #cccccc;
        cursor: not-allowed;
    }

    .error {
        color: red;
        margin-top: 10px;
    }
</style>

<main>
    <header>
        Register
    </header>

    <div class="register-container">
        <h2>Create an Account</h2>
        <p>Join us today!</p>

        <input type="text" placeholder="Name" bind:value={name} />
        <input type="email" placeholder="Email" bind:value={email} />
        <input type="password" placeholder="Password" bind:value={password} />

        {#if error}
            <p class="error">{error}</p>
        {/if}

        <button class="btn" on:click={handleRegister} disabled={isLoading}>
            {isLoading ? 'Registering...' : 'Register'}
        </button>

        <p>Already have an account? <a href="/login">Login here</a></p>
    </div>
</main>

