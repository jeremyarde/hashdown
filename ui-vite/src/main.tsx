import React, { StrictMode, createContext, useContext, useEffect, useState } from 'react'
import ReactDOM from 'react-dom/client'
import { BrowserRouter, Routes, Route, Navigate, createBrowserRouter, useRouteLoaderData, useLoaderData, Link, useParams, RouterProvider, Outlet } from 'react-router-dom'
import './index.css'
import { Login } from './Login.tsx'
import { Navbar } from './Navbar.tsx'
import { ListSurveys } from './pages/ListSurveys.tsx'
import { RenderedForm } from './RenderedForm.tsx'
import { markdown_to_form_wasm } from '../../backend/pkg/markdownparser'
import { Signup } from './Signup.tsx'
import { Button } from './components/ui/button.tsx'
import { BASE_URL, SESSION_TOKEN_KEY } from './lib/constants.ts'
import { EditorPage } from './pages/EditorPage.tsx'
import { ListResponses } from './ListResponses.tsx'


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
      Create using simple markdown format, visualize your survey, publish and share!
    </h1>
    <EditorPage />
    <Button className='bg-blue-400 rounded-lg p-6'>
      <Link to="/login">
        Get started
      </Link>
    </Button>
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
  const [survey, setSurvey] = useState();
  let globalState: GlobalState = useContext(GlobalStateContext);

  useEffect(() => {
    getSurvey(surveyId);
  }, [surveyId]);

  const getSurvey = async (surveyId) => {
    const response = await fetch(`${BASE_URL}/surveys/${surveyId}`, {
      method: "GET",
      headers: {
        "Content-Type": "application/json",
        "session_id": `${globalState.sessionId}`
      },
      credentials: 'include',
    });
    console.log(JSON.stringify(response))
    const data: Survey = await response.json();
    const fullSurvey = {
      ...markdown_to_form_wasm(data.plaintext),
      ...data
    }
    setSurvey((prev) => fullSurvey);
  }


  return (<>
    <div>
      {surveyId}
      <br />
      {JSON.stringify(survey, null, 2)}
      <hr />
      {survey &&
        <RenderedForm plaintext={survey.plaintext} survey={survey} ></RenderedForm>
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
  const exampleText = '# A survey title here\n- q1\n  - option 1\n  - option 2\n  - option 3\n- question 2\n  - q2 option 1\n  - q2 option 2"';
  // const [formtext, setFormtext] = useState('# A survey title here\n- q1\n  - option 1\n  - option 2\n  - option 3\n- question 2\n  - q2 option 1\n  - q2 option 2"');
  const [formtext, setFormtext] = useState(exampleText);
  const survey = markdown_to_form_wasm(exampleText);
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
              <Route path='/surveys/:surveyId' element={<RenderedSurvey />} />
              <Route path='/responses' element={<ListResponses />} />
              <Route path="*" element={<Navigate to="/" />} />
            </Route>
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
