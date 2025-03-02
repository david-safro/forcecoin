export async function load({ fetch, url }) {
    const res = await fetch('/api/auth');
    const { user } = await res.json();
    return { user, url };
}
