import { redirect, fail } from '@sveltejs/kit';
import { prisma } from '$lib/server/prisma';
import bcrypt from 'bcryptjs';

export const actions = {
    default: async ({ request, cookies }) => {
        const formData = await request.formData();
        const email = formData.get('email');
        const password = formData.get('password');


        const user = await prisma.user.findUnique({ where: { email } });

        if (!user) {
            return fail(400, { error: 'Invalid email or password' });
        }

        const passwordMatch = await bcrypt.compare(password, user.password);
        if (!password) {
            return fail(400, {error: 'Password is required'});
        }
        if (!passwordMatch) {
            return fail(400, { error: 'Invalid email or password' });
        }

        cookies.set('session', JSON.stringify({
            userId: user.id,
            email: user.email,
            name: user.name
        }), {
            httpOnly: true,
            secure: process.env.NODE_ENV === 'production',
            path: '/',
            maxAge: 60 * 60 * 24 * 7
        });

        throw redirect(302, '/dashboard');
    }
};


