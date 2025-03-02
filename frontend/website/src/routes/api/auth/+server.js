import { json } from '@sveltejs/kit';

export async function GET({ cookies }) {
    const session = cookies.get('session');

    if (!session) {
        return json({ user: null });
    }

    const user = JSON.parse(session);
    return json({ user });
}
