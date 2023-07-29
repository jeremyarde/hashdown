import { useState } from 'react'
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
// import { RadioGroup, RadioGroupItem } from './components/ui/radio-group'

function App() {
  const [formtext, setFormtext] = useState('# A survey title here\n- q1\n  - option 1\n  - option 2\n  - option 3\n- question 2\n  - q2 option 1\n  - q2 option 2"');
  const survey = markdown_to_form_wasm(formtext);

  return (
    <>
      <div className='grid grid-cols-2 gap-5'>
        <Textarea className='w-full' cols={10} value={formtext} onChange={(evt) => {
          let value = evt.target.value;
          console.log(`Set text value: ${JSON.stringify(value)}`);
          setFormtext(value);
        }} />
        <GenericForm key={'testkey'} plaintext={formtext} survey={survey}></GenericForm>

      </div>
    </>
  )
}

/**
 * The complete Triforce, or one or more components of the Triforce.
 * @typedef {Object} Question
 * @property {string} value - Indicates whether the Courage component is present.
 * @property {string} id - Indicates whether the Power component is present.
 * @property {Array<Option>} options - Indicates whether the Wisdom component is present.
 */


/**
 * The complete Triforce, or one or more components of the Triforce.
 * @typedef {Object} Option
 * @property {string} text - Indicates whether the Courage component is present.
 * @property {string} id - Indicates whether the Power component is present.
 */



function GenericForm({ plaintext, survey }) {
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
        className="flex text-left"
        render={({ field }) => {


          // return question.options.map((option) => {
          return (
            <FormItem>
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
                      // className="flex items-center space-x-3 space-y-0"
                      >
                        <FormControl>
                          <RadioGroupItem value={option.text} />
                        </FormControl>
                        <FormLabel
                        // className="font-normal"
                        >
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
    <><div className='flex flex-col items-start'>
      <h2>id: {survey.id}</h2>
      <h2>{survey.title}</h2>
      <Form {...form}>
        <form onSubmit={form.handleSubmit(onSubmit)}
          className="space-y-6 justify-start items-start"
        >
          {survey.questions.map((question) =>
            formelements(question))
          }
          <Button className="" type="submit" >Submit</Button>
        </form>
      </Form >
    </div>
    </>)
}

export default App
