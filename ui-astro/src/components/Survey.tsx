import React from 'react';
import Checkbox from './Checkbox';
import Radio from './Radio';
import { useForm, SubmitHandler } from "react-hook-form";

// import init, { nanoid_gen } from 'markdownparser';
// import * as test from "markdownparser";

// WebAssembly.instantiate("/markdownparser_bg.wasm");

// async function getData() {
//     const data = await fetch('http://localhost:3000/surveys').then((response) => response.json());
//     return data;
// }
// const wasm = await WebAssembly.instantiateStreaming(fetch('/markdownparser_bg.wasm'));
// let checkboxlist = ['r', 't', 'y']
// const checkboxDict = {};
// checkboxlist.forEach((opt) => {
//     checkboxDict[opt] = false;
// });
const initialToppings = {
    anchovies: false,
    chicken: false,
    tomatoes: false,
}
// test.nanoid_gen(2);
export default function Survey({ survey }) {
    // const [survey, setSurvey] = React.useState('');
    // const [id, setId] = React.useState('');
    // const [checkbox, setCheckbox] = React.useState({ 'thing': false });
    // const [
    //     pizzaToppings,
    //     setPizzaToppings
    // ] = React.useState(initialToppings);

    // Get a list of all toppings.
    // ['anchovies', 'chicken', 'tomato'];
    // const toppingsList = Object.keys(initialToppings);
    return (
        <>
            <div>
                {"this is the survey component"}
                <br></br>
                {survey?.id ? survey.id : 'no survey set'}
            </div>
        </>
    )
    // return (
    //     <>
    //         <h2>Survey:</h2>
    //         <Radio
    //             radio_options={['a', 'b', 'c']}
    //         />
    //         <Checkbox
    //         // options={checkboxDict}
    //         // options={checkbox}
    //         // setOptinos={setCheckbox}
    //         />


    //         <>
    //             <form>
    //                 <fieldset>
    //                     <legend>
    //                         Select toppings:
    //                     </legend>

    //                     {/*
    //         Iterate over those toppings, and
    //         create a checkbox for each one:
    //       */}
    //                     {toppingsList.map(option => (
    //                         <div key={option}>
    //                             <input
    //                                 type="checkbox"
    //                                 id={option}
    //                                 value={option}
    //                                 checked={pizzaToppings[option] === true}
    //                                 onChange={event => {
    //                                     setPizzaToppings({
    //                                         ...pizzaToppings,
    //                                         [option]: event.target.checked,
    //                                     })
    //                                 }}
    //                             />
    //                             <label htmlFor={option}>
    //                                 {option}
    //                             </label>
    //                         </div>
    //                     ))}
    //                 </fieldset>
    //             </form>
    //             <p>
    //                 <strong>Stored state:</strong>
    //             </p>
    //             <p className="output">
    //                 {JSON.stringify(pizzaToppings, null, 2)}
    //             </p>
    //         </>
    //     </>
    // );
}