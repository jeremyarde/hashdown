import { type ClassValue, clsx } from "clsx";
import {
  BASE_URL,
  EnabledFeatures,
  FEATURES,
  SESSION_TOKEN_KEY,
  STAGE,
} from "./constants";

export function handleResponse(apiResponse: Response) {
  switch (apiResponse.status) {
    case 200:
      break;
    case 401:
      window.sessionStorage.setItem(SESSION_TOKEN_KEY, "");
      break;
    case 403:
      window.sessionStorage.setItem(SESSION_TOKEN_KEY, "");
      break;
    default:
      break;
  }
  return apiResponse;
}

export async function setSessionToken(apiResponse: Response) {
  const sessionId = apiResponse.headers.get(SESSION_TOKEN_KEY) || "";
  window.sessionStorage.setItem(SESSION_TOKEN_KEY, sessionId);
}

export function getSessionToken(): string {
  return window.sessionStorage.getItem(SESSION_TOKEN_KEY) || "";
}

export function getApiBaseUrl(): string {
  let stage: string = import.meta.env.MODE || STAGE.DEV;
  return BASE_URL[stage];
}

export function getWebsiteUrl(): string {
  return window.location.host;
}

export function getStage(): STAGE {
  return import.meta.env.MODE === "production" ? STAGE.PROD : STAGE.DEV;
}

export function isDev(): boolean {
  return getStage() === STAGE.DEV;
}

export function isFeatureEnabled(feature: FEATURES): boolean {
  return EnabledFeatures[getStage()].includes(feature);
}

export async function logout() {
  try {
    const response = await fetch(`${getApiBaseUrl()}/v1/auth/logout`, {
      method: "GET",
      headers: {
        "Content-Type": "application/json",
        session_id: getSessionToken(),
      },
    });
    handleResponse(response);
    window.sessionStorage.removeItem(SESSION_TOKEN_KEY);
  } catch (error) {}
}
