import { useEffect, useState } from "react";
import { zodResolver } from "@hookform/resolvers/zod";
import { useForm } from 'react-hook-form';
import { Button } from './components/ui/button';
import { Form, FormControl, FormDescription, FormField, FormItem, FormLabel, FormMessage } from './components/ui/form';
import { RadioGroup, RadioGroupItem } from './components/ui/radio-group';
import { Alert, AlertDescription, AlertTitle } from './components/ui/alert';
import * as z from "zod";

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
    let [submittedValues, setSubmittedValues] = useState(null);
    // let [form, setForm] = useState(undefined);

    console.log(`render - survey: ${JSON.stringify(survey)}`)
    let objects = {};
    let defaultValues = {};
    const questions = survey?.questions ?? [];
    questions.forEach(element => {
        // console.log(element);
        // objects[element.id] = z.string().min(10, { message: "Hey, longer please" });
        objects[element.id] = z.enum(element.options.map((option) => option.text), { required_error: "Please select an option" });
        defaultValues[element.id] = "";
    });
    const formSchema = z.object(objects);
    const form = useForm<z.infer<typeof formSchema>>({
        resolver: zodResolver(formSchema),
        defaultValues: defaultValues,
    });

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
                                <Badge className="bg-gray-800 text-white" variant="outline">{question.id}</Badge>

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
                            {/* <FormDescription>
                                {question.id}
                            </FormDescription> */}
                            <FormMessage />
                        </FormItem>
                    );
                }} />
        );
    }

    let toRender;
    if (survey) {
        toRender = (<>
            <div className='flex flex-col justify-start items-start w-full'>
                <h1 className='w-full text-center'>{survey.title}</h1>
                <Badge className="bg-gray-800 text-white" variant="outline">{survey.survey_id}</Badge>
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
        </>)
    } else {
        <div>nope</div>
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

export function ExampleForm() {
    return (
        <Card>
            <CardContent>
                <form className="space-y-4">
                    <div className="space-y-2">
                        <Label htmlFor="textInput">Text Input</Label>
                        <Input id="textInput" placeholder="Enter text" />
                    </div>
                    <div className="space-y-2">
                        <Label htmlFor="dropdown">Dropdown</Label>
                        <Select>
                            <SelectTrigger id="dropdown">
                                <SelectValue placeholder="Select" />
                            </SelectTrigger>
                            <SelectContent>
                                <SelectItem value="option1">Option 1</SelectItem>
                                <SelectItem value="option2">Option 2</SelectItem>
                                <SelectItem value="option3">Option 3</SelectItem>
                            </SelectContent>
                        </Select>
                    </div>
                    <div className="space-y-2">
                        <Label className="font-semibold">Checkbox Options</Label>
                        <div className="flex flex-col space-y-2">
                            <div className="flex items-center">
                                <Checkbox id="option1" name="checkbox" />
                                <Label className="ml-2 text-sm" htmlFor="option1">
                                    Option 1
                                </Label>
                            </div>
                            <div className="flex items-center">
                                <Checkbox id="option2" name="checkbox" />
                                <Label className="ml-2 text-sm" htmlFor="option2">
                                    Option 2
                                </Label>
                            </div>
                            <div className="flex items-center">
                                <Checkbox id="option3" name="checkbox" />
                                <Label className="ml-2 text-sm" htmlFor="option3">
                                    Option 3
                                </Label>
                            </div>
                        </div>
                    </div>
                    <div className="space-y-2">
                        <Label className="font-semibold">Radio Options</Label>
                        <RadioGroup defaultValue="option1">
                            <Label className="flex items-center space-x-2" htmlFor="option1">
                                <RadioGroupItem id="option1" value="option1" />
                                <span>Option 1</span>
                            </Label>
                            <Label className="flex items-center space-x-2" htmlFor="option2">
                                <RadioGroupItem id="option2" value="option2" />
                                <span>Option 2</span>
                            </Label>
                        </RadioGroup>
                    </div>
                </form>
            </CardContent>
        </Card>
    )
}
