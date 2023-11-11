import React, { useContext, useEffect, useState } from 'react';
import { useSearchParams } from 'react-router-dom';
import { BASE_URL } from './lib/constants.ts';
import { Table, TableBody, TableCaption, TableCell, TableHead, TableHeader, TableRow } from './components/ui/table.tsx';
import { GlobalState, GlobalStateContext, SurveyResponse } from './main.tsx';

export function ListResponses() {
    const [surveyResponses, setSurveyResponses] = useState([]);
    const [queryParams, setQueryParams] = useSearchParams();
    let globalState: GlobalState = useContext(GlobalStateContext);
    const SURVEY_ID_QUERY_KEY = "survey_id";

    console.log(JSON.stringify(queryParams.get(SURVEY_ID_QUERY_KEY)));

    useEffect(() => {
        getResponses(queryParams.get(SURVEY_ID_QUERY_KEY));
    }, [queryParams]);

    async function getResponses(survey_id: string) {
        const response = await fetch(`${BASE_URL}/responses?${new URLSearchParams({
            survey_id: queryParams.get(SURVEY_ID_QUERY_KEY)
        })}`, {
            method: "GET",
            credentials: 'include',
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

    return (
        <>
            SurveyID: {queryParams.get(SURVEY_ID_QUERY_KEY)}
            # answers: {surveyResponses.length}
            <Table className='text-left'>
                <TableCaption></TableCaption>
                <TableHeader>
                    <TableRow>
                        <TableHead className="w-[100px]">ID</TableHead>
                        <TableHead className="">Submitted at</TableHead>
                        <TableHead className="">Survey ID</TableHead>
                        <TableHead className="">Answers</TableHead>
                    </TableRow>
                </TableHeader>
                <TableBody className=''>
                    {surveyResponses.map((surveyResponse: SurveyResponse) => {
                        // console.log(surveyResponse)
                        return (
                            <>
                                <TableRow className='outline outline-1 outline-gray-300 hover:bg-blue-100'>
                                    <TableCell className="font-medium">{`${surveyResponse.id}`}</TableCell>
                                    <TableCell className="font-medium">{surveyResponse.submitted_at}</TableCell>
                                    <TableCell className="font-medium">{surveyResponse.survey_id}</TableCell>
                                    <TableCell className="font-medium">{JSON.stringify(surveyResponse.answers)}</TableCell>
                                </TableRow>
                            </>
                        );
                    })}
                </TableBody>
            </Table>
            <hr></hr>
            <div>
                {/* {ListResponsesV2()} */}
            </div>
        </>
    );
}


function ListResponsesV2() {
    return (
        <div className="space-y-6">
            <form className="relative">
                <svg
                    xmlns="http://www.w3.org/2000/svg"
                    width="24"
                    height="24"
                    viewBox="0 0 24 24"
                    fill="none"
                    stroke="currentColor"
                    stroke-width="2"
                    stroke-linecap="round"
                    stroke-linejoin="round"
                    className=" absolute left-2.5 top-2.5 h-4 w-4 text-zinc-500 dark:text-zinc-400"
                >
                    <circle cx="11" cy="11" r="8"></circle>
                    <path d="m21 21-4.3-4.3"></path>
                </svg>
                <input
                    className="flex h-10 w-full rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background file:border-0 file:bg-transparent file:text-sm file:font-medium placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50 pl-8"
                    placeholder="Search respondents..."
                    type="search"
                />
            </form>
            <div className="border rounded-lg w-full">
                <div className="relative w-full overflow-auto">
                    <div className="w-full overflow-auto">
                        <table className="w-full caption-bottom text-sm">
                            <thead className="[&amp;_tr]:border-b">
                                <tr className="border-b transition-colors hover:bg-muted/50 data-[state=selected]:bg-muted">
                                    <th className="h-12 px-4 text-left align-middle font-medium text-muted-foreground [&amp;:has([role=checkbox])]:pr-0 w-[200px]">
                                        Name
                                    </th>
                                    <th className="h-12 px-4 text-left align-middle font-medium text-muted-foreground [&amp;:has([role=checkbox])]:pr-0 w-[300px]">
                                        Email
                                    </th>
                                    <th className="h-12 px-4 text-left align-middle font-medium text-muted-foreground [&amp;:has([role=checkbox])]:pr-0 w-[100px]">
                                        Age
                                    </th>
                                    <th className="h-12 px-4 text-left align-middle font-medium text-muted-foreground [&amp;:has([role=checkbox])]:pr-0">
                                        Answer 1
                                    </th>
                                    <th className="h-12 px-4 text-left align-middle font-medium text-muted-foreground [&amp;:has([role=checkbox])]:pr-0">
                                        Answer 2
                                    </th>
                                    <th className="h-12 px-4 text-left align-middle font-medium text-muted-foreground [&amp;:has([role=checkbox])]:pr-0">
                                        Answer 3
                                    </th>
                                </tr>
                            </thead>
                            <tbody className="">
                                <tr className="border-b transition-colors hover:bg-muted/50 data-[state=selected]:bg-muted">
                                    <td className="p-4 align-middle [&amp;:has([role=checkbox])]:pr-0 font-medium">John Doe</td>
                                    <td className="p-4 align-middle [&amp;:has([role=checkbox])]:pr-0">johndoe@example.com</td>
                                    <td className="p-4 align-middle [&amp;:has([role=checkbox])]:pr-0">25</td>
                                    <td className="p-4 align-middle [&amp;:has([role=checkbox])]:pr-0">Yes</td>
                                    <td className="p-4 align-middle [&amp;:has([role=checkbox])]:pr-0">No</td>
                                    <td className="p-4 align-middle [&amp;:has([role=checkbox])]:pr-0">Maybe</td>
                                </tr>
                                <tr className="border-b transition-colors hover:bg-muted/50 data-[state=selected]:bg-muted">
                                    <td className="p-4 align-middle [&amp;:has([role=checkbox])]:pr-0 font-medium">Jane Doe</td>
                                    <td className="p-4 align-middle [&amp;:has([role=checkbox])]:pr-0">janedoe@example.com</td>
                                    <td className="p-4 align-middle [&amp;:has([role=checkbox])]:pr-0">30</td>
                                    <td className="p-4 align-middle [&amp;:has([role=checkbox])]:pr-0">No</td>
                                    <td className="p-4 align-middle [&amp;:has([role=checkbox])]:pr-0">Yes</td>
                                    <td className="p-4 align-middle [&amp;:has([role=checkbox])]:pr-0">Maybe</td>
                                </tr>
                                <tr className="border-b transition-colors hover:bg-muted/50 data-[state=selected]:bg-muted">
                                    <td className="p-4 align-middle [&amp;:has([role=checkbox])]:pr-0 font-medium">Alex Smith</td>
                                    <td className="p-4 align-middle [&amp;:has([role=checkbox])]:pr-0">alexsmith@example.com</td>
                                    <td className="p-4 align-middle [&amp;:has([role=checkbox])]:pr-0">35</td>
                                    <td className="p-4 align-middle [&amp;:has([role=checkbox])]:pr-0">Maybe</td>
                                    <td className="p-4 align-middle [&amp;:has([role=checkbox])]:pr-0">No</td>
                                    <td className="p-4 align-middle [&amp;:has([role=checkbox])]:pr-0">Yes</td>
                                </tr>
                            </tbody>
                        </table>
                    </div>
                </div>
            </div>
        </div>
    )
}
