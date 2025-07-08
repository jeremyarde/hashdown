import { useEffect, useState } from "react";
import { RenderedForm } from "./RenderedForm";
import { clsx } from "clsx";

import { MARKDOWN_RULES } from "../../lib/constants";

export type EditorProps = {
  mode: "test" | "prod";
  startingContent: string;
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

const tabTemplates = [
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
  setSelected: (index: number) => void;
  index: number;
}) {
  const onClick = () => {
    setContentCallback(item.template);
    setSelected(index);
  };

  return (
    <div
      style={{
        backgroundColor: active ? "rgb(19, 206, 102)" : "#FFFFFF",
        flexGrow: 1,
        padding: 4,
        margin: 4,
        fontSize: 12,
        borderRadius: 4,
        outline: "1px solid #ccc",
        cursor: "pointer",
        fontWeight: active ? "bold" : "normal",
        boxShadow: active ? "0 0 4px #13ce66" : undefined,
      }}
      onClick={onClick}
    >
      {item.tabname}
    </div>
  );
}

export function SampleForms({
  setEditorContent,
}: {
  setEditorContent: React.Dispatch<React.SetStateAction<string>>;
}) {
  const [selected, setSelected] = useState(0);

  return (
    <div style={{ display: "flex", width: "100%" }}>
      {tabTemplates.map((template: TabContent, i) => (
        <TabItem
          key={i}
          index={i}
          active={selected === i}
          setSelected={setSelected}
          item={template}
          setContentCallback={setEditorContent}
        />
      ))}
    </div>
  );
}

export function EditorPage({ mode = "test", startingContent }: EditorProps) {
  const [markdownToForm, setMarkdownToForm] = useState<
    null | ((s: string) => any)
  >(null);
  const [editorContent, setEditorContent] = useState<string>(startingContent);
  const [survey, setSurvey] = useState<any>(null);
  const [hidden, setHidden] = useState(false);
  const [rules, setRules] = useState(false);

  useEffect(() => {
    import("../../../../backend/pkg/markdownparser").then((wasm) => {
      setMarkdownToForm(() => wasm.markdown_to_form_wasm_v2);
    });
  }, []);

  useEffect(() => {
    if (markdownToForm) {
      setSurvey(markdownToForm(editorContent));
    }
  }, [editorContent, markdownToForm]);

  if (!markdownToForm || !survey) return <div>Loading WASM...</div>;

  // async function submitSurvey() {
  //   const response = await fetch(`${getApiBaseUrl()}/v1/surveys`, {
  //     method: "POST",
  //     headers: {
  //       "content-type": "application/json",
  //       session_id: getSessionToken(),
  //     },
  //     body: JSON.stringify({ plaintext: editorContent }),
  //   });
  //   handleResponse(response);
  //   const result = await response.json();

  //   if (response.status === 401) {
  //     // toast({
  //     //   title: "Submit Survey: Failed",
  //     //   description: result.message,
  //     // });
  //   }

  //   if (response.status === 200) {
  //     // toast({
  //     //   title: "Submit Survey: Success",
  //     //   description: result.message,
  //     // });
  //   }
  // }

  return (
    <div
      style={{
        display: "flex",
        flexDirection: "row",
        width: "100%",
      }}
    >
      <div
        style={{
          display: "flex",
          flexDirection: "column",
          flex: 1,
          padding: 16,
          boxSizing: "border-box",
          borderRight: "1px solid #ccc",
        }}
      >
        <textarea
          style={{
            padding: 8,
            width: "100%",
            borderRadius: 8,
            border: "1px solid #ccc",
            height: "65vh",
            boxSizing: "border-box",
            marginBottom: 8,
          }}
          placeholder="Enter form content here..."
          value={editorContent}
          onChange={(evt) => setEditorContent(evt.target.value)}
        />
        <div style={{ paddingBottom: 8 }}>
          <button
            disabled={!survey.validation[0]}
            style={{
              padding: 8,
              width: "100%",
              border: "1px solid #ccc",
              background: survey.validation[0] ? "#13ce66" : "#eee",
              opacity: survey.validation[0] ? 1 : 0.5,
              cursor: survey.validation[0] ? "pointer" : "not-allowed",
            }}
            // onClick={submit Survey}
          >
            Save Survey
          </button>
        </div>
        <div>
          <ul style={{ paddingLeft: 16 }}>
            {survey.validation[1].map((error: string) => (
              <li
                style={{ background: "#ffe066", marginBottom: 4 }}
                key={error}
              >
                {error}
              </li>
            ))}
          </ul>
        </div>
        <div
          style={{
            display: "flex",
            flexDirection: "row",
            width: "100%",
            gap: 8,
            margin: "8px 0",
          }}
        >
          <button
            style={{
              border: "1px solid #ccc",
              background: "#ffe066",
              padding: 4,
            }}
            onClick={() => setRules((r) => !r)}
          >
            {rules ? "hide rules" : "show rules"}
          </button>
          <button
            style={{
              border: "1px solid #ccc",
              background: "#ffe066",
              padding: 4,
            }}
            onClick={() => setHidden((h) => !h)}
          >
            {hidden ? "hide examples" : "show examples"}
          </button>
        </div>
        <div style={{ textAlign: "center" }}>
          {hidden && <SampleForms setEditorContent={setEditorContent} />}
          {rules && (
            <div>
              <pre style={{ paddingLeft: 16, textAlign: "left" }}>
                {MARKDOWN_RULES}
              </pre>
            </div>
          )}
        </div>
        <div style={{ padding: 12 }}></div>
      </div>
      <div
        style={{
          display: "flex",
          flexDirection: "column",
          flex: 1,
          padding: 16,
          boxSizing: "border-box",
        }}
      >
        <h2 style={{ fontSize: 24, fontWeight: "bold" }}>Preview</h2>
        <RenderedForm survey={survey} mode={mode} showSubmissionData={true} />
      </div>
    </div>
  );
}
