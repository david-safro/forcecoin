import { json } from '@sveltejs/kit';
import { PrismaClient } from '@prisma/client';
import bcrypt from 'bcrypt';

const prisma = new PrismaClient();

export async function POST({ request }) {
    try {
        const { name, email, password } = await request.json();
        console.log("Register request received:", { name, email });

        const trimmedName = name?.trim();
        const trimmedEmail = email?.trim();
        const trimmedPassword = password?.trim();

        if (!trimmedName || !trimmedEmail || !trimmedPassword) {
            console.log("Missing fields");
            return json({ error: "All fields are required" }, { status: 400 });
        }

        // Check if user exists already
        const existingUser = await prisma.user.findUnique({ where: { email: trimmedEmail } });
        if (existingUser) {
            console.log("User already exists:", trimmedEmail);
            return json({ error: "User already exists" }, { status: 400 });
        }

        // Hash password
        const hashedPassword = await bcrypt.hash(trimmedPassword, 10);

        // Create new user
        const user = await prisma.user.create({
            data: {
                name: trimmedName,
                email: trimmedEmail,
                password: hashedPassword
            }
        });

        console.log("User registered successfully:", user.email);
        return json({ message: 'User registered successfully' });

    } catch (error) {
        console.error("Error in register:", error);
        return json({ error: "Internal server error" }, { status: 500 });

    } finally {
        await prisma.$disconnect();
    }
}
