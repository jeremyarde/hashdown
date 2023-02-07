import React, { useEffect } from 'react';
import Radio from './Radio';
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

    return (
        <>
            <Radio
            // radio_options={['a', 'b', 'c']}
            />
        </>
    )
}