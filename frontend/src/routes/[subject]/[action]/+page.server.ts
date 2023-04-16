/** @type {import('./$types').PageLoad} */
import { BASIC_AUTH_PW } from '$env/static/private';
export async function load({ fetch, params }) {
	const res = await fetch(`https://api.wtf.danielc.us/${params.subject}/${params.action}`, {
		method: 'POST',
		headers: {
			'Content-Type': 'form/data',
			'Authorization': `Basic ${Buffer.from('user:' + BASIC_AUTH_PW).toString('base64')}`
		}
	});
	const data = await res.json();
console.log(data)
	return { data };
}
