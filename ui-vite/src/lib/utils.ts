// import { GlobalState } from "@/main";
import { type ClassValue, clsx } from "clsx"
import { twMerge } from "tailwind-merge"
import { BASE_URL, EnabledFeatures, FEATURES, SESSION_TOKEN_KEY, STAGE } from "./constants";

export function cn(...inputs: ClassValue[]) {
  return twMerge(clsx(inputs))
}


export function handleResponse(apiResponse: Response) {
  switch (apiResponse.status) {
    case 200:
      break;
    case 401:
      window.sessionStorage.setItem(SESSION_TOKEN_KEY, '');
      break;
    default:
      window.sessionStorage.removeItem(SESSION_TOKEN_KEY);
      break;
  }
  return apiResponse;
}

export async function setSessionToken(apiResponse: Response) {
  const sessionId = apiResponse.headers.get(SESSION_TOKEN_KEY) || '';
  console.log('session_id', sessionId)
  console.log('response - apiResponse: ', apiResponse);
  window.sessionStorage.setItem(SESSION_TOKEN_KEY, sessionId);
}

export function getSessionToken(): string {
  return window.sessionStorage.getItem(SESSION_TOKEN_KEY) || '';
}

export function getBaseUrl(): string {
  let stage: string = import.meta.env.MODE || STAGE.DEV;

  return BASE_URL[stage];
}

export function getStage(): STAGE {
  return import.meta.env.MODE === 'production' ? STAGE.PROD : STAGE.DEV;
}

export function isDev(): boolean {
  return getStage() === STAGE.DEV;
}

export function isFeatureEnabled(feature: FEATURES): boolean {
  return EnabledFeatures[getStage()].includes(feature);
}

// create a nicer interface for the rust api...
export async function logout() {
  try {
    const response = await fetch(`${getBaseUrl()}/auth/logout`, {
      method: "GET",
      headers: {
        "Content-Type": "application/json",
        "session_id": getSessionToken()
      },
    });
    console.log(`response from API: ${JSON.stringify(response)}`)
    handleResponse(response);

    const data = await response.json();
    window.sessionStorage.removeItem(SESSION_TOKEN_KEY);
  } catch (error) {
    // setIsPending(false);
    // setError(`Could not fetch data: ${error}`);
  }

}
