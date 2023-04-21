/** @type {import('./$types').PageLoad} */
import { BASIC_AUTH_STRING } from '$env/static/private';
export async function load({ fetch, params }) {
	const res = await fetch(`https://api.wtf.danielc.us/${params.subject}/${params.action}/${params.further}`, {
		method: 'GET',
		headers: {
			'Content-Type': 'form/data',
			Authorization: `Basic ${BASIC_AUTH_STRING}`
		}
	});
	const data = await res.json();
console.log(data)
	return { data };
}
