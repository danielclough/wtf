import { error, type Actions } from "@sveltejs/kit";
import { BASIC_AUTH_STRING } from '$env/static/private';

import crypto from 'crypto';

export const actions: Actions = {
	register: async ({ request }) => {
		try {
			const form = await request.formData();
			const password = form.get('password');
			const email = form.get('email');
			const mfa = form.get('mfa');

			if (typeof email === 'string' && typeof password === 'string' && typeof mfa === 'string') {
				const forPwSalt= new Uint32Array(3);
				const pw_salt = crypto.getRandomValues(forPwSalt).toString();
				let pw_hash: string = crypto.scryptSync(password, pw_salt, 16).toString('base64');

				const forMfaSalt= new Uint32Array(3);
				const mfa_salt = crypto.getRandomValues(forMfaSalt).toString();
				let mfa_hash: string = crypto.scryptSync(mfa, mfa_salt, 16).toString('base64');

				const response = await fetch(`https://api.wtf.danielc.us/login/create`, {
					method: 'POST',
					headers: {
						'Content-Type': 'application/x-new_login',
						Authorization: `Basic ${BASIC_AUTH_STRING}`
					},
					body: JSON.stringify({
						email,
						pw_salt: pw_salt.toString(),
						pw_hash,
						mfa_salt: mfa_salt.toString(),
						mfa_hash
					})
				});

				
				const responseJson = await response.json();
				console.log(responseJson);
				
				return { data: responseJson.data };
			}
			return undefined;
		} catch (err: unknown) {
			const message = `Error in /login form: ${err as string}`;
			console.error(message);
			throw error(500, message);
		}
	},
	login: async ({ request }) => {}
};