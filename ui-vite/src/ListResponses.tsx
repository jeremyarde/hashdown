import React, { useContext, useEffect, useState } from 'react';
import { useSearchParams } from 'react-router-dom';
import { BASE_URL } from './lib/constants.ts';
import { Table, TableBody, TableCaption, TableCell, TableHead, TableHeader, TableRow } from './components/ui/table.tsx';
import { GlobalState, GlobalStateContext, SurveyResponse } from './main.tsx';
import { useGetSurvey } from './hooks/useGetSurvey.ts';

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
                {ListResponsesV2(columns, ['id', 'submitted_at', ...Object.keys(idToTitle).map((key) => 'answers.' + key)], surveyResponses)}
            </div>
        </>
    );
}


function ListResponsesV2(cols = [], dataKeys = [], data = []) {
    return (
        <div className="space-y-6">
            <form className="relative">
                <input
                    className="flex h-10 w-full rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background file:border-0 file:bg-transparent file:text-sm file:font-medium placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50 pl-8"
                    placeholder="Search respondents..."
                    type="search"
                />
            </form>
            <div className="w-full border">
                <div className='grid justify-start items-center w-full' style={{
                    gridTemplateColumns: `repeat(${cols.length}, 2fr)`
                }}>
                    {cols.map(colName => {
                        return (
                            <div className='w-full h-full border bg-yellow'>{colName}</div>
                        )
                    })}
                    {data.map(dataItem => {
                        return (
                            <>
                                {dataKeys.map((dataKey) => {
                                    let nested = dataKey.split('.');
                                    let value = dataItem;
                                    nested.forEach(key => {
                                        value = value[key];
                                    });
                                    return (
                                        <div className='h-full w-full border p-1 hover:bg-green'>{value ?? '-'}</div>
                                    )
                                })}
                            </>
                        )
                    })}
                </div>
            </div>
        </div >
    )
}
