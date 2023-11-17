import { useEffect, useState } from "react";
import { zodResolver } from "@hookform/resolvers/zod";
import { useForm } from 'react-hook-form';
import { Button } from './components/ui/button';
import { Form, FormControl, FormDescription, FormField, FormItem, FormLabel, FormMessage } from './components/ui/form';
import { RadioGroup, RadioGroupItem } from './components/ui/radio-group';
import { Alert, AlertDescription, AlertTitle } from './components/ui/alert';
import * as z from "zod";
import { markdown_to_form_wasm, markdown_to_form_wasm_v2 } from '../../backend/pkg/markdownparser'

/**
 * The complete Triforce, or one or more components of the Triforce.
 * @typedef {Object} Option
 * @property {string} text - Indicates whether the Courage component is present.
 * @property {string} id - Indicates whether the Power component is present.
 */
export type RenderedFormProps = {
    plaintext: string;
    survey: object;
}

export function RenderedForm({ plaintext, survey }: RenderedFormProps) {
    const DISPLAY_SURVEY_VERSION = 1;

    // let [submittedValues, setSubmittedValues] = useState(null);

    // const { survey_id } = useParams<{ survey_id: string }>()

    // // let [form, setForm] = useState(undefined);

    // console.log(`render - survey: ${JSON.stringify(survey)}`)
    // let objects = {};
    // let defaultValues = {};
    // const questions = survey?.questions ?? [];
    // questions.forEach(element => {
    //     objects[element.id] = z.enum(element.options.map((option) => option.text), { required_error: "Please select an option" });
    //     defaultValues[element.id] = "";
    // });
    // const formSchema = z.object(objects);
    // const form = useForm<z.infer<typeof formSchema>>({
    //     resolver: zodResolver(formSchema),
    //     defaultValues: defaultValues,
    // });

    async function onSubmit(values: z.infer<typeof formSchema>) {
        // Do something with the form values.
        // ✅ This will be type-safe and validated.
        console.log(`submit:`);
        console.log(values);

        const surveySubmission = {
            survey_id: survey.survey_id ?? '',
            responses: values
        }
        setSubmittedValues((prev) => surveySubmission);

        if (survey.survey_id) {
            const response = await fetch(`${BASE_URL}/submit`, {
                method: "POST",
                headers: {
                    "Content-Type": "application/json",
                },
                credentials: 'include',
                body: JSON.stringify(surveySubmission),
            });
            console.log(`submit response: ${JSON.stringify(response)}`);
        } else {
            console.log("Not sending submittion");
        }
    }

    /**
     *
     * @param {Question} question
     */
    function formelements(question) {
        return (
            <FormField
                key={question.id}
                control={form.control}
                name={question.id}
                className="flex flex-row text-left justify-start"
                render={({ field }) => {
                    return (
                        <FormItem className='flex flex-col w-full text-left'>
                            <div className="flex flex-row items-center justify-between">
                                <FormLabel>{question.value}</FormLabel>

                            </div>
                            <div
                            // className="flex flex-col items-center align-middle"
                            >
                                <FormControl>
                                    <RadioGroup
                                        onValueChange={field.onChange}
                                        defaultValue={field.value}
                                    >
                                        {question.options.map((option) => {
                                            return (
                                                <FormItem
                                                // className="flex flex-col items-center space-x-3 w-full"
                                                >
                                                    <FormControl>
                                                        <RadioGroupItem value={option.text} />
                                                    </FormControl>
                                                    <FormLabel className=''>
                                                        {option.text}
                                                    </FormLabel>
                                                </FormItem>
                                            );
                                        })}
                                    </RadioGroup>
                                </FormControl>
                            </div>
                            <FormMessage />
                        </FormItem>
                    );
                }} />
        );
    }

    let toRender;
    if (survey) {
        if (DISPLAY_SURVEY_VERSION == 0) {
            toRender = (
                <>
                    <div className='flex flex-col justify-start items-start w-full'>
                        <h1 className='w-full text-center'>{survey.title}</h1>
                        {/* <Badge className="bg-gray-800 text-white" variant="outline">{survey.survey_id}</Badge> */}
                        <Form {...form}>
                            <form onSubmit={form.handleSubmit(onSubmit)}
                                className="space-y-6 w-full"
                            >
                                {survey.questions.map((question) => formelements(question))}
                                <Button className="bg-slate-400 rounded hover:bg-slate-150" type="submit">Submit</Button>
                            </form>
                        </Form>
                    </div>
                    {
                        submittedValues && (
                            <>
                                <h3>Submitted Values</h3>
                                <Alert className={'bg-green-300'}>
                                    <AlertDescription>
                                        {submittedValues ? JSON.stringify(submittedValues) : ''}
                                    </AlertDescription>
                                </Alert>
                            </>
                        )
                    }
                </>
            )
        } else {
            toRender = (
                <>
                    <div className=''>
                        {/* <h1 className='w-full text-center'>{survey.title}</h1> */}
                        <h1 className="text-3xl font-bold space-y-2" >
                            {survey.title}
                        </h1>
                        {renderSurvey(survey)}
                    </div>
                    <hr></hr>
                    <div>
                        {renderSurveyV2(plaintext)}
                    </div>
                </>
            );
        }
    } else {
        <div>Not available</div>
    }
    return (toRender)
}





/**
 * v0 by Vercel.
 * @see https://v0.dev/t/VB4f1VvX7aI
 */
