import { StrictMode, useRef, useState } from 'react'
import ReactDOM from 'react-dom/client'
import { BrowserRouter, Routes, Route, Navigate, useParams, Outlet } from 'react-router-dom'
import './index.css'
import { Login } from './components/custom/Login.tsx'
import { Navbar } from './components/custom/Navbar.tsx'
import { ListSurveys } from './pages/ListSurveys.tsx'
import { RenderedForm } from './components/custom/RenderedForm.tsx'
import { Signup } from './components/custom/Signup.tsx'
import { EditorPage } from './pages/EditorPage.tsx'
import { ListResponses } from './components/custom/ListResponses.tsx'
import { useGetSurvey } from './hooks/useGetSurvey.ts'
import { NONAME } from 'dns'
import Dashboard from './pages/Dashboard.tsx'
import { getStage, isDev } from './lib/utils.ts'
import { Crud } from './components/Crud.tsx'
import TestPage from './pages/TestPage.tsx'
import { Home } from './pages/Home.tsx'
import { Waitlist } from './components/custom/Waitlist.tsx'
import { getHeapStatistics } from 'v8'
import { STAGE } from './lib/constants.ts'


export const exampleText = `# User Registration Form

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

const formTypesCopy = `Available input types:
- Text
- Textarea
- Checkbox
- Radio
- Submit (required)`;

const formRulesCopy = `Each survey needs 3 things.
1. Title - Title is found a the top, and starts with #
2. Submit - The submit button can be placed anywhere in the form, and will send the current form data to be saved
3. Questions - use any of the following form input types to ask your questions
`;

const linedPaper = {
  backgroundColor: '#fff',
  backgroundImage:
    'linearGradient(90deg, transparent 79px, #abced4 79px, #abced4 81px, transparent 81px),linearGradient(#eee .1em, transparent .1em)',
  backgroundSize: '100% 1.2em',
}

function TestEditor() {
  const [content, setContent] = useState('starting content');


  return (
    <div contentEditable style={{
      minHeight: '50px',
      // width: '300px',
      backgroundColor: 'white'
    }} className='w-full text-left'
    // onChange={(evt) => setContent(evt.target.value)}
    >
      {content}
      <div className='h-2 w-2 bg-purple'></div>
    </div>
  )
}

function RenderedSurvey() {
  let { surveyId } = useParams();
  let { survey, error, isPending } = useGetSurvey(surveyId);

  return (<>
    {error && <div>{error}</div>}
    <div>
      {survey &&
        <RenderedForm mode="prod" survey={survey} ></RenderedForm>
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
  const [formtext, setFormtext] = useState(exampleText);

  return (
    <>
      <BrowserRouter>
        <Routes>
          <Route element={<Layout />}>
            <Route path="/" element={<Home />} />
            <Route path="/editor" element={<EditorPage mode={getStage() === STAGE.PROD ? 'prod' : 'test'} editorContent={formtext} setEditorContent={setFormtext} />} />
            <Route path="/surveys" element={<ListSurveys />} />
            <Route path='/responses' element={<ListResponses />} />
            <Route path='/waitlist' element={<Waitlist />} />
            <Route path='/dashboard' element={<Dashboard />} />
            <Route path='/dev' element={<Crud />} />
            <Route path='/test' element={<TestPage />} />
            <Route path="*" element={<Navigate to="/" />} />
          </Route>
          <Route path='/surveys/:surveyId' element={<RenderedSurvey />} />
          <Route path="/signup" element={<Signup />} />
          <Route path="/login" element={<Login />} />
        </Routes>
      </BrowserRouter>
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
