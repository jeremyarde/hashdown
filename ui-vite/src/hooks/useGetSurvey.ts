import { useEffect, useState } from "react";
import { markdown_to_form_wasm_v2 } from "../../../backend/pkg/markdownparser";
import { Survey } from "@/lib/constants";
import { getBaseUrl, getSessionToken } from "@/lib/utils";


/**
 * 
 * @param surveyId 
 * @returns {Survey, string, boolean}
 */
export function useGetSurvey(surveyId: string | undefined): { survey: Survey | undefined, error: string, isPending: boolean } {
    // let globalState: GlobalState = useContext(GlobalStateContext);
    const [survey, setSurvey] = useState();
    const [isPending, setIsPending] = useState(false);
    const [error, setError] = useState('');

    const getSurvey = async (getSurveyId: string | undefined) => {
        if (!getSurveyId) {
            return;
        }

        setIsPending(true);
        try {
            const response = await fetch(`${getBaseUrl()}/surveys/${getSurveyId}`, {
                method: "GET",
                headers: {
                    "Content-Type": "application/json",
                    "session_id": getSessionToken()
                },
                // credentials: 'include',
            });
            console.log(`response from API: ${JSON.stringify(response)}`)
            if (response.status === 401) {
                setError('Not authorized');
                return
            }
            if (response.status === 400) {
                setError('Survey not found');
                return
            }
            const data = await response.json();
            const fullSurvey = {
                ...data,
                questions: markdown_to_form_wasm_v2(data.plaintext)
            }
            setIsPending(false);
            setSurvey(fullSurvey);
            setError('');
        } catch (error) {
            setIsPending(false);
            setError(`Could not fetch data: ${error}`);
        }
    }

    useEffect(() => {
        getSurvey(surveyId);
    }, []);

    return {
        survey, error, isPending
    }
}