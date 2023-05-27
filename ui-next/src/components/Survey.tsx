import React, { useState } from 'react';
// import Checkbox from './Checkbox';
// import Radio from './Radio';
// import { useForm, SubmitHandler } from "react-hook-form";


export default function Survey({ survey, BACKEND_URL }) {

    function formToJSON(elem) {
        let output = {};
        let formdata = new FormData(elem);
        formdata.forEach(
            (value, key) => {
                console.log("getall: " + key + "  " + formdata.getAll(key));
                formdata.getAll(key);
                output[key] = formdata.getAll(key);
            }
        );
        return JSON.stringify(output);
    }

    const onSubmit = event => {
        event.preventDefault();
        console.log("formdata json: " + formToJSON(event.target));
    }

    const form_content = survey?.questions?.map((question) => {
        console.log(survey);
        let count = 0;
        let options = question?.options.map((option) => {
            count += 1;
            let key = `{option.id}_${count}`;

            // jsx for the options
            return (
                <>
                    <li key={key} className='border-red-400 border'>
                        <label htmlFor={option.text} >
                            {option.text + " id: " + option.id}
                        </label>
                        <input id={option.text} type={question.type} name={question.id} value={option.text} />
                    </li>
                </>
            )
        });

        return (
            <>
                <label key={question.id}>{"question: " + question.value + " id: " + question.id}</label>
                {options}
            </>
        )
    });


    return (
        <>
            {!survey && !survey.questions ? '' :
                <div className='flex flex-col'>
                    <form
                        className='flex flex-col'
                        onSubmit={(event) => {
                            event.preventDefault();
                            onSubmit(event);
                        }}
                    >
                        <ul key='form'>
                            {form_content}
                        </ul>
                        <button
                            // type="submit"
                            onSubmit={event => {
                                event.preventDefault();
                                onSubmit(event);
                            }}
                            // onClick={onSubmit}
                            // onClick={(event) => {
                            //     event.preventDefault();
                            //     onSubmit(event);
                            // }}
                            className={'hover:bg-violet-600 w-20  bg-red-200 rounded p-2'}
                        >Submit</button>
                    </form>
                </div >}
        </>
    )
}