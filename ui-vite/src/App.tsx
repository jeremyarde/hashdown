import { useState } from 'react'
import reactLogo from './assets/react.svg'
import viteLogo from '/vite.svg'
import './App.css'
import { zodResolver } from "@hookform/resolvers/zod"

import { useForm } from 'react-hook-form';

import { Button } from './components/ui/button'
import * as z from "zod";

// import markdownparser
import { markdown_to_form_wasm } from '../../backend/pkg/markdownparser';
import { Form, FormControl, FormDescription, FormField, FormItem, FormLabel, FormMessage } from './components/ui/form'
import { Input } from './components/ui/input'

function App() {
  const [count, setCount] = useState(0)

  return (
    <>
      <GenericForm></GenericForm>
      <Button onClick={() => setCount((count) => count + 1)}>Click me</Button>
    </>
  )
}


function GenericForm() {
  let [survey, setSurvey] = useState(markdown_to_form_wasm("- test\n  - option"));

  // let schema = survey.questions.map();


  let objects = {};
  let x = survey.questions.forEach(element => {
    console.log(element);
    objects[element.id] = z.string().min(10, { message: "Hey, longer please" });
  });
  // const formSchema = z.object(objects);
  const formSchema = z.object({ username: z.string() });

  const form = useForm<z.infer<typeof formSchema>>({
    resolver: zodResolver(formSchema)
  });


  function onSubmit(values: z.infer<typeof formSchema>) {
    // Do something with the form values.
    // ✅ This will be type-safe and validated.
    console.log(values)
  }

  return (
    <>
      <Form {...form}>
        <form onSubmit={form.handleSubmit(onSubmit)} className="space-y-8">
          <FormField
            control={form.control}
            name="username"
            render={({ field }) => (
              <FormItem>
                <FormLabel>Username</FormLabel>
                <FormControl>
                  <Input placeholder="shadcn" {...field} />
                </FormControl>
                <FormDescription>
                  This is your public display name.
                </FormDescription>
                <FormMessage />
              </FormItem>
            )}
          />
          <Button type="submit">Submit</Button>
        </form>
      </Form>
      {JSON.stringify(x)}
      {JSON.stringify(survey, null, 2)}
    </>)
}

export default App
