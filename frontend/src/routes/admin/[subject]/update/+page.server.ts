/** @type {import('./$types').PageLoad} */
import jwt from 'jsonwebtoken';
import { BASIC_AUTH_STRING, JWT_SECRET } from '$env/static/private';
export async function load({ cookies, fetch, params }) {

	const session = cookies.get('session');
	
	if (!!session) {
		const jwtUser = jwt.verify(session, JWT_SECRET);

		if (typeof jwtUser === 'object') {	
		console.log(jwtUser.user_id);
			const getRes = await fetch(`https://api.wtf.danielc.us/${params.subject}/${jwtUser.user_id}`, {
				method: 'GET',
				headers: {
					Authorization: `Basic ${BASIC_AUTH_STRING}`
				}
			});
			const userData = await getRes.json();

			delete userData.id;
			userData.email = ["test@testy.io"]
			let body = JSON.stringify(userData);

			const res = await fetch(`https://api.wtf.danielc.us/${params.subject}/${jwtUser.user_id}`, {
				method: 'PUT',
				headers: {
					Authorization: `Basic ${BASIC_AUTH_STRING}`,
					'Content-Type': `application/x-new_user`
				},
				body
			});
			const data = await res.json();
			console.log(data);
			return { data };
		}
	} else {
		const data = {}
		console.log(data)
		return { data };
	}
}