import { useState } from "react";
import { Button } from './components/ui/button';
import { markdown_to_form_wasm_v2 } from '../../backend/pkg/markdownparser'
import { Label } from "@/components/ui/label"
import { Input } from "@/components/ui/input"
import { BASE_URL } from "./lib/constants";

/**
 * The complete Triforce, or one or more components of the Triforce.
 * @typedef {Object} Option
 * @property {string} text - Indicates whether the Courage component is present.
 * @property {string} id - Indicates whether the Power component is present.
 */
export type RenderedFormProps = {
    // plaintext: string;
    survey: object;
}

export function RenderedForm({ survey }: RenderedFormProps) {
    console.log(`RenderedForm: ${JSON.stringify(survey)}`)
    return (
        <>
            <div className="border border-gray-300 p-4 rounded">
                {/* <h1 className='w-full text-center'>{survey.title}</h1> */}
                <h1 className="text-3xl font-bold space-y-2" >
                    {survey.title}
                </h1>
                {/* {renderSurvey(survey)} */}
                {renderSurveyV2(survey)}
            </div>
        </>
    );
}

function checkboxGroupV2(block, setStateFn) {
    // const questionId = crypto.randomUUID();

    return (
        <>
            <Label className="font-semibold">{block.properties.question}</Label>
            <div className="flex flex-col space-y-2">
                {block.properties.options.map((option, i) => {

                    // if (!option.text) { return; } if (!option.ListItem) { return; }

                    // console.log("checkboxgrouppart", option);
                    return (
                        <div className="flex items-center">
                            <input type="checkbox" defaultChecked={option.checked} id={block.id + `_${i}`} name={block.id + `_${i}`} />
                            <Label className="ml-2 text-sm items-center" htmlFor={block.id + `_${i}`}>
                                {option.text}
                            </Label>
                        </div>
                    )
                })}
            </div>
        </>
    )
}

function radioGroupV2(block, setStateFn) {

    return (
        <>
            <Label className="space-y-2 p-2 text-left">{block.properties.question}</Label>
            <Label>{block.id}</Label>
            <ul className="" >
                <div className="flex flex-col space-y-2 ">
                    {block.properties.options.map((option: string) => {
                        // if (!option.ListItem) { return; }
                        console.log("part - option:", option);
                        return (
                            <li>
                                <div className="flex items-center space-x-2">
                                    <input type="radio" id={option} name={block.id} value={option} />
                                    <Label className="items-center" htmlFor={option} >
                                        {option}
                                    </Label >
                                </div>
                            </li>
                        )
                    })}
                </div>
            </ul>
        </>
    )
}

function textInput(block, setStateFn) {
    return (
        <>
            <Label htmlFor={block.id}>{block.properties.question}</Label>
            <hr></hr>
            <Label>{block.id}</Label>
            <Input id={block.id} name={block.id} placeholder="Enter text" />
        </>
    )
}

function renderSurveyV2(survey) {
    // const [parsingError, setParsingError] = useState();
    const [exampleSubmittion, setExampleSubmittion] = useState();
    // console.log("render v2 plaintext:", plaintext);

    let parsingError = undefined;
    if (!survey.blocks) {
        parsingError = survey;
    }
    console.log("renderSurveyV2", survey);

    const handleSubmit = async (evt) => {
        evt.preventDefault();
        let formdata = new FormData(evt.target);
        console.log('form entries');
        console.log(Object.fromEntries(formdata));

        const surveySubmission = {
            survey_id: survey.survey_id ?? '',
            answers: Object.fromEntries(formdata)
        }

        console.log(`submission: ${JSON.stringify(surveySubmission)}`)
        // setSubmittedValues((prev) => surveySubmission);

        if (survey.id) {
            const response = await fetch(`${BASE_URL}/submit`, {
                method: "POST",
                headers: {
                    "Content-Type": "application/json",
                },
                credentials: 'include',
                // body: JSON.stringify(surveySubmission),
                body: JSON.stringify(surveySubmission)
            });
            console.log(`submit response: ${JSON.stringify(response)}`);
        } else {
            console.log("Not sending submittion");
        }
    }

    return (
        <>
            {parsingError ? (
                <div style={{ whiteSpace: "pre-wrap", textAlign: "left" }}>
                    <pre>
                        {/* <code className="bg-red-200">{parsingError}</code> */}
                    </pre>
                </div >
            ) : ''}
            {exampleSubmittion ? (
                <pre>
                    <code className="bg-blue-200">{JSON.stringify(exampleSubmittion, null, 2)}</code>
                </pre>
            ) : ''}
            <div>
                <form onSubmit={handleSubmit} >
                    {
                        survey.blocks?.map(block => {
                            console.log("map entries: ", block)

                            switch (block.block_type) {
                                case "Title":
                                    return (
                                        <h1 className="text-3xl font-bold space-y-2" >
                                            {block.properties.title}
                                        </h1>)
                                case "TextInput":
                                    return (
                                        <div>
                                            {textInput(block, setExampleSubmittion)}
                                        </div>
                                    )
                                case "Checkbox":
                                    return (
                                        <div>
                                            {checkboxGroupV2(block, setExampleSubmittion)}
                                        </div>
                                    )
                                case "Radio":
                                    return (
                                        <div>
                                            {radioGroupV2(block, setExampleSubmittion)}
                                        </div>
                                    )
                                case "Submit":
                                    return (
                                        <div>
                                            {submitButton(block)}
                                        </div>
                                    )
                            }
                        })
                    }
                </form>

            </div>

        </>)
}

function submitButton(block) {
    return (
        <>
            <div>
                <Button className="" type="submit">{block.properties.text}</Button>
            </div>
        </>
    )
}