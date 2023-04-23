export interface Survey {
    title: string;
	image: string;
	copyright: Copyright;
    api: string;
	responses: string[];
	questions: SurveyQuestions[];
}

export interface SurveyQuestion {
	id: number;
	text: string;
	type: [string];
}

export interface Copyright {
    title: string;
    date: string;
    rights: string;
    cite: string[];
    alt: string[];
}

export interface SurveyQuestions {
    topic: string;
    questions: SurveyQuestion[];
}
