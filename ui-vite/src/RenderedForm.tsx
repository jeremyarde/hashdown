import { useState } from "react";
import { Button } from './components/ui/button';
import { Label } from "@/components/ui/label"
import { Input } from "@/components/ui/input"
import { BASE_URL } from "./lib/constants";
import { setDefaultHighWaterMark } from "stream";

/**
 * The complete Triforce, or one or more components of the Triforce.
 * @typedef {Object} Option
 * @property {string} text - Indicates whether the Courage component is present.
 * @property {string} id - Indicates whether the Power component is present.
 */
export type RenderedFormProps = {
    // plaintext: string;
    survey: object;
    mode: "test" | "prod"
}

export function RenderedForm({ survey, mode }: RenderedFormProps) {
    const [exampleSubmission, setExampleSubmittion] = useState();
    const [displayTextMode, setDisplayTextMode] = useState(false);
    const surveyIdToText = Object.fromEntries(survey.blocks.map((block) => [block.id, block.properties.question]));

    console.log(`RenderedForm: ${JSON.stringify(survey)}`)
    console.log(`IdToText: ${JSON.stringify(surveyIdToText)}`)

    let parsingError = undefined;
    if (!survey.blocks) {
        parsingError = survey;
    }
    console.log("renderSurveyV2", survey);

    const handleSubmit = async (evt) => {
        evt.preventDefault();
        let formdata = new FormData(evt.target);
        const survey_id = survey.survey_id;

        const surveySubmission = {
            survey_id: survey_id ?? '',
            answers: Object.fromEntries(formdata)
        }

        if (mode === "test") {
            setExampleSubmittion(surveySubmission);
            return;
        }

        console.log(`submission: ${JSON.stringify(surveySubmission)}`)
        // setSubmittedValues((prev) => surveySubmission);

        if (survey_id) {
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
        }
    }

    const handleUpdate = (evt) => {
        console.log('update event')
        console.log(evt.target.form)
        let formdata = new FormData(evt.target.form);
        const survey_id = survey.survey_id;

        const surveySubmission = {
            survey_id: survey_id ?? 'surveyid',
            answers: Object.fromEntries(formdata)
        }

        if (mode === "test") {
            setExampleSubmittion(surveySubmission);
            return;
        }
    }

    function substituteSubmissionIdToText(exampleSubmission): any {
        let textVersion = {};
        Object.entries(exampleSubmission.answers).map((([key, value]) => textVersion[surveyIdToText[key]] = value));
        return { ...exampleSubmission, answers: textVersion }
    }

    return (
        <>
            <div className="border border-gray-300 p-4 rounded">
                {/* <h1 className='w-full text-center'>{survey.title}</h1> */}
                {/* <h1 className="text-3xl font-bold space-y-2 text-center" >
                    {survey.title}
                </h1> */}
                {parsingError ? (
                    <div style={{ whiteSpace: "pre-wrap", textAlign: "left" }}>
                        <pre>
                            {/* <code className="bg-red-200">{parsingError}</code> */}
                        </pre>
                    </div >
                ) : ''}
                <div>
                    <form onSubmit={handleSubmit} onChange={handleUpdate} className="text-left">
                        {
                            survey.blocks?.map(block => {
                                console.log("map entries: ", block)
                                let blockHtml = undefined;
                                switch (block.block_type) {
                                    case "Title":
                                        blockHtml = (
                                            <h1 className="text-3xl font-bold space-y-2 text-center" >
                                                {block.properties.title}
                                            </h1>)
                                        break;
                                    case "TextInput":
                                        blockHtml = (
                                            <div>
                                                {textInput(block, setExampleSubmittion)}
                                            </div>
                                        )
                                        break;
                                    case "Checkbox":
                                        blockHtml = (
                                            <div>
                                                {checkboxGroupV2(block, setExampleSubmittion)}
                                            </div>
                                        )
                                        break;
                                    case "Radio":
                                        blockHtml = (
                                            <div>
                                                {radioGroupV2(block, setExampleSubmittion)}
                                            </div>
                                        )
                                        break;
                                    case "Submit":
                                        blockHtml = (
                                            <div>
                                                {submitButton(block)}
                                            </div>
                                        )
                                        break;
                                }

                                return (
                                    <div style={{ margin: "20px", border: "line" }}>
                                        {blockHtml}
                                    </div>
                                )
                            })
                        }
                    </form>

                </div>
            </div>
            {exampleSubmission ? (
                <>
                    {/* <Button type="button" onClick={(evt) => displayTextMode ? setDisplayTextMode(false) : setDisplayTextMode(true)}></Button> */}
                    <div className="fixed skeu">
                        <div>
                            <h3>Submission data</h3>
                            <label className="switch">
                                <input type="checkbox" onClick={(evt) => displayTextMode ? setDisplayTextMode(false) : setDisplayTextMode(true)} />
                                <span className="slider round"></span>
                            </label>
                        </div>
                        <div className="text-left">
                            <pre>
                                <code className="bg-blue-200">
                                    {!displayTextMode ? (JSON.stringify(exampleSubmission, null, 2)) :
                                        (JSON.stringify(substituteSubmissionIdToText(exampleSubmission), null, 2))}
                                </code>
                            </pre>
                        </div>
                    </div>
                </>
            ) : ''}
        </>
    );
}

function checkboxGroupV2(block, setStateFn) {
    return (
        <>
            <Label className="font-semibold">{block.properties.question}</Label>
            <div className="flex flex-col space-y-2">
                {block.properties.options.map((option, i) => {
                    return (
                        <div className="flex items-center">
                            <input type="checkbox" defaultChecked={option.checked} id={block.id + `_${i}`} name={block.id + `_${i}`} />
                            <Label className="ml-2 text-sm items-center" htmlFor={block.id + `_${i}`}>
                                {option.text}
                            </Label>
                        </div>
                    )
                })}
            </div >
        </>
    )
}

function radioGroupV2(block, setStateFn) {

    return (
        <>
            <Label className="space-y-2 p-2 text-left">{block.properties.question}</Label>
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
            <Input id={block.id} name={block.id} placeholder="Enter text" />
        </>
    )
}

function renderSurveyV2(survey) {
    const [exampleSubmittion, setExampleSubmittion] = useState();

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