import { json } from '@sveltejs/kit';
import bcrypt from 'bcryptjs';
import { prisma } from '$lib';

export async function POST({ request, cookies }) {
    try {
        const { email, password } = await request.json();

        // Find user
        const user = await prisma.user.findUnique({ where: { email } });
        if (!user) {
            return json({ error: 'Invalid credentials' }, { status: 401 });
        }

        // Compare passwords
        const passwordMatch = await bcrypt.compare(password, user.password);
        if (!passwordMatch) {
            return json({ error: 'Invalid credentials' }, { status: 401 });
        }

        // Store login session for 7 days
        cookies.set('session', JSON.stringify({ userId: user.id, email: user.email }), {
            httpOnly: true, // Prevents JavaScript access
            secure: process.env.NODE_ENV === 'production',
            path: '/',
            maxAge: 60 * 60 * 24 * 7 // 7 days
        });

        return json({ message: 'Login successful' });
    } catch (error) {
        console.error('Login error:', error);
        return json({ error: 'Internal server error' }, { status: 500 });
    }
}
