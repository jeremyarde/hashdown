import { useEffect, useState } from 'react'
import './App.css'
import { zodResolver } from "@hookform/resolvers/zod"

import { useForm } from 'react-hook-form';

import { Button } from './components/ui/button'
import * as z from "zod";

// import markdownparser
import { markdown_to_form_wasm } from '../../backend/pkg/markdownparser';
import { Form, FormControl, FormDescription, FormField, FormItem, FormLabel, FormMessage } from './components/ui/form'
import { Input } from './components/ui/input'
import { RadioGroup, RadioGroupItem } from './components/ui/radio-group';
import { Textarea } from './components/ui/textarea';
import { Alert, AlertDescription, AlertTitle } from './components/ui/alert';


const base_url = "http://localhost:3000/api";

function ListSurveys() {
  const [surveys, setSurveys] = useState(undefined);
  const [error, setError] = useState('');

  async function getSurveys() {
    const response = await fetch(`${base_url}/surveys`, {
      method: "GET",
      credentials: 'same-origin'
    });

    const result = await response.json();
    console.log('data: ', result);
    if (result.error) {
      console.log('failed to get surveys: ', result);
      setError(result.message ?? 'Generic error getting surveys');
    } else {
      setSurveys(result);
    }
  }

  return (
    <>
      <div className='bg-green-300'>

        <Button onClick={(evt) => {
          console.log('clicked button');
          // setError('');
          getSurveys();
        }}>My Surveys</Button>
        <div>
          Surveys
          {[surveys]}
        </div>
        <div className='bg-red-600'>
          Errors?
          <div>
            {error ? error : 'No errors'}
          </div>
        </div>
      </div>
    </>
  )

}



function Login() {
  const formSchema = z.object({
    email: z.string().min(2).max(50),
    password: z.string()
  });

  const form = useForm<z.infer<typeof formSchema>>({
    resolver: zodResolver(formSchema),
    defaultValues: {
      email: "",
      password: "",
    },
  })
  const onSubmit = async (evt) => {
    console.log(evt);
    try {
      const response = await fetch(`${base_url}/auth/login`, {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        credentials: 'same-origin',
        body: JSON.stringify(evt),
      });

      const result = await response.json();
      const headers = await response.headers;
      console.log("Success:", result, headers);
    } catch (error) {
      console.error("Error:", error);
    }

  }

  return (
    <>
      <Form {...form}>
        <form onSubmit={form.handleSubmit(onSubmit)} className="space-y-1">
          <FormField
            control={form.control}
            name="email"
            render={({ field }) => (
              <FormItem>
                <FormLabel>Username</FormLabel>
                <FormControl>
                  <Input placeholder="shadcn" {...field} />
                </FormControl>

                <FormMessage />
              </FormItem>
            )}
          />
          <FormField
            control={form.control}
            name="password"
            render={({ field }) => (
              <FormItem>
                <FormLabel>password</FormLabel>
                <FormControl>
                  <Input placeholder="shadcn" {...field} />
                </FormControl>
                {/* <FormDescription>
                  password here
                </FormDescription> */}
                <FormMessage />
              </FormItem>
            )}
          />
          <Button type="submit">Submit</Button>
        </form>
      </Form>
    </>
  );
}


function App() {
  const [formtext, setFormtext] = useState('# A survey title here\n- q1\n  - option 1\n  - option 2\n  - option 3\n- question 2\n  - q2 option 1\n  - q2 option 2"');
  const survey = markdown_to_form_wasm(formtext);

  return (
    <>
      <div className='grid grid-cols-2 gap-5'>
        <div>
          <Login></Login>
          <ListSurveys></ListSurveys>
        </div>
        {/* <div>
          <Textarea className='w-full h-full' cols={10} value={formtext} onChange={(evt) => {
            let value = evt.target.value;
            console.log(`Set text value: ${JSON.stringify(value)}`);
            setFormtext(value);
          }} />
        </div>
        <GenericForm key={'testkey'} plaintext={formtext} survey={survey}></GenericForm> */}

      </div>
    </>
  )
}

/**
 * The complete Triforce, or one or more components of the Triforce.
 * @typedef {Object} Option
 * @property {string} text - Indicates whether the Courage component is present.
 * @property {string} id - Indicates whether the Power component is present.
 */
function GenericForm({ plaintext, survey }) {
  let [submittedValues, setSubmittedValues] = useState(null);
  // let [survey, setSurvey] = useState(markdown_to_form_wasm("# A survey title here\n- q1\n  - option 1\n  - option 2\n  - option 3\n- question 2\n  - q2 option 1\n  - q2 option 2"));
  // let [survey, setSurvey] = useState(markdown_to_form_wasm("# A survey title here\n- q1\n  - option 1\n  - option 2"));
  // let [survey, setSurvey] = useState(markdown_to_form_wasm(plaintext));
  // setSurvey(() => markdown_to_form_wasm("# title\n- q1\n  - option 1"));

  // let schema = survey.questions.map();

  // const survey = markdown_to_form_wasm(plaintext);
  let objects = {};
  let defaultValues = {};
  survey.questions.forEach(element => {
    console.log(element);
    // objects[element.id] = z.string().min(10, { message: "Hey, longer please" });
    objects[element.id] = z.enum(element.options.map((option) => option.text), { required_error: "Please select an option" });
    defaultValues[element.id] = "";
  });
  const formSchema = z.object(objects);


  const form = useForm<z.infer<typeof formSchema>>({
    resolver: zodResolver(formSchema),
    defaultValues: defaultValues,
  });


  function onSubmit(values: z.infer<typeof formSchema>) {
    // Do something with the form values.
    // ✅ This will be type-safe and validated.
    console.log(`submit:`)
    console.log(values);
    setSubmittedValues((prev) => values);

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
        className="flex text-left justify-start"
        render={({ field }) => {


          // return question.options.map((option) => {
          return (
            <FormItem className='flex flex-col w-full text-left'>
              <FormLabel>{question.value}</FormLabel>
              <FormControl>
                <RadioGroup
                  onValueChange={field.onChange}
                  defaultValue={field.value}
                // className="flex flex-col space-y-6"
                >
                  {question.options.map((option) => {
                    return (
                      <FormItem
                        className="flex items-center space-x-3 w-full"
                      >
                        <FormControl>
                          <RadioGroupItem value={option.text} />
                        </FormControl>
                        <FormLabel className=''>
                          {option.id}: {option.text}
                        </FormLabel>
                      </FormItem>
                    )
                  })}
                </RadioGroup>
              </FormControl>
              <FormDescription>
                {question.id}
              </FormDescription>
              <FormMessage />
            </FormItem>
          )
        }}
      />
    )
  }

  return (
    <>
      <div className='flex flex-col justify-start items-start w-full'>
        <h2 className='w-full text-left'>id: {survey.id} - {survey.title}</h2>
        <Form {...form}>
          <form onSubmit={form.handleSubmit(onSubmit)}
            className="space-y-6 w-full"
          >
            {survey.questions.map((question) =>
              formelements(question))
            }
            <Button className="bg-slate-400 rounded hover:bg-slate-150" type="submit" >Submit</Button>
          </form>
        </Form >
      </div>
      <Alert>
        {/* <Terminal className="h-4 w-4" /> */}
        <AlertTitle>Heads up!</AlertTitle>
        <AlertDescription>
          {submittedValues}
        </AlertDescription>
      </Alert>
    </>)
}

export default App
