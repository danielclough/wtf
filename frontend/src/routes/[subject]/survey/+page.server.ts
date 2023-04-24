import { cookieOpts, type SessionUser } from '$lib/utils/session';
import jwt from 'jsonwebtoken';
import { JWT_SECRET } from '$env/static/private';
import { error, type Actions, redirect } from '@sveltejs/kit';

import { putRocket, getRocket, postRocket } from '$lib/utils/rocket';

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
					// console.log("jwtUser", jwtUser);

					if (jwtUser.survey_results_id !== '') {
						const result = await getRocket(
							`https://api.wtf.danielc.us/survey_result/${jwtUser.survey_results_id}`
						);
						resultGetSurveyResults = await result.json();
					}
					// console.log('resultGetSurveyResults', resultGetSurveyResults);
	
					const updateOrNot = (current: string, subject: string, updateString: string, objToUpdate: any) => {
						console.log(objToUpdate[current], current, subject, updateString);
						if (objToUpdate[current].length > 1 && current == subject) {
							return [objToUpdate[current][0], updateString];
						} else if (objToUpdate[current].length > 0) {
							return [...objToUpdate[current]];
						} else if (current == subject) {
							return [updateString];
						} else {
							return [];
						}
					}

					const body = JSON.stringify({
						timestamp: Date.now().toString(),
						aesthetics: updateOrNot('aesthetics', subjectLower, answers, resultGetSurveyResults),
						cognitive: updateOrNot('cognitive', subjectLower, answers, resultGetSurveyResults),
						cosmology: updateOrNot('cosmology', subjectLower, answers, resultGetSurveyResults),
						environmental: updateOrNot('environmental', subjectLower, answers, resultGetSurveyResults),
						epistemology: updateOrNot('epistemology', subjectLower, answers, resultGetSurveyResults),
						ethics: updateOrNot('ethics', subjectLower, answers, resultGetSurveyResults),
						history: updateOrNot('history', subjectLower, answers, resultGetSurveyResults),
						isms: updateOrNot('isms', subjectLower, answers, resultGetSurveyResults),
						law: updateOrNot('law', subjectLower, answers, resultGetSurveyResults),
						logic: updateOrNot('logic', subjectLower, answers, resultGetSurveyResults),
						maths: updateOrNot('maths', subjectLower, answers, resultGetSurveyResults),
						ontology: updateOrNot('ontology', subjectLower, answers, resultGetSurveyResults),
						political: updateOrNot('political', subjectLower, answers, resultGetSurveyResults),
						rhetoric: updateOrNot('rhetoric', subjectLower, answers, resultGetSurveyResults),
						science: updateOrNot('science', subjectLower, answers, resultGetSurveyResults),
						theology: updateOrNot('theology', subjectLower, answers, resultGetSurveyResults),
					});
					
					// GET ACCOUNT
					const resultGetAccount = await getRocket(
						`https://api.wtf.danielc.us/account/${jwtUser.account_id}`
					);
					const resultGetAccountJson = await resultGetAccount.json();
					const accountId = resultGetAccountJson.id;
					delete resultGetAccountJson.id;
					resultGetAccountJson.survey_results_ids = [resultGetSurveyResults.id];

					// CHECK FOR EXISTING SURVEY RESULTS (PUT/POST)
					let resultCreateResponse:any = {};
					if (!!resultGetSurveyResults.id) {
						resultCreateResponse = await putRocket(
							`https://api.wtf.danielc.us/survey_result/${resultGetSurveyResults.id}`,
							'new_survey_result',
							body
						);
					} else {
						resultCreateResponse = await postRocket(
							`https://api.wtf.danielc.us/survey_result/create`,
							'new_survey_result',
							body
						);
					}
					const resultCreateResponseJson: any = await resultCreateResponse.json();

					
					// UPDATE ACCOUNT
					const resultAccountResponse = await putRocket(
						`https://api.wtf.danielc.us/account/${accountId}`,
						'new_account',
						JSON.stringify(resultGetAccountJson)
					);
					
					const sessionUser: SessionUser = {
						id: jwtUser.id,
						first_name: jwtUser.first_name,
						account_id: accountId,
						avatar: jwtUser.avatar,
						level: jwtUser.level,
						preference_id: jwtUser.preference_ids,
						relationship_id: jwtUser.relationship_ids,
						survey_results_id: resultCreateResponseJson.id,
						user_id: jwtUser.user_id
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
