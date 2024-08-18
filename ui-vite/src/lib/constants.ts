import { getStage } from "./utils";

export enum STAGE {
  DEV = "development",
  PROD = "production",
}

export const BASE_URL: { [stage: string]: string } = {
  [STAGE.DEV]: "http://localhost:8080",
  // [STAGE.PROD]: 'https://mdp-api.onrender.com/v1',
  [STAGE.PROD]: "https://api.gethashdown.com",
};

export enum FEATURES {
  WAITLIST,
  LOGIN,
  TESTTABS,
  CHECKOUT,
}

export const EnabledFeatures: { [stage: string]: FEATURES[] } = {
  [STAGE.DEV]: [FEATURES.LOGIN, FEATURES.TESTTABS, FEATURES.CHECKOUT],
  [STAGE.PROD]: [FEATURES.LOGIN],
};

export const SESSION_TOKEN_KEY = "session_id";

export const MARKDOWN_RULES = `# Titles have a '#' at the start

If there is an issue parsing your form, you will see an error like this.
Different questions start with the type of question, a colon ":" and then the question text. The choices for the question are written in the list format under the quesiton. For example:

radio: This is a radio button question
- this is the first radio button
- second radio button

checkbox: This is a checkbox. Multiple options can be selected.
- [ ] this starts with "unchecked" default
- [x] this starts out as "checked"
- option three will also be unchecked

textarea: This is for longer paragraphs of text

text: This is for single lines of text. Like a username, email, etc

submit: This button sends the response. What you type here will be on the button
`;

// export interface Survey {
//     survey_id: string;
//     created_at: string;
//     blocks: Block[]
//     id: string
//     parse_version: string
//     plaintext: string
//     questions: any[]
//     title: string
// }

// export interface Block {
//     block_type: string
//     id: string
//     index: number
//     properties: Properties
// }

// export interface Properties {
//     title?: string
//     type: string
//     options?: any[]
//     question?: string
//     text?: string
// }

export interface Root {
  blocks: Block[];
  id: string;
  parse_version: string;
  plaintext: string;
  questions: any[];
  title: string;
}

export interface Block {
  block_type: string;
  id: string;
  index: number;
  properties: Properties;
}

export interface Properties {
  title?: string;
  type: string;
  default?: string;
  id?: string;
  question?: string;
  options?: any[];
  button?: string;
}

export const styleTokens = {
  /* CSS HSL */
  blue: "hsla(229, 22%, 29%, 1)",
  pink: "hsla(26, 80%, 88%, 1)",
  powderBlue: "hsla(218, 50%, 73%, 1)",
  melon: "hsla(10, 61%, 78%, 1)",
  umber: "hsla(20, 21%, 34%, 1)",
};
