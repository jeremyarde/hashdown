import { GlobalState } from "@/main";
import { type ClassValue, clsx } from "clsx"
import { twMerge } from "tailwind-merge"
import { SESSION_TOKEN_KEY } from "./constants";

export function cn(...inputs: ClassValue[]) {
  return twMerge(clsx(inputs))
}


export function handleResponse(apiResponse: Response, globalState: GlobalState) {
  switch (apiResponse.status) {
    case 200:
      // const session_header = apiResponse.headers.get(SESSION_TOKEN_KEY);
      // globalState.setSessionId(session_header);
      // window.sessionStorage.setItem(SESSION_TOKEN_KEY, session_header);
      break;
    case 401:
      globalState.setSessionId('');
      break;
  }
  return apiResponse;
}

export async function setSessionToken(apiResponse: Response, globalState: GlobalState) {
  const sessionId = apiResponse.headers.get(SESSION_TOKEN_KEY);
  console.log('session_id', sessionId)
  console.log('response - apiResponse: ', apiResponse);
  window.sessionStorage.setItem(SESSION_TOKEN_KEY, sessionId);
  globalState.setSessionId((curr) => sessionId);
}