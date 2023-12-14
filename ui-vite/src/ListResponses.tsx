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

    console.log(JSON.stringify(queryParams.get(SURVEY_ID_QUERY_KEY)));

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

    // const testanswers = {
    //     "responses": [
    //         {
    //             "answers":
    //                 { "e3tchx2yha83": "third radio", "e7z6l6p1z17w_0": "on", "e7z6l6p1z17w_1": "on", "jznm1ytwrnnn": "fasdfffdfadjj", "ptiumgvmkcqc": "asdfasdfasdf" }, "id": 1, "submitted_at": "2023-12-03T05:50:11.394114Z", "survey_id": "nh7bvpfssi2n"
    //         },
    //         {
    //             "answers": { "e3tchx2yha83": "radio button", "e7z6l6p1z17w_0": "on", "jznm1ytwrnnn": "fasdfasf", "ptiumgvmkcqc": "officla response" }, "id": 2, "submitted_at": "2023-12-03T17:47:23.967877Z", "survey_id": "nh7bvpfssi2n"
    //         }, { "answers": { "e7z6l6p1z17w_0": "on", "jznm1ytwrnnn": "", "ptiumgvmkcqc": "" }, "id": 3, "submitted_at": "2023-12-04T00:00:33.966446Z", "survey_id": "nh7bvpfssi2n" }, { "answers": { "e7z6l6p1z17w_0": "on", "jznm1ytwrnnn": "", "ptiumgvmkcqc": "" }, "id": 4, "submitted_at": "2023-12-04T03:37:24.056721Z", "survey_id": "nh7bvpfssi2n" }]
    // };

    // const testsurvey = {
    //     "blocks":
    //         [
    //             { "block_type": "Title", "id": "gb9645iuy91f", "index": 0.0, "properties": { "title": "User Registration Form", "type": "Title" } }, { "block_type": "TextInput", "id": "ptiumgvmkcqc", "index": 0.0, "properties": { "question": "First name [John Dog", "type": "TextInput" } }, { "block_type": "TextInput", "id": "jznm1ytwrnnn", "index": 0.0, "properties": { "question": "Email Address [john@dog.com", "type": "TextInput" } }, { "block_type": "Textarea", "id": "yng98zrklbjx", "index": 0.0, "properties": { "question": "Textarea: This is nice [Enter your comments here]", "type": "Textarea" } }, { "block_type": "Checkbox", "id": "e7z6l6p1z17w", "index": 0.0, "properties": { "options": [{ "checked": true, "text": "Subscribe to newsletter" }, { "checked": false, "text": "second value here" }], "question": " subscribe?", "type": "Checkbox" } }, { "block_type": "Radio", "id": "e3tchx2yha83", "index": 0.0, "properties": { "options": ["radio button", "another one", "third radio"], "question": " my radio", "type": "Radio" } }, { "block_type": "Dropdown", "id": "j6ktc38dgjy9", "index": 0.0, "properties": { "options": ["Option 1", "Option 2", "Option 3"], "question": " My question here", "type": "Dropdown" } }, { "block_type": "Submit", "id": "5rl52gxjj7x4", "index": 0.0, "properties": { "text": "[Submit]", "type": "Submit" } }, { "block_type": "Empty", "id": "tidtg9dk4wu2", "index": 0.0, "properties": { "type": "Nothing" } }
    //         ],
    //     "created_at": "2023-12-03T04:08:28.289715Z", "id": 4, "modified_at": "2023-12-03T04:08:28.289716Z", "name": "name - todo", "parse_version": "2", "plaintext": "# User Registration Form\n\nText: First name [John Dog]\n\nText: Email Address [john@dog.com]\n\nTextarea: This is nice [Enter your comments here]\n\ncheckbox: subscribe?\n- [x] Subscribe to newsletter\n- [ ] second value here\n\nradio: my radio\n- radio button\n- another one\n- third radio\n\nDropdown: My question here\n    - Option 1\n    - Option 2\n    - Option 3\n\n[Submit]", "survey_id": "nh7bvpfssi2n", "user_id": "testuserid", "version": "version - todo"
    // };

    const idToTitle = {};
    survey?.blocks.forEach((block) => idToTitle[block.id] = block.properties.question);

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
                        {Object.entries(idToTitle).map(([key, value]) => {
                            console.log(`tablehead - ${key}: ${value}`)
                            if (value) {
                                return (
                                    <>
                                        <TableHead className="">{value}</TableHead>
                                    </>
                                )
                            }
                        })}
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
                                    {Object.entries(idToTitle).map(([key, value]) => {
                                        if (value) {
                                            console.log('getting answer with id')
                                            console.log(surveyResponse.answers[key])
                                            let answer = surveyResponse.answers[key] ?? '-';
                                            return (
                                                <>
                                                    <TableCell className="font-medium">{`${key}: ${answer}`}</TableCell>
                                                </>
                                            )
                                        }
                                    })}
                                </TableRow>
                            </>
                        );
                    })}
                </TableBody>
            </Table>
            <hr></hr>
            <div>
                {ListResponsesV2()}
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
