import { getStage } from "./utils";

export enum STAGE {
    DEV = 'development',
    PROD = 'production',
};

export const BASE_URL: { [stage: string]: string } = {
    [STAGE.DEV]: 'http://localhost:8080/v1',
    // [STAGE.PROD]: 'https://mdp-api.onrender.com/v1',
    [STAGE.PROD]: 'https://api.gethashdown.com/v1',
};

export enum FEATURES {
    WAITLIST,
    LOGIN
};

export const EnabledFeatures: { [stage: string]: FEATURES[] } = {
    [STAGE.DEV]: [FEATURES.WAITLIST, FEATURES.LOGIN],
    [STAGE.PROD]: [FEATURES.WAITLIST]
};

export const SESSION_TOKEN_KEY = 'session_id';

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