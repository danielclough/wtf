import type { SessionUser } from '$lib/utils/security';
import jwt from 'jsonwebtoken';
import { error, type Actions, redirect } from '@sveltejs/kit';
import { JWT_SECRET } from '$env/static/private';

import crypto from 'crypto';
import { postLogin, getRocket } from '$lib/utils/rocket';

export const actions: Actions = {
	register: async ({ cookies, request }) => {
		try {
			const form = await request.formData();
			const first_name = form.get('first_name');
			const last_name = form.get('last_name');
			const password = form.get('password');
			const email = form.get('email');
			const mfa = form.get('mfa');

			if (
				typeof first_name === 'string' &&
				typeof last_name === 'string' &&
				typeof email === 'string' &&
				typeof password === 'string' &&
				typeof mfa === 'string'
			) {
				const forPwSalt = new Uint32Array(3);
				const pw_salt = crypto.getRandomValues(forPwSalt).toString();
				let pw_hash: string = crypto.scryptSync(password, pw_salt, 16).toString('base64');

				const forMfaSalt = new Uint32Array(3);
				const mfa_salt = crypto.getRandomValues(forMfaSalt).toString();
				let mfa_hash: string = crypto.scryptSync(mfa, mfa_salt, 16).toString('base64');

				const loginResponse = await postLogin(
					'https://api.wtf.danielc.us/login/create',
					JSON.stringify({
						email,
						pw_salt,
						pw_hash,
						mfa_salt,
						mfa_hash
					})
				);
				const loginResponseJson = await loginResponse.json();
				const loginData = loginResponseJson.data;

				const userResponse = await postLogin(
					'https://api.wtf.danielc.us/user/create',
					JSON.stringify({
						first_name,
						last_name,
						login_ids: [loginResponseJson.id],
						address: [],
						address_verified: [],
						email: [email],
						email_verified: [false],
						phone: [],
						phone_verified: [],
						taint: ''
					})
				);
				const userResponseJson = await userResponse.json();
				const userData = userResponseJson.data;

				const accountResponse = await postLogin(
					'https://api.wtf.danielc.us/login/create',
					JSON.stringify({
						avatar: '',
						level: 'beta',
						user_ids: [userResponseJson.id],
						preference_ids: [userResponseJson.id],
						role_ids: [userResponseJson.id],
						sensitivity_ids: [userResponseJson.id],
						survey_results_ids: [userResponseJson.id]
					})
				);
				const accountResponseJson = await accountResponse.json();
				const accountData = accountResponseJson.data;

				const sessionUser: SessionUser = {
					id: loginData.id,
					first_name: userData.first_name,
					account_id: accountData.id,
					avatar: accountData.avatar,
					level: accountData.level,
					preference_ids: accountData.preference_ids[0],
					role_ids: accountData.role_ids[0],
					sensitivity_ids: accountData.sensitivity_ids[0],
					survey_results_ids: accountData.survey_results_ids[0],
					user_ids: accountData.user_ids[0]
				};

				const sessionJwt = jwt.sign(sessionUser, JWT_SECRET);

				cookies.set('session', sessionJwt);

			}
		} catch (err: unknown) {
			const message = `Error in /login form: ${err as string}`;
			console.error(message);
			throw error(500, message);
		}
		throw redirect(303, '/');
	},
	login: async ({ cookies, request }) => {
		try {
			const form = await request.formData();
			const email = form.get('email');
			const password = form.get('password');

			if (typeof email === 'string' && typeof password === 'string') {
				const loginResponse: any = await getRocket(`https://api.wtf.danielc.us/login/email/${email}`);
				const loginResponseJson = await loginResponse.json();

				let pw_check =
					crypto.scryptSync(password, loginResponseJson.pw_salt, 16).toString('base64') ==
					loginResponseJson.pw_hash;

				if (pw_check) {
					const userResponse = await getRocket(
						`https://api.wtf.danielc.us/user/login/${loginResponseJson.id}`
					);
					const userResponseJson = await userResponse.json();

					const accountResponse = await getRocket(
						`https://api.wtf.danielc.us/account/user/${userResponseJson.id}`
					);
					const accountResponseJson: any = await accountResponse.json();

					const sessionUser: SessionUser = {
						id: loginResponseJson.id,
						first_name: userResponseJson.first_name,
						account_id: accountResponseJson.id,
						avatar: accountResponseJson.avatar,
						level: accountResponseJson.level,
						preference_ids: accountResponseJson.preference_ids[0],
						role_ids: accountResponseJson.role_ids[0],
						sensitivity_ids: accountResponseJson.sensitivity_ids[0],
						survey_results_ids: accountResponseJson.survey_results_ids[0],
						user_ids: accountResponseJson.user_ids[0]
					};

					const sessionJwt = jwt.sign(sessionUser, JWT_SECRET);

					cookies.set('session', sessionJwt);

				}
			}
		} catch (err: unknown) {
			const message = `Error in /login form: ${err as string}`;
			console.error(message);
			throw error(500, message);
		}
		throw redirect(303, '/');
	}
};
