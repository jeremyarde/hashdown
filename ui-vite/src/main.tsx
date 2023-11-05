import React, { StrictMode, createContext, useEffect, useState } from 'react'
import ReactDOM from 'react-dom/client'
import { BrowserRouter, Routes, Route, Navigate } from 'react-router-dom'
import './index.css'
import { Login } from './Login.tsx'
import { Navbar } from './Navbar.tsx'
import { ListSurveys } from './pages/ListSurveys.tsx'
import { Editor } from './pages/Editor.tsx'
import { RenderedForm } from './RenderedForm.tsx'
import { markdown_to_form_wasm } from '../../backend/pkg/markdownparser'
import { Signup } from './Signup.tsx'
import { Button } from './components/ui/button.tsx'
import { SESSION_TOKEN_KEY } from './lib/constants.ts'


export type GlobalState = {
  token: string;
  setToken: React.Dispatch<React.SetStateAction<string>>,
}
export const GlobalStateContext = createContext({ token: '', setToken: undefined });

// const routerContext = new RouterContext<GlobalState>()

function Home() {
  return (<>
    <h1 className='flex top-10 text-center justify-center m-12'>The easiest way to create and share surveys</h1>
    <a href='/editor'>
      <Button className='bg-blue-400 rounded-lg'>
        Get started
      </Button>
    </a>
  </>)
}

function Test() {

  return (<>
    <div>Yo this better work</div>
  </>)
}

function App() {
  const [formtext, setFormtext] = useState('# A survey title here\n- q1\n  - option 1\n  - option 2\n  - option 3\n- question 2\n  - q2 option 1\n  - q2 option 2"');
  const survey = markdown_to_form_wasm(formtext);
  const [token, setToken] = useState(window.sessionStorage.getItem(SESSION_TOKEN_KEY) ?? '');
  // const [editorContent, setEditorContent] = useState()

  const globalState: GlobalState = {
    token,
    setToken,
  };

  return (
    <>
      {/* <GlobalStateContext.Provider value={globalState}> */}
      <BrowserRouter>
        <Navbar />
        <Routes>
          <Route path='/test' element={<Test />} />
          <Route path="/" element={<Home />} />
          <Route path="/editor" element={<Editor editorContent={formtext} setEditorContent={setFormtext} />} />
          <Route path="/surveys" element={<ListSurveys />} />
          <Route path='/surveys/:survey_id' element={<RenderedForm survey={survey} plaintext={formtext} />} />
          <Route path="*" element={<Navigate to="/" />} />
        </Routes>
      </BrowserRouter>
      {/* </GlobalStateContext.Provider> */}
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
