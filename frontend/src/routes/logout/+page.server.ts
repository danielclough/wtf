import { cookieOpts } from '$lib/utils/session.js';
import { redirect } from '@sveltejs/kit';

/** @type {import('./$types').PageLoad} */
export async function load({ cookies }) {
	cookies.set('session', '', cookieOpts);
	
	throw redirect(303, '/');
}
