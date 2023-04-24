import type { UUID } from "crypto";

export interface SessionUser {
	id: UUID;
	first_name: string;
	account_id: string;
	avatar: string;
	level: 'beta';
	preference_id: string;
	relationship_id: string;
	survey_results_id: string;
	user_id: string;
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
	sameSite: 'lax',
	secure: true,
	maxAge: 60 * 60 * 24 * 7
};