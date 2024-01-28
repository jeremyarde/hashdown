// @ts-nocheck
import { useState } from "react";
import { Button } from '../ui/button';
import { Label } from "@/components/ui/label"
import { Input } from "@/components/ui/input"
import { Textarea } from "../ui/textarea";
import { getBaseUrl } from "../../lib/utils";
import { useNavigate } from 'react-router-dom';
import { RenderedFormProps } from "@/lib/constants";

type SurveyEvent = {
    question_id: string;
    value: any;
}
export type RenderedFormProps = {
    // plaintext: string;
    survey: object;
    mode: "test" | "prod",
    showSubmissionData: boolean,
}

export function RenderedForm({ survey, mode, showSubmissionData = false }: RenderedFormProps) {
    const [displayTextMode, setDisplayTextMode] = useState(false);
    const [showEndScreen, setShowEndScreen] = useState(false);
    const [dummy, setDummy] = useState(true); // use to trigger rerender
    const [exampleSubmission, setExampleSubmittion] = useState(getDefaultState());
    console.log('ex', exampleSubmission)

    function getDefaultState() {
        return {
            survey_id: survey.survey_id,
            answers: {}
        }
    }

    function handleEvent(surveyEvent: SurveyEvent) {
        console.log('handleEvent: ', surveyEvent)
        setExampleSubmittion(curr => {
            curr.answers[surveyEvent.question_id] = surveyEvent.value
            return curr
        })
        // use this to trigger a rerender
        setDummy(dummy ? false : true)
    }

    let parsingError = undefined;
    if (!survey.blocks) {
        parsingError = survey;
    }

    const handleSubmit = async (evt) => {
        evt.preventDefault();
        const survey_id = survey.survey_id;
        const surveySubmission = exampleSubmission;

        console.log('jere/ mode', mode);
        if (mode === "test") {
            setExampleSubmittion(surveySubmission);
            setShowEndScreen(true);
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
            setShowEndScreen(true);
        }
    }

    const handleUpdate = (evt) => {
        const survey_id = survey.survey_id;

        const surveySubmission = exampleSubmission
        if (mode === "test") {
            setExampleSubmittion(surveySubmission);
            return;
        }
    }

    return (
        <>
            <div className="flex w-full h-full items-center align-middle justify-center">
                {parsingError ? (
                    <div style={{ whiteSpace: "pre-wrap", textAlign: "left" }}>
                        <pre>
                            <code className="bg-red-200">{parsingError}</code>
                        </pre>
                    </div >
                ) : ''}
                {showEndScreen ? <EndScreen></EndScreen> :


                    <div className="" style={{
                        width: '1000px',
                        maxWidth: '48rem',
                        minWidth: '12rem',
                        // height: '1vh',
                        // justifyContent: 'center',
                        // alignSelf: 'center'
                    }}>
                        <form
                            onKeyUp={evt => {
                                console.log('onkeyup...', evt.key)
                                evt.key === "Enter" ? handleSubmit(evt) : ''
                            }}
                            onSubmit={handleSubmit}
                            onChange={handleUpdate}
                            className="text-left border border-solid rounded-xl">
                            {
                                survey.blocks?.map(block => {
                                    // console.log("map entries: ", block)
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
                                                    {TextInput(block, setExampleSubmittion, handleEvent)}
                                                </div>
                                            )
                                            break;
                                        case "Textarea":
                                            blockHtml = (
                                                <div>
                                                    {TextareaComponent(block, setExampleSubmittion, handleEvent)}
                                                </div>
                                            )
                                            break;
                                        case "Checkbox":
                                            blockHtml = (
                                                <div>
                                                    {CheckboxGroup(block, setExampleSubmittion, handleEvent)}
                                                </div>
                                            )
                                            break;
                                        case "Radio":
                                            blockHtml = (
                                                <div>
                                                    {RadioGroup(block, setExampleSubmittion, handleEvent)}
                                                </div>
                                            )
                                            break;
                                        case "Submit":
                                            blockHtml = (
                                                <div>
                                                    {SubmitButton(block)}
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
                }
            </div>
            {showEndScreen && mode === 'test' ?
                (<div>
                    <button
                        onClick={evt => setShowEndScreen(false)}
                        className="bg-purple p-2 w-2/3"
                    >Go back</button>
                </div>) : <></>
            }
            {exampleSubmission && showSubmissionData ? (
                <>
                    <div className="">
                        <div>
                            <h3>Submission data</h3>
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

function CheckboxGroup(block, setStateFn, handleEvent) {
    // const [checkboxGroup, setCheckboxGroup] = useState(
    //     block.properties.options.map(option => option.checked ? option.text : undefined).filter(item => item)
    // )
    // const onChange = (evt, option) => {
    //     // e.preventDefault()
    //     if (checkboxGroup.includes(option.text)) {
    //         setCheckboxGroup(checkboxGroup.filter(c => c !== option.text))
    //         setStateFn(curr => {
    //             if (!curr.answers[block.properties.id]) {
    //                 curr.answers[block.properties.id] = []
    //             }
    //             curr.answers[block.properties.id] = curr.answers[block.properties.id].filter((c => c !== option.text))
    //             return curr
    //         })
    //     } else {
    //         setCheckboxGroup([
    //             ...checkboxGroup,
    //             option.text
    //         ])
    //         setStateFn(curr => {
    //             curr.answers[block.properties.id] = [
    //                 ...checkboxGroup,
    //                 option.text
    //             ]
    //             return curr
    //             // return {
    //             //     ...curr,
    //             //     [block.properties.id]: [
    //             //         ...checkboxGroup,
    //             //         option.text
    //             //     ]
    //             // }
    //         })
    //     }
    // }

    // console.log('jere/ checkbox: ', checkboxGroup)
    return (
        <>
            <Label className="font-semibold">{block.properties.question}</Label>
            <div className="flex flex-col space-y-2">
                {block.properties.options.map((option, i) => {
                    return (
                        <div className="flex items-center">
                            {/* <input type="checkbox" defaultChecked={option.checked} id={block.id + `_${i}`} name={block.id + `_${i}`} /> */}
                            <input type="checkbox"
                                // defaultChecked={option.checked}
                                // checked={checkboxGroup.includes(option.text) ? true : false}
                                //  id={`${block.properties.id}.${option.id}`} name={`${block.properties.id}.${option.id}`}
                                onChange={e => {
                                    // onChange(e, option)
                                    handleEvent({ value: option.text, question_id: block.properties.id })
                                }}
                            />
                            <Label
                                onClick={e => {
                                    handleEvent({ value: option.text, question_id: block.properties.id })
                                }}
                                className="ml-2 text-sm items-center" htmlFor={`${block.properties.id}.${option.id}`}>
                                {option.text}
                            </Label>
                        </div>
                    )
                })}
            </div >
        </>
    )
}

function RadioGroup(block, setStateFn, handleEvent) {
    // const [state, setState] = useState('')

    // function onClick(evt, option, id) {
    //     // setState(option)
    //     setStateFn(curr => {
    //         console.log('radio group', curr)
    //         curr.answers[id] = option
    //         return curr
    //     })
    // }

    return (
        <>
            <Label className="space-y-2 text-left">{block.properties.question}</Label>
            <div className="flex flex-col">
                <ul className="space-y-2" >
                    {block.properties.options.map((option: string) => {
                        return (
                            <li>
                                <div
                                    // onClick={evt => onClick(evt, option, block.properties.id)}
                                    onClick={evt => handleEvent({ value: option, question_id: block.properties.id })}
                                    className="flex items-center space-x-2">
                                    <input type="radio" id={option} name={block.properties.id} value={option} />
                                    <Label className="items-center" htmlFor={option} >
                                        {option}
                                    </Label >
                                </div>
                            </li>
                        )
                    })}
                </ul>
            </div>
        </>
    )
}

function TextInput(block, setStateFn, handleEvent) {
    // const [state, setState] = useState('')

    // const updateTextInput = (evt, id) => {
    //     setState(evt.target.value)
    //     setStateFn(curr => {
    //         curr.answers[id] = evt.target.value
    //         return curr
    //     })
    // }
    return (
        <>
            <Label htmlFor={block.properties.id}>{block.properties.question}</Label>
            <Input
                onChange={evt => {
                    // updateTextInput(evt, block.properties.id)
                    handleEvent({ value: evt.target.value, question_id: block.properties.id })
                }}
                id={block.properties.id} name={block.properties.id} placeholder="Enter text" />
        </>
    )
}

function TextareaComponent(block, setStateFn, handleEvent) {
    // const [state, setState] = useState('')

    // const updateTextInput = (evt, id) => {
    //     setState(evt.target.value)
    //     setStateFn(curr => {
    //         curr.answers[id] = evt.target.value
    //         return curr
    //     })
    // }
    return (
        <>
            <Label htmlFor={block.properties.id}>{block.properties.question}</Label>
            <Textarea
                // value={state}
                onChange={evt => {
                    // setState(evt.target.value)
                    handleEvent({ value: evt.target.value, question_id: block.properties.id })
                }}
                id={block.properties.id} name={block.properties.id} placeholder="Enter text" />
        </>
    )
}

function SubmitButton(block) {
    return (
        <>
            <div>
                <Button className="outline outline-1 active:bg-green" type="submit">{block.properties.button}</Button>
            </div>
        </>
    )
}

function EndScreen(block) {
    return (
        <>
            <div
                className="outline outline-solid rounded-2xl w-2/3 text-center flex justify-center"
                style={{ height: '50vh', display: '', alignItems: 'center' }}>
                <h1>Thanks for your response!</h1>
            </div>
        </>
    )
}