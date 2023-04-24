import jwt from 'jsonwebtoken';
import type { Handle } from '@sveltejs/kit';
/** @type {import('@sveltejs/kit').Handle} */
import { JWT_SECRET } from '$env/static/private';

function redirect(location: string, body?: string) {
    return new Response(body, {
        status: 303,
        headers: { location }
    });
}

const unProtectedRoutes: string[] = [
    '/',
    '/login',
    '/about',
];

// User ids
const admin = '5fc84a6c-0f69-4d40-98aa-2a8608c5bc59';

export const handle: Handle = async ({ event, resolve }) => {
	const session = event.cookies.get('session');
	if (!session && !unProtectedRoutes.includes(event.url.pathname))
		return redirect('/login', 'No authenticated user.');
        
	if (!session) {
		if (!unProtectedRoutes.includes(event.url.pathname)) return redirect('/', 'Not a valid user');
        
	} else {
        try {
            const jwtUser = jwt.verify(session, JWT_SECRET);
            if (typeof jwtUser === 'object') {
                event.locals = jwtUser;
                if (event.url.pathname.includes('admin') && jwtUser.user_id !== admin)
					return redirect('/', 'Not Admin');
            }
            
        } catch (error) {
            console.error(error);
        }
    }

	return resolve(event);
};