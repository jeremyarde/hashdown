export const BASE_URL = 'http://localhost:8080/v1';
export const SESSION_TOKEN_KEY = 'session_id';
// export type Survey = {
//     title: string;
//     parse_version: string;
//     id: string;
//     created_at: string;
//     survey_id: string;
//     questions: any[];
//     blocks: 
// };

// export type Survey {
//     blocks: Block[]
//     id: string
//     parse_version: string
//     plaintext: string
//     questions: any[]
//     title: string
// }


export type RenderedFormProps = {
    // plaintext: string;
    survey: object;
    mode: "test" | "prod"
}

export interface Survey {
    survey_id: string;
    created_at: string;
    blocks: Block[]
    id: string
    parse_version: string
    plaintext: string
    questions: any[]
    title: string
}

export interface Block {
    block_type: string
    id: string
    index: number
    properties: Properties
}

export interface Properties {
    title?: string
    type: string
    options?: any[]
    question?: string
    text?: string
}