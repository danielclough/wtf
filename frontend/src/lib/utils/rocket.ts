import { BASIC_AUTH_STRING } from '$env/static/private';

export const postLogin = async (url: string, body: any) => {
	return await fetch(url, {
		method: 'POST',
		headers: {
			'Content-Type': 'application/x-new_login',
			Authorization: `Basic ${BASIC_AUTH_STRING}`
		},
		body
	});
};

export const getRocket = async (url: string) => {
	return await fetch(url, {
		method: 'GET',
		headers: {
			Authorization: `Basic ${BASIC_AUTH_STRING}`
		}
	});
};
