// import { GlobalState } from "@/main";
import { type ClassValue, clsx } from "clsx"
import { twMerge } from "tailwind-merge"
import { SESSION_TOKEN_KEY } from "./constants";

export function cn(...inputs: ClassValue[]) {
  return twMerge(clsx(inputs))
}


export function handleResponse(apiResponse: Response) {
  switch (apiResponse.status) {
    case 200:
      break;
    case 401:
      // if (globalState.setSessionId) {
      //   globalState.setSessionId('');
      // }
      window.sessionStorage.setItem(SESSION_TOKEN_KEY, '');
      break;
  }
  return apiResponse;
}

export async function setSessionToken(apiResponse: Response) {
  const sessionId = apiResponse.headers.get(SESSION_TOKEN_KEY) || '';
  console.log('session_id', sessionId)
  console.log('response - apiResponse: ', apiResponse);
  window.sessionStorage.setItem(SESSION_TOKEN_KEY, sessionId);
  // if (globalState.setSessionId) {
  //   globalState.setSessionId(sessionId);
  // }
}

export function getSessionToken(): string {
  return window.sessionStorage.getItem(SESSION_TOKEN_KEY) || '';
}