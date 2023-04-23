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

interface CookieOpts {
	path: string;
	httpOnly: boolean;
	sameSite: boolean | "strict" | "lax" | "none" | undefined;
	secure: boolean;
	maxAge: number;
}

export const cookieOpts: CookieOpts = {
	path: '/',
	httpOnly: true,
	sameSite: 'strict',
	secure: false,
	maxAge: 60 * 60 * 24 * 7
};