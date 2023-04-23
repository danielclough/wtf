import type { UUID } from "crypto";

export interface SessionUser {
	id: UUID;
	first_name: string;
	account_id: string;
	avatar: string;
	level: 'beta';
	preference_ids: string;
	relationship_ids: string;
	survey_results_ids: string;
	user_ids: string;
}



export const cookieOpts = {
	path: '/',
	httpOnly: false
};