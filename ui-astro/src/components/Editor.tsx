import React, { useCallback, useEffect, useMemo, useRef, useState } from 'react';
// import { SubmitHandler, useForm } from 'react-hook-form';

// import { nanoid_gen, markdown_to_form_wasm } from "../../../backend/pkg";
// import { CreateSurveyRequest } from "../../../server/bindings/CreateSurveyRequest";

// const wasm = await WebAssembly.instantiateStreaming(
//     fetch("markdownparser_bg.wasm")
// );


// let wasm;

// WebAssembly.instantiateStreaming(fetch("/Users/jarde/Documents/code/markdownparser/ui-astro/public/markdownparser_bg.wasm")).then(
//     (results) => {
//         // Do something with the results!
//         wasm = results;
//     },
// );

// import init, { parse_markdown_v3 } from 'markdownparser';

export function useDebouncedCallback<A extends any[]>(
    callback: (...args: A) => void,
    wait: number
) {
    // track args & timeout handle between calls
    const argsRef = useRef<A>();
    const timeout = useRef<ReturnType<typeof setTimeout>>();

    function cleanup() {
        if (timeout.current) {
            clearTimeout(timeout.current);
        }
    }

    // make sure our timeout gets cleared if
    // our consuming component gets unmounted
    useEffect(() => cleanup, []);

    return function debouncedCallback(
        ...args: A
    ) {
        // capture latest args
        argsRef.current = args;

        // clear debounce timer
        cleanup();

        // start waiting again
        timeout.current = setTimeout(() => {
            if (argsRef.current) {
                callback(...argsRef.current);
            }
        }, wait);
    };
}


export default function Editor({ editor, setEditor, setSurvey }) {
    // const { register, handleSubmit, watch, formState: { errors } } = useForm();
    // const onSubmit = data => console.log(data);
    // console.log(watch("example")); // watch input value by passing the name of it

    function onSubmit(event) {
        event.preventDefault();
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
            console.log('inside promise: ' + JSON.stringify(resp));
            setSurvey(resp);
        });
    };

    function sendSurveyTestRequest() {
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

            console.log('inside promise: ' + JSON.stringify(resp));
            setSurvey(resp);
        });
    }

    const handleEditorChange = useDebouncedCallback(() => {
        sendSurveyTestRequest();
    }, 2000);


    useEffect(() => {
        handleEditorChange();
    }, [editor]);

    return (
        <>
            <React.StrictMode>
                <div className={"m-5 p-3 rounded-xl bg-white dark:bg-gray-800 "}>
                    <form onSubmit={(event) => onSubmit(event)}>
                        <label htmlFor="editor-field" className='sr-only'>
                            Create your survey
                        </label>
                        <textarea
                            className={' p-3 w-full text-sm text-gray-800  border-0 resize-y rounded-xl dark:bg-gray-800 dark:text-white dark:placeholder-gray-400'}
                            name="testname" id="editor-field" rows={3} value={editor}
                            onChange={event => {
                                setEditor(event.target.value);
                            }}
                        ></textarea>

                        <input type="submit" value="submit" className={'hover:bg-violet-600 w-full text-blue-500 bg-blue-200 rounded p-2'}
                        // onClick={event => {
                        // postQuestions();
                        // console.log('posting the questions');
                        // event.stopPropagation();
                        // event.preventDefault();
                        // handleSubmit(onSubmit);
                        // }} 
                        />
                        {/* <p>{survey}</p> */}
                    </form>
                </div>
                <br></br>
                {/* {errors && <span>This field is required</span>} */}

            </React.StrictMode>
        </>
    )
}

