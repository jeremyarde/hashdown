import { useEffect, useState } from "react";
import { Survey } from "@/lib/constants";
import { getBaseUrl, getSessionToken, handleResponse } from "@/lib/utils";


type ListSurvey = {
    surveys: Survey[]
};

/**
 * 
 * @returns {Survey[], string, boolean}
 */
export function useListSurveys(): { surveys: ListSurvey | undefined, error: string, isPending: boolean } {
    const [surveys, setSurveys] = useState();
    const [isPending, setIsPending] = useState(false);
    const [error, setError] = useState('');

    const listSurvey = async () => {
        setIsPending(true);
        try {
            const response = await fetch(`${getBaseUrl()}/surveys`, {
                method: "GET",
                headers: {
                    "Content-Type": "application/json",
                    "session_id": getSessionToken()
                },
            });
            console.log(`response from API: ${JSON.stringify(response)}`)
            handleResponse(response);

            if (response.status === 401) {
                setError('Not authorized');
                setIsPending(false);
                return {
                    surveys, error, isPending
                }
            }
            if (response.status === 400) {
                setError('Survey not found');
                setIsPending(false);
                return {
                    surveys, error, isPending
                }
            }
            const data = await response.json();
            setIsPending(false);
            setSurveys(data);
            setError('');
        } catch (error) {
            setIsPending(false);
            setError(`Could not fetch data: ${error}`);
        }
    }

    useEffect(() => {
        listSurvey();
    }, []);

    return {
        surveys, error, isPending
    }
}

