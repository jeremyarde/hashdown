// @ts-nocheck
import { useState } from "react";
import { Button } from './components/ui/button';
import { Label } from "@/components/ui/label"
import { Input } from "@/components/ui/input"
import { Textarea } from "./components/ui/textarea";
import { getBaseUrl } from "./lib/utils";

function surveyToForm(survey: Survey) {
    let form = {};
    let idToText = {};

    survey.blocks?.forEach((block) => {
        if (block.block_type === 'Checkbox') {
            block.properties.options?.forEach((option, i) => {
                form[option.id] = Boolean(option.checked)
                idToText[option.id] = option.text
            });
        } else {
            form[block.id] = '';
            idToText[block.id] = block.properties.question
        }
    });
    return [form, idToText]
}

export function RenderedForm({ survey, mode }: RenderedFormProps) {
    const [exampleSubmission, setExampleSubmittion] = useState();
    const [displayTextMode, setDisplayTextMode] = useState(false);

    const [_, surveyIdToText] = surveyToForm(survey);

    let parsingError = undefined;
    if (!survey.blocks) {
        parsingError = survey;
    }

    const handleSubmit = async (evt) => {
        console.log('jere/ submit survey');
        console.log('jere/ survey', survey);

        evt.preventDefault();
        let formdata = new FormData(evt.target);
        const survey_id = survey.survey_id;

        const surveySubmission = {
            survey_id: survey_id ?? '',
            answers: Object.fromEntries(formdata)
        }

        console.log('jere/ mode', mode);
        if (mode === "test") {
            setExampleSubmittion(surveySubmission);
            return;
        }
        console.log('jere/ surveyid', survey_id);

        if (survey_id) {
            const response = await fetch(`${getBaseUrl()}/submit`, {
                method: "POST",
                headers: {
                    "Content-Type": "application/json",
                },
                body: JSON.stringify(surveySubmission)
            });
            console.log(`submit response: ${JSON.stringify(response)}`);
        }
    }

    const handleUpdate = (evt) => {
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
                {parsingError ? (
                    <div style={{ whiteSpace: "pre-wrap", textAlign: "left" }}>
                        <pre>
                            <code className="bg-red-200">{parsingError}</code>
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
                                            <h1 className="text-xl font-bold space-y-2 text-center" >
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
                                    case "Textarea":
                                        blockHtml = (
                                            <div>
                                                {textareaComponent(block, setExampleSubmittion)}
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
                    <div className="skeu">
                        <div>
                            <h3>Submission data</h3>
                            <div>
                                <label>{"Show real questions"}</label>
                                <label className="switch">
                                    <input type="checkbox" onClick={(evt) => displayTextMode ? setDisplayTextMode(false) : setDisplayTextMode(true)} />
                                    <span className="slider round"></span>
                                </label>

                            </div>
                        </div>
                        <div className="text-left p-6 border-dotted border">
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
                            {/* <input type="checkbox" defaultChecked={option.checked} id={block.id + `_${i}`} name={block.id + `_${i}`} /> */}
                            <input type="checkbox" defaultChecked={option.checked} id={option.id} name={option.id} />
                            <Label className="ml-2 text-sm items-center" htmlFor={option.id}>
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

function textInput(block) {
    return (
        <>
            <Label htmlFor={block.id}>{block.properties.question}</Label>
            <Input id={block.id} name={block.id} placeholder="Enter text" />
        </>
    )
}

function textareaComponent(block) {
    return (
        <>
            <Label htmlFor={block.id}>{block.properties.question}</Label>
            <Textarea id={block.id} name={block.id} placeholder="Enter text" />
        </>
    )
}

function submitButton(block) {
    return (
        <>
            <div>
                <Button className="outline outline-1 active:bg-green" type="submit">{block.properties.question}</Button>
            </div>
        </>
    )
}