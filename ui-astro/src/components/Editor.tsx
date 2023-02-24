import React from 'react';
import { nanoid_gen, parse_markdown_v3 } from "../../../backend/pkg";
import { CreateSurveyRequest } from "../../../server/bindings/CreateSurveyRequest";

export default function Editor() {
    const [editor, setEditor] = React.useState('');
    const [survey, setSurvey] = React.useState('');

    console.log(`THIS IS BROKEN`);
    // let createSurveyReq = { };


    return (
        <>
            <div className={"p-4 rounded-xl bg-white dark:bg-gray-800 "}>
                <form action="">
                    <label htmlFor="editor-field" className='sr-only'>
                        Create your survey
                    </label>
                    <textarea
                        className={'m-2 p-3 w-full text-sm text-gray-800  border-0 resize-y rounded-xl dark:bg-gray-800 dark:text-white dark:placeholder-gray-400'}
                        name="testname" id="editor-field" rows={10} value={editor}
                        onChange={event => {
                            setEditor(event.target.value);
                            const results = parse_markdown_v3(event.target.value);
                            setSurvey(results);
                            console.log("parsing results:");
                            console.log(results);
                        }}
                    ></textarea>
                    <button className={'hover:bg-violet-600 w-full text-blue-500 bg-blue-200 rounded p-2'} onClick={event => {
                        // postQuestions();
                        console.log('posting the questions');
                    }}>
                        Publish
                    </button>
                    <p>{survey}</p>
                </form>
            </div>
        </>
    )
}

