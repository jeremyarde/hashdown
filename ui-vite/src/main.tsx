import React, { StrictMode, createContext, useContext, useState } from 'react'
import ReactDOM from 'react-dom/client'
import { BrowserRouter, Routes, Route, Navigate, useParams, Outlet } from 'react-router-dom'
import './index.css'
import { Login } from './Login.tsx'
import { Navbar } from './Navbar.tsx'
import { ListSurveys } from './pages/ListSurveys.tsx'
import { RenderedForm } from './RenderedForm.tsx'
import { Signup } from './Signup.tsx'
import {  SESSION_TOKEN_KEY } from './lib/constants.ts'
import { EditorPage } from './pages/EditorPage.tsx'
import { ListResponses } from './ListResponses.tsx'
import { useGetSurvey } from './hooks/useGetSurvey.ts'
import { markdown_to_form_wasm_v2 } from '../../backend/pkg/markdownparser'


// export type GlobalState = {
//   sessionId: string | undefined;
//   setSessionId: React.Dispatch<React.SetStateAction<string>>,
//   // refreshToken: string,
//   // setRefreshToken: React.Dispatch<React.SetStateAction<string>>,
// }
// export const GlobalStateContext = createContext({ sessionId: undefined, setSessionId });

// const routerContext = new RouterContext<GlobalState>()
const exampleText = `# User Registration Form

Text: First name [John Dog]

Text: Email Address [john@dog.com]

Textarea: This is nice [Enter your comments here]

checkbox: subscribe?
- [x] Subscribe to newsletter
- [ ] second value here

radio: my radio
- radio button
- another one
- third radio

Dropdown: My question here
  - Option 1
  - Option 2
  - Option 3

Submit: submit`;

function Home() {
  const [editorContent, setEditorContent] = useState(exampleText);

  return (
    <>
      <div className='flex-col flex'>

        <EditorPage mode={'test'} editorContent={editorContent} setEditorContent={setEditorContent} />
        <div className=''>
          <h1 className='flex top-10 text-center justify-center text-xl'>
            The easiest way to create and share surveys.
            <br />
            Create using simple markdown, visualize, publish and share!
          </h1>
        </div>
      </div>
    </>)
}

// type Survey = {
//   id: string;
//   survey_id: string;
//   user_id: string;
//   created_at: Date;
//   modified_at: Date,
//   plaintext: string;
//   version: string;
//   parse_version: string;
// }

function RenderedSurvey() {
  // const data = useLoaderData();
  let { surveyId } = useParams();
  let { survey, error, isPending } = useGetSurvey(surveyId);
  // let globalState: GlobalState = useContext(GlobalStateContext);

  console.log('rendered from url: ' + JSON.stringify(survey));
  console.log(`useGetSurvey: ${JSON.stringify(survey)}, ${error}, ${isPending}`);
  return (<>
    {error && <div>{error}</div>}
    <div>
      {survey &&
        <RenderedForm survey={survey} ></RenderedForm>
      }
    </div >
  </>)
}

export function Layout() {
  return (
    <>
      <Navbar />
      <Outlet />
    </>
  )
}

function Waitlist() {
  let survey = markdown_to_form_wasm_v2(`# Hashdown waitlist :)
Text: Email
textarea: What do you want to use Hashdown for?
Submit: Put me on waitlist
`);
  return (
    <>
      <RenderedForm survey={survey}></RenderedForm>
    </>
  )
}

function App() {
  // const exampleText = '# A survey title here\n- q1\n  - option 1\n  - option 2\n  - option 3\n- question 2\n  - q2 option 1\n  - q2 option 2"';

  const [formtext, setFormtext] = useState(exampleText);
  const [token, setToken] = useState(window.sessionStorage.getItem(SESSION_TOKEN_KEY) ?? '');

  // const globalState: GlobalState = {
  //   sessionId: token,
  //   setSessionId: setToken,
  // };

  return (
    <>
      {/* <GlobalStateContext.Provider value={globalState}> */}
      <BrowserRouter>
        <Routes>
          <Route element={<Layout />}>
            <Route path="/" element={<Home />} />
            <Route path="/editor" element={<EditorPage mode='prod' editorContent={formtext} setEditorContent={setFormtext} />} />
            <Route path="/surveys" element={<ListSurveys />} />
            <Route path='/responses' element={<ListResponses />} />
            <Route path='/waitlist' element={<Waitlist />} />
            <Route path="*" element={<Navigate to="/" />} />
          </Route>
          <Route path='/surveys/:surveyId' element={<RenderedSurvey />} />
          <Route path="/signup" element={<Signup />} />
          <Route path="/login" element={<Login />} />
        </Routes>
      </BrowserRouter>
      {/* </GlobalStateContext.Provider > */}
    </>
  )
}
export type SurveyResponse = {
  id: Number;
  submitted_at: string;
  survey_id: string;
  answers: Map<string, string>;
}
// Render our app!
const rootElement = document.getElementById('root')!

if (!rootElement.innerHTML) {
  const root = ReactDOM.createRoot(rootElement)
  root.render(
    <StrictMode>
      <App />
    </StrictMode >
  )
}
