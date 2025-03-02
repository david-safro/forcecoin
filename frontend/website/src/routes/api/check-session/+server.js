import { json } from '@sveltejs/kit';

export function GET({ cookies }) {
    const session = cookies.get('session');
    if (!session) {
        return json({ loggedIn: false });
    }

    return json({ loggedIn: true, session: JSON.parse(session) });
}
