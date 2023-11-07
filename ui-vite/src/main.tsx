import React, { StrictMode, createContext, useContext, useEffect, useState } from 'react'
import ReactDOM from 'react-dom/client'
import { BrowserRouter, Routes, Route, Navigate, createBrowserRouter, useRouteLoaderData, useLoaderData, Link, useParams, RouterProvider } from 'react-router-dom'
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


export type GlobalState = {
  token: string;
  setToken: React.Dispatch<React.SetStateAction<string>>,
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
        "session_id": `${globalState.token}`
      },
      credentials: 'include',
    });
    console.log(JSON.stringify(response))
    const data = await response.json();
    setSurvey((curr) => data);
  }


  return (<>
    <div>
      {surveyId}
      <br />
      {JSON.stringify(survey, null, 2)}
    </div>
  </>)
}


// const router = createBrowserRouter([
//   {
//     path: "/",
//     element: <App />,
//     children: [
//       {
//         path: "/editor",
//         element: <EditorPage />
//       },
//       {
//         path: "/login",
//         element: <Login />
//       },
//       {
//         path: "/surveys",
//         element: <ListSurveys />,
//         children: [
//           {
//             element: <RenderedSurvey />,
//             path: ":surveyId",
//           }
//         ]
//       }
//     ]
//   }
// ])


function App() {
  const exampleText = '# A survey title here\n- q1\n  - option 1\n  - option 2\n  - option 3\n- question 2\n  - q2 option 1\n  - q2 option 2"';
  // const [formtext, setFormtext] = useState('# A survey title here\n- q1\n  - option 1\n  - option 2\n  - option 3\n- question 2\n  - q2 option 1\n  - q2 option 2"');
  const [formtext, setFormtext] = useState(exampleText);
  const survey = markdown_to_form_wasm(exampleText);
  const [token, setToken] = useState(window.sessionStorage.getItem(SESSION_TOKEN_KEY) ?? '');
  // const [editorContent, setEditorContent] = useState()

  const globalState: GlobalState = {
    token,
    setToken,
  };

  return (
    <>
      <GlobalStateContext.Provider value={globalState}>
        <BrowserRouter>
          <Navbar />
          <Routes>
            <Route path="/" element={<Home />} />
            <Route path="/editor" element={<EditorPage editorContent={formtext} setEditorContent={setFormtext} />} />
            <Route path="/surveys" element={<ListSurveys />} />
            <Route path='/surveys/:surveyId' element={<RenderedSurvey />} />
            <Route path="/login" element={<Login />} />
            <Route path="*" element={<Navigate to="/" />} />
          </Routes>
        </BrowserRouter>
      </GlobalStateContext.Provider>
    </>
  )
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
