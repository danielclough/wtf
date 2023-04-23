import { cookieOpts, type SessionUser } from '$lib/utils/session';
import jwt from 'jsonwebtoken';
import { error, type Actions, redirect } from '@sveltejs/kit';
import { JWT_SECRET } from '$env/static/private';

import crypto from 'crypto';
import { postRocket, getRocket } from '$lib/utils/rocket';

export const actions: Actions = {
	submit: async ({ cookies, request }) => {
		try {
			const form = await request.formData();
			const answers = form.get('answers');
			const subject = form.get('subject');
			const session = cookies.get('session');
			
			if (!!session && typeof answers === 'string' && typeof subject === 'string') {
				const tmp = subject.split(" ");
				const subjectLower = tmp[0].toLowerCase();
				
				const jwtUser = jwt.verify(session, JWT_SECRET);

				if (typeof jwtUser === 'object') {
					let resultGetSurveyResults: any = {};

					if (jwtUser.survey_results_ids.length > 0 && jwtUser.survey_results_ids[0] !== '') {
						const result = await getRocket(
							`https://api.wtf.danielc.us/survey_result/${jwtUser.survey_results_ids}`
						);
						resultGetSurveyResults = await result.json();
					}
					console.log(resultGetSurveyResults);
	
					const body = JSON.stringify({
						timestamp: Date.now().toString(),
						aesthetics: subjectLower === 'aesthetics' ? [answers] : [''],
						cognitive: subjectLower === 'cognitive' ? [answers] : [''],
						cosmology: subjectLower === 'cosmology' ? [answers] : [''],
						environmental: subjectLower === 'environmental' ? [answers] : [''],
						epistemology: subjectLower === 'epistemology' ? [answers] : [''],
						ethics: subjectLower === 'ethics' ? [answers] : [''],
						history: subjectLower === 'history' ? [answers] : [''],
						isms: subjectLower === 'isms' ? [answers] : [''],
						law: subjectLower === 'law' ? [answers] : [''],
						logic: subjectLower === 'logic' ? [answers] : [''],
						maths: subjectLower === 'maths' ? [answers] : [''],
						ontology: subjectLower === 'ontology' ? [answers] : [''],
						political: subjectLower === 'political' ? [answers] : [''],
						rhetoric: subjectLower === 'rhetoric' ? [answers] : [''],
						science: subjectLower === 'science' ? [answers] : [''],
						theology: subjectLower === 'theology' ? [answers] : ['']
					});
					
					const resultCreateResponse = await postRocket(
						`https://api.wtf.danielc.us/survey_result/create`,
						'new_survey_result',
						body
					);
					const resultCreateResponseJson: any = await resultCreateResponse.json();
	
					console.log(
						resultGetSurveyResults,
						resultCreateResponseJson,
					);
	
					const sessionUser: SessionUser = {
						id: jwtUser.id,
						first_name: jwtUser.first_name,
						account_id: jwtUser.id,
						avatar: jwtUser.avatar,
						level: jwtUser.level,
						preference_ids: jwtUser.preference_ids,
						relationship_ids: jwtUser.relationship_ids,
						survey_results_ids: resultCreateResponseJson.id,
						user_ids: jwtUser.user_ids
					};
	
					const sessionJwt = jwt.sign(sessionUser, JWT_SECRET);
						
					cookies.set('session', sessionJwt, cookieOpts);
				}
			}
		} catch (err: unknown) {
			const message = `Error in /submit form: ${err as string}`;
			console.error(message);
			throw error(500, message);
		}
		throw redirect(303, '/');
	}
};
