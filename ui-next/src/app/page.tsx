'use client';

import Editor from "@/components/Editor";
import Login from "@/components/Login";
import Survey from "@/components/Survey";
import { useState } from "react";


export default function Home() {
  const URL = "http://localhost:3000";
  const starttext = '- yea this is cool\n\
  - another one\n\
  - popo\n\
- [checkbox] yo this is not so cool\n\
  - asdfasd\n\
  - asdfasdf\n\
  - oollolol\n\
';
  let [editor, setEditor] = useState(starttext);
  let [survey, setSurvey] = useState(starttext);


  return (
    <main className="flex min-h-screen flex-col items-center justify-between p-24">
      <div className='grid grid-cols-3'>
        {/* <div className='bg-purple-400 min-h-[50px] min-w-[50px]'></div>
        <div className='bg-gray-100 min-h-[50px] min-w-[50px]'></div>
        <div className='bg-teal-300 min-h-[50px] min-w-[50px]'></div>
        <div className='bg-red-200 min-h-[50px] min-w-[50px]'></div>
        <div className='bg-green-400 min-h-[50px] min-w-[50px]'></div>
        <div className='bg-blue-400 min-h-[50px] min-w-[50px]'></div>
        <div className='bg-yellow-400 min-h-[50px] min-w-[50px]'></div>
        <div className='bg-orange-400 min-h-[50px] min-w-[50px]'></div>
        <div className='bg-cyan-400 min-h-[50px] min-w-[50px]'></div> */}
        <Login URL={URL}></Login>
        <Editor URL={URL} editor={editor} setEditor={setEditor} setSurvey={setSurvey}></Editor>
        {JSON.stringify(survey, null, 2)}
        <Survey survey={survey} BACKEND_URL={URL}></Survey>
      </div>
    </main>
  )
}
