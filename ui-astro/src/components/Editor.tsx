import React, { useCallback, useEffect, useMemo, useRef, useState } from 'react';
import { nanoid_gen, markdown_to_form_wasm } from "../../../backend/pkg";
import { CreateSurveyRequest } from "../../../server/bindings/CreateSurveyRequest";

// export function useSearchDebounce(delay = 350) {
//     const [search, setSearch] = useState('');
//     // const [searchQuery, setSearchQuery] = useState('');
//     const sendRequest = useCallback((value) => {
//         console.log("Value change");
//     }, []);

//     useEffect(() => {
//         const delayFn = setTimeout(() => setSearch(searchQuery), delay);
//         return () => clearTimeout(delayFn);
//     }, [searchQuery, delay]);

//     return [search, setSearchQuery];
// }

// export function debounce(callback, delay) {
//     let timeout;
//     return function () {
//         clearTimeout(timeout);
//         timeout = setTimeout(callback, delay);
//     }
// }


export default function Editor({ editor, setEditor, setSurvey }) {
    // const [editor, setEditor] = React.useState('');

    // const timeout = useRef<any>();
    // const [editor, setEditor] = useSearchDebounce();
    // const timeout;

    // const [survey, setSurvey] = React.useState('');

    console.log(`THIS IS BROKEN`);
    // let createSurveyReq = { };
    useEffect(() => {
        console.log('setting survey soon...');
        let result = fetch("http://127.0.0.1:8080/surveys", {
            method: "POST",
            body: JSON.stringify({
                plaintext: editor
            }),
            headers: {
                "Content-type": "application/json"
            }
        }).then(async (response) => {
            let resp = await response.json();

            console.log(resp);
            setSurvey(resp.survey);
        });
        console.log('results from fetch');
        console.log(result);
    }, [editor]);

    return (
        <>
            <React.StrictMode>
                <div className={"p-4 rounded-xl bg-white dark:bg-gray-800 "}>
                    <form action="">
                        <label htmlFor="editor-field" className='sr-only'>
                            Create your survey
                        </label>
                        <textarea
                            className={'m-2 p-3 w-full text-sm text-gray-800  border-0 resize-y rounded-xl dark:bg-gray-800 dark:text-white dark:placeholder-gray-400'}
                            name="testname" id="editor-field" rows={10} value={editor}
                            onChange={event => {
                                // if (event.target.value) {
                                setEditor(event.target.value);
                                // const results = markdown_to_form_wasm(event.target.value);
                                // setSurvey(results);
                                console.log("parsing results:");
                                // console.log(results);
                            }}
                        ></textarea>
                        <button className={'hover:bg-violet-600 w-full text-blue-500 bg-blue-200 rounded p-2'} onClick={event => {
                            // postQuestions();
                            console.log('posting the questions');
                        }}>
                            Publish
                        </button>
                        {/* <p>{survey}</p> */}
                    </form>
                </div>
            </React.StrictMode>
        </>
    )
}

