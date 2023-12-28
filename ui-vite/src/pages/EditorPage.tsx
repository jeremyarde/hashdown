import { useEffect, useState } from "react";
import { RenderedForm } from "../RenderedForm";
import { useToast } from "@/components/ui/use-toast";
import { markdown_to_form_wasm_v2 } from "../../../backend/pkg/markdownparser";
import { getBaseUrl, getSessionToken, handleResponse } from "@/lib/utils";
import { redirect } from "react-router-dom";

export type EditorProps = {
    mode: "test" | "prod"
    editorContent: string;
    setEditorContent: React.Dispatch<React.SetStateAction<string>>;
}

const userFeedbackTemplate = `# hashdown User Feedback

Textarea: How was your experience using hashdown

Textarea: What is the number one complaint you have?

radio: How did you hear about us?
- Friend
- Online
- Search
Text: Other?

checkbox: How did you hear about us?
- [ ] Option 1
- [ ] Option 2
- [ ] Option 3

radio: Can we contact you?
- yes
- no

Text: Email address

submit: send feedback`;

const userSignupTemplate = `# User Registration Form

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

const emailCaptureSignup = `# Email

Text: Email address

Submit: Get updates`;

const emailMarketingOptOut = `# Unsubscribe to Hashdown
checkbox: I don't want to receive...
- [ ] marketing emails
- [ ] launch alerts
- [ ] new product alerts
- [ ] unsubscribe from all

submit: Submit`;

let tabTemplates = [
    {
        tabname: 'User signup',
        template: userSignupTemplate,
        active: false,
    },
    {
        tabname: 'Product feedback',
        template: userFeedbackTemplate,
        active: false,
    },
    {
        tabname: 'Email capture',
        template: emailCaptureSignup,
        active: false,
    },
    {
        tabname: 'Email opt out',
        template: emailMarketingOptOut,
        active: false,
    }
];

type TabContent = {
    tabname: string;
    template: string;
}

function TabItem({ item, setContentCallback, active, setSelected, index }:
    {
        item: TabContent,
        setContentCallback: React.Dispatch<React.SetStateAction<string>>,
        active: boolean,
        setSelected: Function,
        index: number,
    }
) {
    const onClick = () => {
        setContentCallback(item.template);
        setSelected(index);
    };

    // let sel_style = 'bg-blue p-3';
    // let unsel_style = 'bg-yellow p-3 ';

    return (
        <>
            <div className={active ?
                'bg-gray p-1 text-xs rounded-t-lg flex-grow shadow-md' :
                'bg-gray-light p-1 text-xs rounded-t-md flex-grow shadow-md'}
                onClick={onClick}>{item.tabname}</div>
        </>
    )
}

function SampleForms({ setEditorContent }: { setEditorContent: React.Dispatch<React.SetStateAction<string>> }) {
    const [selected, setSelected] = useState(0);

    return (
        <>
            <div className='flex w-full'>
                {tabTemplates.map((template: TabContent, i) => {
                    // console.log('making tabs', selected);
                    // const styleName = selected[i] === true ? "bg-green p-3" : "bg-blue p-3";
                    return (
                        <>
                            <TabItem key={i} index={i} active={selected === i} setSelected={setSelected} item={template} setContentCallback={setEditorContent}></TabItem>
                            {/* <div className="bg-blue p-3 active:bg-purple" onClick={(evt) => tabClick(i, template)}>{template.tabname}</div> */}
                        </>
                    )
                })}
                {/* <div className='bg-blue p-3' onClick={(evt) => setEditorContent(userFeedbackTemplate)}>User signup</div>
                <div className='bg-blue p-3' onClick={(evt) => setEditorContent(userFeedbackTemplate)}>Product feedback</div>
                <div className='bg-blue p-3' onClick={(evt) => setEditorContent(emailCaptureSignup)}>Email capture</div>
                <div className='bg-blue p-3' onClick={(evt) => setEditorContent(userFeedbackTemplate)}>Email opt out</div> */}
            </div >
        </>
    );
}

export function EditorPage({ mode = "test", editorContent, setEditorContent }: EditorProps) {
    const { toast } = useToast()
    console.log('editorContent: ' + editorContent);
    const [survey, setSurvey] = useState(markdown_to_form_wasm_v2(editorContent));

    useEffect(() => {
        console.log('editor useeffect');
        const newSurvey = markdown_to_form_wasm_v2(editorContent);
        setSurvey(newSurvey);
    }, [editorContent]);
    // const [token, setToken] = useState('');

    async function submitSurvey(event: React.MouseEvent<HTMLElement>) {
        const response = await fetch(`${getBaseUrl()}/surveys`, {
            method: "POST",
            // credentials: 'include',
            headers: {
                'content-type': 'application/json',
                'session_id': getSessionToken(),
            },
            body: JSON.stringify({ plaintext: editorContent })
        });
        handleResponse(response);
        const result = await response.json();

        if (response.status === 401) {
            toast({
                title: "Submit Survey: Failed",
                description: result.message
            })
            // redirect("/login");
        }

        if (response.status === 200) {
            toast({
                title: "Submit Survey: Success",
                description: result.message
            })
        }
    };

    return (
        <>
            <div className="w-full flex h-full flex-wrap">
                <div className="w-1/2 p-4 overflow-auto flex-wrap">
                    <h2 className="text-2xl font-bold">Enter Form Content</h2>
                    <SampleForms setEditorContent={setEditorContent}></SampleForms>
                    <textarea
                        className="w-full h-4/6 p-2 rounded border border-gray-300"
                        placeholder="Enter form content here..."
                        value={editorContent}
                        onChange={evt => setEditorContent(evt.target.value)} />
                    <div className="flex flex-row">
                        <button className="bg-gray-200 border p-1 w-full" onClick={submitSurvey}>Save Survey</button>
                        <button className="bg-green-200 border w-full p-1 flex-1" onClick={submitSurvey}>Publish</button>
                    </div>
                </div>
                <div className="w-1/2 flex-wrap">
                    <h2 className="text-2xl font-bold">Preview</h2>
                    <RenderedForm survey={survey} mode={mode} ></RenderedForm>
                </div>
            </div>
        </>
    );
}