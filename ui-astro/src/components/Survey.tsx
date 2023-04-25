import React, { useState } from 'react';
import Checkbox from './Checkbox';
import Radio from './Radio';
import { useForm, SubmitHandler } from "react-hook-form";


export default function Survey({ survey }) {
    const [formstate, setformstate] = useState('');
    // const { register, handleSubmit, watch, formState: { errors } } = useForm();
    const onSubmit = data => {
        console.log('submitted data:')
        console.log(data);
    }

    const form_content = survey?.questions.map((question) => {

        let newOptions = {};
        let options = question?.options.map((option) => {
            // newOptions[option.id] = '';
            return (
                <>
                    <label key={option.id}>
                        {/* <input key={option.id} type={question.type} {...register(option.id, { required: true })} /> */}

                        <input key={option.id} type={question.type} name={option.id} />
                        {option.text}
                    </label>
                </>
            )
        });
        console.log('options:');
        console.log(options);

        return (
            <>
                <label key={question.id}>{question.value}</label>
                {/* <input type={question.type} {...register(question.id, { required: true, })}></input> */}
                {options}
            </>
        )
    });


    return (
        <>
            <div className='flex flex-col'>
                <br></br>
                {survey?.questions ?
                    <form
                        className='flex flex-col'
                        onSubmit={onSubmit}
                        action={`http://localhost:8080/surveys/`}
                        method="POST"
                    >
                        {form_content}
                        <input
                            type="submit"
                            // onClick={(event) => {
                            //     event.preventDefault();
                            //     // alert();
                            // }}
                            value="submit" className={'hover:bg-violet-600 w-20  bg-red-200 rounded p-2'}

                        />
                    </form> : 'no survey set'}
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