import { useEffect, useState } from 'react';
import { useSearchParams } from 'react-router-dom';
import { useGetSurvey } from './hooks/useGetSurvey.ts';
import { createTable } from './createTable.tsx';
import { getBaseUrl, getSessionToken } from './lib/utils.ts';


export function ListResponses() {
    const [surveyResponses, setSurveyResponses] = useState([]);
    const [queryParams, setQueryParams] = useSearchParams();
    // let globalState: GlobalState = useContext(GlobalStateContext);
    const SURVEY_ID_QUERY_KEY = "survey_id";
    let { survey, error, isPending } = useGetSurvey(queryParams.get(SURVEY_ID_QUERY_KEY) || '');

    useEffect(() => {
        getResponses(queryParams.get(SURVEY_ID_QUERY_KEY) || '');
    }, [queryParams]);

    async function getResponses(survey_id: string) {
        if (!survey_id) {
            return;
        }

        const response = await fetch(`${getBaseUrl()}/responses?${new URLSearchParams({
            survey_id: queryParams.get(SURVEY_ID_QUERY_KEY) || ''
        })}`, {
            method: "GET",
            // credentials: 'include',
            headers: {
                'session_id': getSessionToken(),
                // 'Content-Type': 'application/json'
            },
        });

        const result = await response.json();
        console.log('data: ', result);
        if (result.error) {
            console.log('failed to get surveys: ', result);
            // setError(result.message ?? 'Generic error getting surveys');
            if (response.status === 401) {
                // redirect({ to: "/login", replace: true });
            }
        } else {
            console.log('Found surveys: ', result);
            setSurveyResponses(result["responses"]);
            // setError('');
        }
    }

    const idToTitle: { [id: string]: string } = {};
    let columns = ['ID', 'Submitted at'];
    survey?.blocks.forEach((block) => {
        if (block.properties.question) {
            idToTitle[block.id] = block.properties.question;
            columns.push(block.properties.question);
        }
    });

    return (
        <>
            <div>
                {createTable(columns, ['id', 'submitted_at', ...Object.keys(idToTitle).map((key) => 'answers.' + key)], surveyResponses)}
            </div>
        </>
    );
}



