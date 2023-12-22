import React, { useContext, useEffect, useState } from 'react';
import { useSearchParams } from 'react-router-dom';
import { BASE_URL } from './lib/constants.ts';
import { Table, TableBody, TableCaption, TableCell, TableHead, TableHeader, TableRow } from './components/ui/table.tsx';
import { GlobalState, GlobalStateContext, SurveyResponse } from './main.tsx';
import { useGetSurvey } from './hooks/useGetSurvey.ts';
import { createTable } from './createTable.tsx';

export function ListResponses() {
    const [surveyResponses, setSurveyResponses] = useState([]);
    const [queryParams, setQueryParams] = useSearchParams();
    let globalState: GlobalState = useContext(GlobalStateContext);
    const SURVEY_ID_QUERY_KEY = "survey_id";
    let { survey, error, isPending } = useGetSurvey(queryParams.get(SURVEY_ID_QUERY_KEY));

    useEffect(() => {
        getResponses(queryParams.get(SURVEY_ID_QUERY_KEY));
    }, [queryParams]);

    async function getResponses(survey_id: string) {
        const response = await fetch(`${BASE_URL}/responses?${new URLSearchParams({
            survey_id: queryParams.get(SURVEY_ID_QUERY_KEY)
        })}`, {
            method: "GET",
            // credentials: 'include',
            headers: {
                'session_id': globalState.sessionId ?? '',
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

    const idToTitle = {};
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



