import React, { useCallback, useEffect, useMemo, useRef, useState } from 'react';
import { SubmitHandler, useForm } from 'react-hook-form';
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
    const { register, handleSubmit, watch, formState: { errors } } = useForm();
    const onSubmit = data => console.log(data);
    console.log(watch("example")); // watch input value by passing the name of it
    // const [editor, setEditor] = React.useState('');

    // const timeout = useRef<any>();
    // const [editor, setEditor] = useSearchDebounce();
    // const timeout;

    // const [survey, setSurvey] = React.useState('');

    // let createSurveyReq = { };
    useEffect(() => {
        console.log('setting survey soon...');
        let result = fetch("http://127.0.0.1:8080/surveys/test", {
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
                <br></br>
                <div className="grid grid-cols-1 gap-2">
                    <div>
                        "another item"
                    </div>
                    {"other form"}
                    /* "handleSubmit" will validate your inputs before invoking "onSubmit" */
                    <form onSubmit={handleSubmit(onSubmit)}>
                        {/* register your input into the hook by invoking the "register" function */}
                        <input defaultValue="test" {...register("example")} />

                        {/* include validation with required or other standard HTML validation rules */}
                        <input {...register("exampleRequired", { required: true })} />
                        {/* errors will return when field validation fails  */}
                        {errors.exampleRequired && <span>This field is required</span>}

                        <input type="submit" />
                    </form>
                </div>
            </React.StrictMode>
        </>
    )
}

