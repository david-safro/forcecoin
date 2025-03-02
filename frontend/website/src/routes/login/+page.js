/** @type {import('./$types').PageLoad} */
export async function load({ fetch, depends }) {
    depends('app:auth');

    const response = await fetch('/api/auth');
    const user = response.ok ? await response.json() : null;

    return { user };
}
