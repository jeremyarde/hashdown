import { StrictMode, useRef, useState } from 'react'
import ReactDOM from 'react-dom/client'
import { BrowserRouter, Routes, Route, Navigate, useParams, Outlet } from 'react-router-dom'
import './index.css'
import { Login } from './Login.tsx'
import { Navbar } from './Navbar.tsx'
import { ListSurveys } from './pages/ListSurveys.tsx'
import { RenderedForm } from './RenderedForm.tsx'
import { Signup } from './Signup.tsx'
import { EditorPage, SampleForms } from './pages/EditorPage.tsx'
import { ListResponses } from './ListResponses.tsx'
import { useGetSurvey } from './hooks/useGetSurvey.ts'
import { NONAME } from 'dns'
import { markdown_to_form_wasm_v2 } from '../../backend/pkg/markdownparser'


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
3. Questions - use any of the following form input types to ask your questions
`;

const simpleSurveyCopy = `# Feedback

text: How did you hear about us?

radio: Can we contact you for follow up questions? making this longer to see if things are bad
- yes
- no

submit: submit`;

var mystyle = {
  formCopy: {
    // 'white-space': 'pre-wrap',
    whiteSpace: 'pre-wrap'
  }
}

const linedPaper = {
  backgroundColor: '#fff',
  backgroundImage:
    'linearGradient(90deg, transparent 79px, #abced4 79px, #abced4 81px, transparent 81px),linearGradient(#eee .1em, transparent .1em)',
  backgroundSize: '100% 1.2em',
}

function HeroSection() {
  let sampleSurvey = markdown_to_form_wasm_v2(simpleSurveyCopy)

  return (
    <div className='p-8'>
      <div className='flex-col flex'>
        <div className='p-6 pb-24'>
          <h2 className='flex top-10 text-center justify-center text-4xl pt-4' style={{ fontWeight: '700', color: 'black' }}>
            The fastest way to create and share surveys.
            <br />
            Write, visualize, share.
          </h2>
          <h1 className='text-xl' style={{ color: 'forestgreen' }}>Hashdown is the easiest text based form maker</h1>
        </div>
        <div className='flex flex-row pt-10 pb-10'>
          <p style={{ whiteSpace: 'pre-wrap' }} className='p-6 text-2xl flex-1 w-1/2 flex-wrap justify-center'>
            {'A few lines of text like this'}
          </p>
          {/* <p style={{ whiteSpace: 'pre-wrap', ...linedPaper }} className='p-6 text-left flex-1 w-1/2 flex-wrap h-full'>
            {simpleSurveyCopy}

          </p> */}
          <div className='p-6 w-1/2 border border-dashed'>
            <ol style={{ whiteSpace: 'pre', wordWrap: 'normal' }}>
              {simpleSurveyCopy.split('\n').map(item => {
                return (
                  <li style={{
                    fontSize: '1rem', listStyleType: 'decimal',
                    textAlign: 'left', color: 'gray', wordWrap: 'normal',
                    wordBreak: 'normal',
                    whiteSpace: 'normal',
                    borderBottom: '1px dashed gray'
                  }}>{item}</li>
                )
              })}
            </ol>
          </div>
        </div>
        <div className='flex flex-row'>
          <p style={{ whiteSpace: 'pre-wrap' }} className='p-6 text-2xl flex-1 w-1/2 flex-wrap justify-center'>

            {'Turns into this'}
          </p>
          {/* <p style={{ whiteSpace: 'pre-wrap' }} className='p-6 text-left flex-1 w-1/2 flex-wrap h-full'>
            {'explanation of the output'}
          </p> */}
          <div className='w-1/2 h-full'>
            <RenderedForm survey={sampleSurvey}></RenderedForm>
          </div>
        </div>
      </div >
    </div>
  )
}

function Home() {
  const [editorContent, setEditorContent] = useState(exampleText);

  return (
    <>
      <HeroSection></HeroSection>
      <div className='flex flex-col pt-8 items-center pb-24'>
        <h4 className='text-xl p-6 w-2/3 rounded'>Give it a try below</h4>
        <a href='/waitlist' className='outline outline-1 p-6 w-2/3 rounded'>Get notified when available</a>
        <hr></hr>
      </div>
      <div className="">
        <h4 className='text-left'>Click on one of the examples</h4>
        <SampleForms setEditorContent={setEditorContent}></SampleForms>
      </div>
      <EditorPage mode={'test'} editorContent={editorContent} setEditorContent={setEditorContent} />
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
