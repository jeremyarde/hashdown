import React, { StrictMode, createContext, useContext, useEffect, useState } from 'react'
import ReactDOM from 'react-dom/client'
import { BrowserRouter, Routes, Route, Navigate, createBrowserRouter, useRouteLoaderData, useLoaderData, Link, useParams, RouterProvider, Outlet } from 'react-router-dom'
import './index.css'
import { Login } from './Login.tsx'
import { Navbar } from './Navbar.tsx'
import { ListSurveys } from './pages/ListSurveys.tsx'
import { RenderedForm } from './RenderedForm.tsx'
import { markdown_to_form_wasm_v2 } from '../../backend/pkg/markdownparser'
import { Signup } from './Signup.tsx'
import { Button } from './components/ui/button.tsx'
import { BASE_URL, SESSION_TOKEN_KEY } from './lib/constants.ts'
import { EditorPage } from './pages/EditorPage.tsx'
import { ListResponses } from './ListResponses.tsx'
import { useGetSurvey } from './hooks/useGetSurvey.ts'


export type GlobalState = {
  sessionId: string;
  setSessionId: React.Dispatch<React.SetStateAction<string>>,
  refreshToken: string,
  setRefreshToken: React.Dispatch<React.SetStateAction<string>>,
}
export const GlobalStateContext = createContext({ token: '', setToken: undefined });

// const routerContext = new RouterContext<GlobalState>()

function Home() {
  return (<>
    <h1 className='flex top-10 text-center justify-center m-12 text-xl'>
      The easiest way to create and share surveys.
      <br />
      Create using simple markdown, visualize, publish and share!
    </h1>
    <EditorPage />
  </>)
}

type Survey = {
  id: string;
  survey_id: string;
  user_id: string;
  created_at: Date;
  modified_at: Date,
  plaintext: string;
  version: string;
  parse_version: string;
}

function RenderedSurvey() {
  // const data = useLoaderData();
  let { surveyId } = useParams();
  let { survey, error, isPending } = useGetSurvey(surveyId);
  let globalState: GlobalState = useContext(GlobalStateContext);

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

function App() {
  // const exampleText = '# A survey title here\n- q1\n  - option 1\n  - option 2\n  - option 3\n- question 2\n  - q2 option 1\n  - q2 option 2"';
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

[Submit]`;

  const [formtext, setFormtext] = useState(exampleText);
  // const survey = markdown_to_form_wasm_v2(exampleText);
  const [token, setToken] = useState(window.sessionStorage.getItem(SESSION_TOKEN_KEY) ?? '');
  // const [editorContent, setEditorContent] = useState()

  const globalState: GlobalState = {
    sessionId: token,
    setSessionId: setToken,
  };

  return (
    <>
      <GlobalStateContext.Provider value={globalState}>
        <BrowserRouter>
          <Routes>
            <Route element={<Layout />}>
              <Route path="/" element={<Home />} />
              <Route path="/editor" element={<EditorPage editorContent={formtext} setEditorContent={setFormtext} />} />
              <Route path="/surveys" element={<ListSurveys />} />
              <Route path='/responses' element={<ListResponses />} />
              <Route path="*" element={<Navigate to="/" />} />
            </Route>
            <Route path='/surveys/:surveyId' element={<RenderedSurvey />} />
            <Route path="/signup" element={<Signup />} />
            <Route path="/login" element={<Login />} />
          </Routes>
        </BrowserRouter>
      </GlobalStateContext.Provider >
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
