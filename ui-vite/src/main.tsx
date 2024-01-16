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
import Dashboard from './pages/Dashboard.tsx'
import { isDev } from './lib/utils.ts'
import { Crud } from './components/Crud.tsx'
import TestPage from './pages/TestPage.tsx'


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

// let simpleSurveyCopy = `# Feedback

// text: How did you hear about us?

// radio: Can we contact you for follow up questions? making this longer to see if things are bad
// - yes
// - no

// submit: submit`;

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
  const [heroContent, setHeroContent] = useState(`# Feedback

text: How did you hear about us?

radio: Can we contact you for follow up questions? making this longer to see if things are bad
- yes
- no

submit: submit`);

  let sampleSurvey = markdown_to_form_wasm_v2(heroContent)

  const updatecontent = (newcontent: string, linenum: number) => {
    let content = heroContent.split('\n');
    content[linenum] = newcontent;
    let allcontent = content.join('\n');
    console.log('allcontent: ', allcontent);
    setHeroContent(allcontent);
  };

  return (
    <div className=''>
      <div className='flex-col flex'>
        <div className='p-6 pb-24'>
          <h2 className='flex top-10 text-center justify-center text-4xl pt-4' style={{ fontWeight: '700', color: 'black' }}>
            The fastest way to create and share surveys.
            <br />
            Write, visualize, share.
          </h2>
          <h1 className='text-xl' style={{ color: 'forestgreen' }}>Hashdown is the easiest text based form maker</h1>
        </div>
        <div className='flex flex-row pt-10 pb-10 pr-10'>
          <p
            style={{ whiteSpace: 'pre-wrap' }}
            className='p-6 text-2xl flex-1 w-1/2 flex-wrap self-center'
          >
            {'A few lines of text like this'}
          </p>
          <div
            className=' w-1/2 h-full'
          >
            <ol style={{ whiteSpace: 'pre', wordWrap: 'normal', backgroundColor: 'white' }}
              className='flex flex-col pl-2 ml-4 border border-dashed bg-white'>
              {heroContent.split('\n').map((item, i) => {
                return (
                  <li className='text-left justify-between min-h-6 text-xl '
                    style={{
                      fontSize: '1rem',
                      // listStyleType: 'decimal',
                      // textAlign: 'left', 
                      // color: 'gray',
                      wordWrap: 'normal',
                      wordBreak: 'normal',
                      whiteSpace: 'normal',
                      borderBottom: '1px dashed gray',
                    }}>
                    <div className='w-full h-full justify-between' >
                      {item}
                    </div>
                  </li>
                )
              })}
            </ol>
            {/* <pre className='h-full'>
              <p className='text-left break-all'>
                {heroContent}
              </p>
            </pre> */}
          </div>
        </div>
        <div className='flex flex-row'>
          <p
            style={{ whiteSpace: 'pre-wrap' }}
            className='p-6 text-2xl w-1/2 flex-wrap justify-center self-center'
          >

            {'Turns into this'}
          </p>
          {/* <p style={{ whiteSpace: 'pre-wrap' }} className='p-6 text-left flex-1 w-1/2 flex-wrap h-full'>
            {'explanation of the output'}
          </p> */}
          <div className='w-1/2 h-full pr-10'>
            <RenderedForm survey={sampleSurvey}></RenderedForm>
          </div>
        </div>
      </div >
    </div >
  )
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

function Home() {
  const [editorContent, setEditorContent] = useState(exampleText);

  return (
    <>
      <HeroSection></HeroSection>
      <div className='flex flex-col pt-8 items-center pb-16'>
        <a href='/waitlist' className='outline outline-1 p-6 w-2/3 rounded'>Join the waitlist</a>
        <h4
          style={{ fontSize: '4rem' }}
          className='p-6 w-2/3 rounded pt-10'>
          Try it below
        </h4>
        <hr></hr>
      </div>
      <div className="p-16">
        <h4 className='text-left'>Click on one of the examples</h4>
        <SampleForms setEditorContent={setEditorContent}></SampleForms>
        <EditorPage mode={'test'} editorContent={editorContent} setEditorContent={setEditorContent} />
      </div>
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

function Faq() {
  const questions = [
    'What is hashdown?',
    'When can I get it?',
    'Who should use hashdown?',
    'How can I use the forms?',
    'Where can I use hashdown forms?',
    'Who is building hashdown?',
  ]
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
