import { useEffect, useState } from "react";
import { RenderedForm } from "../components/custom/RenderedForm";
import { useToast } from "@/components/ui/use-toast";
import { markdown_to_form_wasm_v2 } from "../../../backend/pkg/markdownparser";
import { getBaseUrl, getSessionToken, handleResponse } from "@/lib/utils";
import { redirect } from "react-router-dom";
import { clsx } from "clsx";

import { Button } from "@/components/ui/button";
import { MARKDOWN_RULES } from "@/lib/constants";

export type EditorProps = {
  mode: "test" | "prod";
  editorContent: string;
  setEditorContent: React.Dispatch<React.SetStateAction<string>>;
};

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
    tabname: "User signup",
    template: userSignupTemplate,
    active: false,
  },
  {
    tabname: "Product feedback",
    template: userFeedbackTemplate,
    active: false,
  },
  {
    tabname: "Email capture",
    template: emailCaptureSignup,
    active: false,
  },
  {
    tabname: "Email opt out",
    template: emailMarketingOptOut,
    active: false,
  },
];

type TabContent = {
  tabname: string;
  template: string;
};

function TabItem({
  item,
  setContentCallback,
  active,
  setSelected,
  index,
}: {
  item: TabContent;
  setContentCallback: React.Dispatch<React.SetStateAction<string>>;
  active: boolean;
  setSelected: Function;
  index: number;
}) {
  const onClick = () => {
    setContentCallback(item.template);
    setSelected(index);
  };

  // let sel_style = 'bg-blue p-3';
  // let unsel_style = 'bg-yellow p-3 ';

  return (
    <>
      <div
        style={{ backgroundColor: active ? "rgb(19, 206, 102)" : "#FFFFFF" }}
        className={
          active
            ? "outline outline-1 p-1 m-1 text-xs rounded flex-grow bg-green"
            : "outline outline-1 p-1 m-1 text-xs rounded flex-grow"
        }
        onClick={onClick}
      >
        {item.tabname}
      </div>
    </>
  );
}

export function SampleForms({
  setEditorContent,
}: {
  setEditorContent: React.Dispatch<React.SetStateAction<string>>;
}) {
  const [selected, setSelected] = useState(0);

  return (
    <>
      <div className="flex w-full">
        {tabTemplates.map((template: TabContent, i) => {
          return (
            <>
              <TabItem
                key={i}
                index={i}
                active={selected === i}
                setSelected={setSelected}
                item={template}
                setContentCallback={setEditorContent}
              ></TabItem>
            </>
          );
        })}
      </div>
    </>
  );
}

export function EditorPage({
  mode = "test",
  editorContent,
  setEditorContent,
}: EditorProps) {
  const { toast } = useToast();
  const [survey, setSurvey] = useState(markdown_to_form_wasm_v2(editorContent));
  const [hidden, setHidden] = useState(false);
  const [rules, setRules] = useState(false);

  useEffect(() => {
    const newSurvey = markdown_to_form_wasm_v2(editorContent);
    setSurvey(newSurvey);
  }, [editorContent]);
  // const [token, setToken] = useState('');

  async function submitSurvey(event: React.MouseEvent<HTMLElement>) {
    const response = await fetch(`${getBaseUrl()}/v1/surveys`, {
      method: "POST",
      // credentials: 'include',
      headers: {
        "content-type": "application/json",
        session_id: getSessionToken(),
      },
      body: JSON.stringify({ plaintext: editorContent }),
    });
    handleResponse(response);
    const result = await response.json();

    if (response.status === 401) {
      toast({
        title: "Submit Survey: Failed",
        description: result.message,
      });
      // redirect("/login");
    }

    if (response.status === 200) {
      toast({
        title: "Submit Survey: Success",
        description: result.message,
      });
    }
  }

  return (
    <>
      <div className="flex flex-col flex-wrap w-full h-full md:flex-row">
        <div className="flex flex-col flex-1 w-full p-2 md:w-1/2">
          <textarea
            className="w-full p-2 border border-solid rounded-xl"
            style={{ height: "65vh" }}
            placeholder="Enter form content here..."
            value={editorContent}
            onChange={(evt) => setEditorContent(evt.target.value)}
          />
          <div className="pb-2">
            <button
              disabled={!survey.validation[0]}
              className={clsx(
                "p-2 h-full w-full border border-solid",
                survey.validation[0] ? "bg-green" : "bg-gray opacity-50"
              )}
              onClick={submitSurvey}
            >
              Save Survey
            </button>
          </div>
          <div>
            <ul>
              {survey.validation[1].map((error: string) => (
                <li className="bg-yellow">{error}</li>
              ))}
            </ul>
          </div>
          {/* <div>
                        {survey.validation[0] === true ?
                            <div className="bg-green">Valid</div> :
                            <>
                                <div className="bg-red-600">Not valid</div>
                                {survey.validation[1].map((error: string) =>
                                    <div>{error}</div>
                                )}
                            </>
                        }
                    </div> */}
          {/* Toolbar*/}
          <div className="flex flex-row w-full">
            <Button
              className="border border-solid  bg-yellow"
              onClick={() => (rules ? setRules(false) : setRules(true))}
            >
              {rules ? "hide rules" : "show rules"}
            </Button>
            <Button
              className="border border-solid  bg-yellow"
              onClick={() => (hidden ? setHidden(false) : setHidden(true))}
            >
              {hidden ? "hide examples" : "show examples"}
            </Button>
          </div>
          <div className="flex text-center">
            {hidden && (
              <>
                <SampleForms setEditorContent={setEditorContent}></SampleForms>
              </>
            )}
            {rules && (
              <>
                <div>
                  <pre className="pl-4 text-left">{MARKDOWN_RULES}</pre>
                </div>
              </>
            )}
          </div>
          <div className="p-3"></div>
        </div>
        <div className="p-1 align-middle"></div>
        <div className="flex flex-col flex-1 w-full p-2 md:w-1/2">
          <h2 className="text-2xl font-bold">Preview</h2>
          <RenderedForm
            survey={survey}
            mode={mode}
            showSubmissionData={true}
          ></RenderedForm>
        </div>
      </div>
    </>
  );
}
