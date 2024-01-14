import { useEffect, useState } from "react";
import { markdown_to_form_wasm_v2 } from "../../../backend/pkg/markdownparser";
import { Survey } from "@/lib/constants";
import { getBaseUrl, getSessionToken, handleResponse } from "@/lib/utils";

type LoginResult = {
    email: string
};

export function useLogin(): { result: LoginResult | undefined, error: string, isPending: boolean } {
    const [result, setResult] = useState();
    const [isPending, setIsPending] = useState(false);
    const [error, setError] = useState('');

    // this can be used to login for the first time 
    // to retrieve the session token, or to refresh 
    //the current token
    const getResult = async () => {
        setIsPending(true);
        try {
            const response = await fetch(`${getBaseUrl()}/login}`, {
                method: "GET",
                headers: {
                    "Content-Type": "application/json",
                    // "session_id": getSessionToken()
                },
                // credentials: 'include',
            });
            console.log(`response from API: ${JSON.stringify(response)}`)
            handleResponse(response);

            if (response.status === 401) {
                setError('Not authorized');
                return
            }
            if (response.status === 400) {
                setError('Survey not found');
                return
            }
            const data = await response.json();
            setIsPending(false);
            setResult(data);
            setError('');
        } catch (error) {
            setIsPending(false);
            setError(`Could not fetch data: ${error}`);
        }
    }

    useEffect(() => {
        getResult();
    }, []);

    return {
        result, error, isPending
    }
}