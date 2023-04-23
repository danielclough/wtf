import type { SessionUser } from '$lib/utils/session';
import jwt from 'jsonwebtoken';
import { error, type Actions, redirect } from '@sveltejs/kit';
import { JWT_SECRET } from '$env/static/private';
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

				const jwtUser = jwt.verify(session, JWT_SECRET);

				let resultGetSurveyResults: any = {
					aesthetics: "",
					cognitive: "",
					cosmology: "",
					environmental: "",
					epistemology: "",
					ethics: "",
					history: "",
					isms: "",
					law: "",
					logic: "",
					maths: "",
					ontology: "",
					political: "",
					rhetoric: "",
					science: "",
					theology: "",
				};
				if (typeof jwtUser === 'object') {

					if (jwtUser.survey_results_ids.length > 0 && jwtUser.survey_results_ids !== '') {
					console.log("jwtUser", jwtUser.survey_results_ids[0]);
						const result = await getRocket(
							`https://api.wtf.danielc.us/survey_result/${jwtUser.survey_results_ids[0]}`
						);
						resultGetSurveyResults = await result.json();
					}
	
					const timestamp = Date.now().toString();
					const body = {
						timestamp,
						aesthetics: resultGetSurveyResults.aesthetics[0] !== "" && subject === 'aesthetics'
							?  [...resultGetSurveyResults.aesthetics, answers]
								?  subject === 'aesthetics'
									: [answers]
									: [''],
						cognitive: resultGetSurveyResults.cognitive[0] !== "" && subject === 'cognitive'
							?  [...resultGetSurveyResults.cognitive, answers]
								?  subject === 'cognitive'
									: [answers]
									: [''],
						cosmology: resultGetSurveyResults.cosmology[0] !== "" && subject === 'cosmology'
							?  [...resultGetSurveyResults.cosmology, answers]
								?  subject === 'cosmology'
									: [answers]
									: [''],
						environmental: resultGetSurveyResults.environmental[0] !== "" && subject === 'environmental'
							?  [...resultGetSurveyResults.environmental, answers]
								?  subject === 'environmental'
									: [answers]
									: [''],
						epistemology: resultGetSurveyResults.epistemology[0] !== "" && subject === 'epistemology'
							?  [...resultGetSurveyResults.epistemology, answers]
								?  subject === 'epistemology'
									: [answers]
									: [''],
						ethics: resultGetSurveyResults.ethics[0] !== "" && subject === 'ethics'
							?  [...resultGetSurveyResults.ethics, answers]
								?  subject === 'ethics'
									: [answers]
									: [''],
						history: resultGetSurveyResults.history[0] !== "" && subject === 'history'
							?  [...resultGetSurveyResults.history, answers]
								?  subject === 'history'
									: [answers]
									: [''],
						isms: resultGetSurveyResults.isms[0] !== "" && subject === 'isms'
							?  [...resultGetSurveyResults.isms, answers]
								?  subject === 'isms'
									: [answers]
									: [''],
						law: resultGetSurveyResults.law[0] !== "" && subject === 'law'
							?  [...resultGetSurveyResults.law, answers]
								?  subject === 'law'
									: [answers]
									: [''],
						logic: resultGetSurveyResults.logic[0] !== "" && subject === 'logic'
							?  [...resultGetSurveyResults.logic, answers]
								?  subject === 'logic'
									: [answers]
									: [''],
						maths: resultGetSurveyResults.maths[0] !== "" && subject === 'maths'
							?  [...resultGetSurveyResults.maths, answers]
								?  subject === 'maths'
									: [answers]
									: [''],
						ontology: resultGetSurveyResults.ontology[0] !== "" && subject === 'ontology'
							?  [...resultGetSurveyResults.ontology, answers]
								?  subject === 'ontology'
									: [answers]
									: [''],
						political: resultGetSurveyResults.political[0] !== "" && subject === 'political'
							?  [...resultGetSurveyResults.political, answers]
								?  subject === 'political'
									: [answers]
									: [''],
						rhetoric: resultGetSurveyResults.rhetoric[0] !== "" && subject === 'rhetoric'
							?  [...resultGetSurveyResults.rhetoric, answers]
								?  subject === 'rhetoric'
									: [answers]
									: [''],
						science: resultGetSurveyResults.science[0] !== "" && subject === 'science'
							?  [...resultGetSurveyResults.science, answers]
								?  subject === 'science'
									: [answers]
									: [''],
						theology: resultGetSurveyResults.theology[0] !== "" && subject === 'theology'
							?  [...resultGetSurveyResults.theology, answers]
								?  subject === 'theology'
									: [answers]
									: [''],
					};
					console.log(body);
					const resultCreateResponse = await postRocket(
						`https://api.wtf.danielc.us/survey_result/create`,
						'new_survey_result',
						JSON.stringify(body)
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
	
					cookies.set('session', sessionJwt);
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
