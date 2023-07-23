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
// import { RadioGroup, RadioGroupItem } from './components/ui/radio-group'

function App() {
  const [count, setCount] = useState(0)

  return (
    <>
      <GenericForm></GenericForm>
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



function GenericForm() {
  let [survey, setSurvey] = useState(markdown_to_form_wasm("# A survey title here\n- q1\n  - option 1\n  - option 2\n  - option 3\n- question 2\n  - q2 option 1\n  - q2 option 2"));
  // let [survey, setSurvey] = useState(markdown_to_form_wasm("# A survey title here\n- q1\n  - option 1\n  - option 2"));
  // setSurvey(() => markdown_to_form_wasm("# title\n- q1\n  - option 1"));

  // let schema = survey.questions.map();


  let objects = {};
  let defaultValues = {};
  survey.questions.forEach(element => {
    console.log(element);
    // objects[element.id] = z.string().min(10, { message: "Hey, longer please" });
    objects[element.id] = z.enum(element.options.map((option) => option.text), { required_error: "Please select an option" });
    defaultValues[element.id] = "";
  });

  // console.log(`survey: ${JSON.stringify(survey)}`);
  // console.log(`default: ${JSON.stringify(defaultValues)}`);
  // console.log(`objects: ${JSON.stringify(objects)}`);
  // console.log(`keys: ${JSON.stringify(Object.keys(objects))}`);


  const formSchema = z.object(objects);
  // const formSchema = z.object({ username: z.string() });

  // const form = useForm<z.infer<typeof formSchema>>({
  //   resolver: zodResolver(formSchema),
  //   defaultValues: {
  //     username: "",
  //   },
  // });
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
    return (<FormField
      key={question.id}
      control={form.control}
      name={question.id}
      render={({ field }) => {


        // return question.options.map((option) => {
        return (
          <FormItem>
            <FormLabel>{question.value}</FormLabel>
            <FormControl>
              <RadioGroup
                onValueChange={field.onChange}
                defaultValue={field.value}
                className="flex flex-col space-y-1"
              >
                {question.options.map((option) => {
                  return (
                    <FormItem
                      className="flex items-center space-x-3 space-y-0"
                    >
                      <FormControl>
                        <RadioGroupItem value={option.text} />
                      </FormControl>
                      <FormLabel className="font-normal">
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
      }} />)
  }






  return (
    <>
      <h2>{survey.title}</h2>
      <Form {...form}>
        <form onSubmit={form.handleSubmit(onSubmit)} className="w-2/3 space-y-6 flex ">
          {survey.questions.map((question) =>
            formelements(question))
          }
          <Button type="submit" > Submit</Button>
        </form>
      </Form >
      {/* {JSON.stringify(x)}
      {JSON.stringify(survey, null, 2)} */}

      {/* <Form {...form}>
      <form onSubmit={form.handleSubmit(onSubmit)} className="space-y-8">
        <FormField
          control={form.control}
          name="items"
          render={() => (
            <FormItem>
              <div className="mb-4">
                <FormLabel className="text-base">Sidebar</FormLabel>
                <FormDescription>
                  Select the items you want to display in the sidebar.
                </FormDescription>
              </div>
              {items.map((item) => (
                <FormField
                  key={item.id}
                  control={form.control}
                  name="items"
                  render={({ field }) => {
                    return (
                      <FormItem
                        key={item.id}
                        className="flex flex-row items-start space-x-3 space-y-0"
                      >
                        <FormControl>
                          <Checkbox
                            checked={field.value?.includes(item.id)}
                            onCheckedChange={(checked) => {
                              return checked
                                ? field.onChange([...field.value, item.id])
                                : field.onChange(
                                    field.value?.filter(
                                      (value) => value !== item.id
                                    )
                                  )
                            }}
                          />
                        </FormControl>
                        <FormLabel className="font-normal">
                          {item.label}
                        </FormLabel>
                      </FormItem>
                    )
                  }}
                />
              ))}
              <FormMessage />
            </FormItem>
          )}
        />
        <Button type="submit">Submit</Button>
      </form>
    </Form> */}
    </>)
}

export default App
