import React, { useEffect } from 'react';
// import init, { nanoid_gen } from 'markdownparser';
// import * as test from "markdownparser";

// WebAssembly.instantiate("/markdownparser_bg.wasm");

async function getData() {
    const data = await fetch('http://localhost:3000/surveys').then((response) => response.json());
    return data;
}
// const wasm = await WebAssembly.instantiateStreaming(fetch('/markdownparser_bg.wasm'));

// test.nanoid_gen(2);
export default function Survey() {
    const [survey, setSurvey] = React.useState('');
    const [id, setId] = React.useState('');

    // useEffect(() => {
    //     init().then(() => {
    //         setId(nanoid_gen(5));
    //     })
    // }, [])
    // let allsurveys = survey.forEach(element => { });
    // wasmtest.initSync()
    // wasmtest.nanoid_gen(1);
    // const testnanoid = wasmtest.nanoid_gen(8);
    // const data = getData();

    return (
        <>
            <h2>From rust...</h2>
            <h3>The id is: {id}</h3>
            <h2>Above was from rust...</h2>
        </>
    )
}