import { Label } from "@/components/ui/label"
import { Input } from "@/components/ui/input"
import { SelectValue, SelectTrigger, SelectItem, SelectContent, Select } from "@/components/ui/select"
import { Checkbox } from "./components/ui/checkbox"
import { CardContent, Card } from "./components/ui/card"
import { Badge } from "./components/ui/badge";
import { BASE_URL } from "./lib/constants";
import { useParams } from "react-router-dom";
import { render } from "react-dom";


export interface SurveyModel {
    title: string
    plaintext: string
    questions: Question[]
    parse_version: string
    survey_id: string
}

export interface Question {
    id: string
    value: string
    options: Option[]
    type: "Text" | "Dropdown" | "Radio" | "Checkbox"
    created_on: string
    modified_on: string
}

export interface Option {
    id: string
    text: string
}

function radioGroup(question: Question, setStateFn) {
    const onchangeexample = (evt) => {
        console.log('onchangeexample');

        // console.log(evt.target);
        setStateFn(prev => ({
            ...prev,
            [question.id]: evt.target.value,
        }));
    }
    return (
        <div className="space-y-2 p-2 text-left" >
            <Label className="font-semibold">{question.value}</Label>
            <RadioGroup onChange={onchangeexample} >
                {question.options.map((option: Option) => {
                    return (
                        <>
                            <div className="flex items-center space-x-2">
                                <RadioGroupItem id={option.id} value={option.text} />
                                <Label className="" htmlFor={option.id} >{option.text}</Label >
                            </div>
                        </>
                    )
                })}
            </RadioGroup>
        </div>
    )
}

function checkboxGroup(question: Question) {
    return (
        <>
            <Label className="font-semibold">{question.value}</Label>
            <div className="flex flex-col space-y-2">
                {question.options.map((option) => {
                    return (
                        <div className="flex items-center">
                            <Checkbox id={option.id} name={option.id} required />
                            <Label className="ml-2 text-sm" htmlFor={option.id}>
                                {option.text}
                            </Label>
                        </div>
                    )
                })}
            </div>
        </>
    )
}

function dropdownGroup(question: Question) {
    return (
        <>
            <Label htmlFor="dropdown">{question.value}</Label>
            <Select required>
                <SelectTrigger id="dropdown">
                    <SelectValue placeholder="Select" />
                </SelectTrigger>
                <SelectContent>
                    {question.options.map((option) => {
                        return (
                            <SelectItem value={option.text}>{option.text} </SelectItem>
                        )
                    })}
                </SelectContent>
            </Select>
        </>
    )
}

function textInput(question: Question) {
    return (
        <>
            <Label htmlFor={question.id}>{question.value}</Label>
            <Input id={question.id} placeholder="Enter text" />
        </>
    )
}

function renderSurveyV2(plaintext) {
    const surveydetails = markdown_to_form_wasm_v2(plaintext);
    return (
        <>
            <div>Survey V2 rendering</div>
            <div>{JSON.stringify(surveydetails, null, 2)}</div>
        </>)
}

function renderSurvey(survey: SurveyModel) {
    const initialState = survey.questions.map((question: Question) => [question.id, '']);
    console.log('initial: ' + JSON.stringify(initialState));
    const [formstate, setFormstate] = useState(Object.fromEntries(initialState));
    console.log('formstate: ' + JSON.stringify(formstate));

    // console.log(`jere/ formstate: ` + formstate);
    async function onSubmit(evt) {
        evt.preventDefault();
        // Do something with the form values.
        // ✅ This will be type-safe and validated.
        const formdata = new FormData(evt.target);
        console.log(`submit:`);
        console.log(evt.target);
        console.log(JSON.stringify(formdata));
        console.log(formdata.get("myform"));


        const surveySubmission = {
            survey_id: survey.survey_id ?? '',
            responses: formstate
        }
        // setSubmittedValues((prev) => surveySubmission);

        if (survey.survey_id) {
            const response = await fetch(`${BASE_URL}/submit`, {
                method: "POST",
                headers: {
                    "Content-Type": "application/json",
                },
                credentials: 'include',
                body: JSON.stringify(surveySubmission),
            });
            console.log(`submit response: ${JSON.stringify(response)}`);
        } else {
            console.log("Not sending submittion");
        }
    }

    return (
        <Card>
            <CardContent>
                <form className="space-y-4"
                    // action="http://localhost:3000/api/submit"
                    // method="post"
                    // encType="application/x-www-form-urlencoded"
                    onSubmit={(evt) => onSubmit(evt)}
                    id="myform"
                // id={survey.survey_id}
                >
                    {survey.questions.map((question: Question) => {
                        let renderedQuestion = undefined;
                        switch (question.type) {
                            case "Radio":
                                renderedQuestion = radioGroup(question, setFormstate);
                                break;
                            case "Checkbox":
                                renderedQuestion = checkboxGroup(question);
                                break;
                            case "Dropdown":
                                renderedQuestion = dropdownGroup(question);
                                break;
                            case "Text":
                                renderedQuestion = textInput(question);
                                break;
                            default:
                                renderedQuestion = (<div>Not a question type</div>);
                                break;
                        }
                        return (
                            <div key={question.id} className="space-y-2">
                                {renderedQuestion}
                            </div>
                        )
                    })}
                    <Button className="bg-slate-400 rounded hover:bg-slate-150" type="submit">Submit</Button>
                </form>
            </CardContent>
        </Card>
    )
}
