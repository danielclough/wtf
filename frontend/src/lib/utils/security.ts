import type { UUID } from "crypto";

export interface SessionUser {
	id: UUID;
	first_name: string;
	account_id: string;
	avatar: string;
	level: 'beta';
	preference_ids: UUID;
	role_ids: UUID;
	sensitivity_ids: UUID;
	survey_results_ids: UUID;
	user_ids: UUID;
}
