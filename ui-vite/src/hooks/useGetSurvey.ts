import { BASE_URL } from "@/lib/constants";
import { GlobalState, GlobalStateContext } from "@/main";
import { useContext, useEffect, useState } from "react";
import { markdown_to_form_wasm_v2 } from "../../../backend/pkg/markdownparser";

export function useGetSurvey(surveyId: string) {
    let globalState: GlobalState = useContext(GlobalStateContext);
    const [survey, setSurvey] = useState();
    const [isPending, setIsPending] = useState(false);
    const [error, setError] = useState(null);

    const getSurvey = async (getSurveyId) => {
        setIsPending(true);
        try {

            const response = await fetch(`${BASE_URL}/surveys/${getSurveyId}`, {
                method: "GET",
                headers: {
                    "Content-Type": "application/json",
                    "session_id": `${globalState.sessionId}`
                },
                credentials: 'include',
            });
            console.log(`response from API: ${JSON.stringify(response)}`)
            if (response.status === 401) {
                setError('Not authorized');
                return
            }
            const data = await response.json();
            const fullSurvey = {
                ...data,
                questions: markdown_to_form_wasm_v2(data.plaintext)
            }
            setIsPending(false);
            setSurvey(fullSurvey);
            setError(null);
        } catch (error) {
            setIsPending(false);
            setError(`Could not fetch data: ${error}`);
        }
    }

    useEffect(() => {
        getSurvey(surveyId);
    }, []);

    return { survey, error, isPending }
}