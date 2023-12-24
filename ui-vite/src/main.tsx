import { StrictMode, useState } from 'react'
import ReactDOM from 'react-dom/client'
import { BrowserRouter, Routes, Route, Navigate, useParams, Outlet } from 'react-router-dom'
import './index.css'
import { Login } from './Login.tsx'
import { Navbar } from './Navbar.tsx'
import { ListSurveys } from './pages/ListSurveys.tsx'
import { RenderedForm } from './RenderedForm.tsx'
import { Signup } from './Signup.tsx'
import { EditorPage } from './pages/EditorPage.tsx'
import { ListResponses } from './ListResponses.tsx'
import { useGetSurvey } from './hooks/useGetSurvey.ts'


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

const formTypesCopy = `Available input types:
- Text
- Textarea
- Checkbox
- Radio
- Submit (required)`;

const formRulesCopy = `Each survey needs 3 things.
1. Title - Title is found a the top, and starts with #
2. Submit - The submit button can be placed anywhere in the form, and will send the current form data to be saved
3. Questions - use any of the folling form input types to ask your questions
`;

var mystyle = {
  formCopy: {
    // 'white-space': 'pre-wrap',
    whiteSpace: 'pre-wrap'
  }

}

function Home() {
  const [editorContent, setEditorContent] = useState(exampleText);

  return (
    <>
      <div className='flex-col flex'>
        <EditorPage mode={'test'} editorContent={editorContent} setEditorContent={setEditorContent} />
        <div className=''>
          <h2 className='flex top-10 text-center justify-center text-xl'>
            The easiest way to create and share surveys.
            <br />
            Create using simple markdown, visualize, publish and share!
          </h2>
        </div>
        <div className='flex'>
          <p style={{ whiteSpace: 'pre-wrap' }} className='p-6 text-left flex-1 w-1/2 flex-wrap h-full'>
            {formRulesCopy}
          </p>
          <p style={{ whiteSpace: 'pre-wrap' }} className='p-6 text-left flex-1 w-1/2 flex-wrap h-full'>
            {formTypesCopy}
          </p>
        </div>
      </div >
    </>)
}

function RenderedSurvey() {
  let { surveyId } = useParams();
  let { survey, error, isPending } = useGetSurvey(surveyId);

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
  const { survey, error, isPending } = useGetSurvey("k3itjqi4mxhq");
  return (
    <>
      {survey &&
        <RenderedForm survey={survey}></RenderedForm>
      }
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
