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
        backgroundColor: active ? "#EDEDED" : "transparent",
        color: active ? "#0B0D11" : "#EDEDED",
        flexGrow: 1,
        padding: 8,
        margin: 4,
        fontSize: 12,
        borderRadius: 6,
        border: "1px solid #27272A",
        cursor: "pointer",
        fontWeight: active ? "bold" : "normal",
        boxShadow: active ? "0 2px 4px rgba(0,0,0,0.1)" : "none",
        transition: "all 0.2s",
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
    <div style={{ display: "flex", width: "100%", gap: 4 }}>
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

  if (!markdownToForm || !survey) return <div style={{color: "#EDEDED"}}>Loading WASM...</div>;

  return (
    <div
      style={{
        display: "flex",
        flexDirection: "row",
        width: "100%",
        background: "#0B0D11",
        color: "#EDEDED",
        gap: 24,
      }}
    >
      <div
        style={{
          display: "flex",
          flexDirection: "column",
          flex: 1,
          padding: 0,
          boxSizing: "border-box",
        }}
      >
        <textarea
          style={{
            padding: 16,
            width: "100%",
            borderRadius: 8,
            border: "1px solid #27272A",
            background: "#15171B",
            color: "#EDEDED",
            height: "65vh",
            boxSizing: "border-box",
            marginBottom: 16,
            fontFamily: "monospace",
            fontSize: "14px",
            outline: "none",
            resize: "none",
          }}
          placeholder="Enter form content here..."
          value={editorContent}
          onChange={(evt) => setEditorContent(evt.target.value)}
        />
        <div style={{ paddingBottom: 8 }}>
          <button
            disabled={!survey.validation[0]}
            style={{
              padding: "10px 20px",
              width: "100%",
              borderRadius: 6,
              border: "none",
              background: survey.validation[0] ? "#fff" : "#333",
              color: survey.validation[0] ? "#000" : "#666",
              fontWeight: 600,
              opacity: survey.validation[0] ? 1 : 0.5,
              cursor: survey.validation[0] ? "pointer" : "not-allowed",
              transition: "all 0.2s",
            }}
          >
            {survey.validation[0] ? "Save Survey" : "Invalid Syntax"}
          </button>
        </div>
        <div>
          <ul style={{ paddingLeft: 16, listStyle: "none", margin: 0 }}>
            {survey.validation[1].map((error: string) => (
              <li
                style={{ 
                    background: "rgba(239, 68, 68, 0.1)", 
                    color: "#ef4444",
                    border: "1px solid rgba(239, 68, 68, 0.2)",
                    padding: "8px 12px",
                    borderRadius: 6,
                    marginBottom: 4,
                    fontSize: "14px"
                }}
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
              border: "1px solid #27272A",
              background: "transparent",
              color: "#A1A1AA",
              padding: "8px 16px",
              borderRadius: 6,
              cursor: "pointer",
              fontSize: "14px",
            }}
            onClick={() => setRules((r) => !r)}
          >
            {rules ? "Hide Rules" : "Show Rules"}
          </button>
          <button
            style={{
              border: "1px solid #27272A",
              background: "transparent",
              color: "#A1A1AA",
              padding: "8px 16px",
              borderRadius: 6,
              cursor: "pointer",
              fontSize: "14px",
            }}
            onClick={() => setHidden((h) => !h)}
          >
            {hidden ? "Hide Examples" : "Show Examples"}
          </button>
        </div>
        <div style={{ textAlign: "center" }}>
          {hidden && <SampleForms setEditorContent={setEditorContent} />}
          {rules && (
            <div style={{
                background: "#15171B",
                border: "1px solid #27272A",
                borderRadius: 8,
                marginTop: 16,
                padding: 16,
            }}>
              <pre style={{ 
                  textAlign: "left", 
                  color: "#A1A1AA", 
                  fontSize: "13px",
                  whiteSpace: "pre-wrap",
                  margin: 0
              }}>
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
          padding: 0,
          boxSizing: "border-box",
        }}
      >
        <h2 style={{ fontSize: 24, fontWeight: "bold", marginBottom: 16 }}>Preview</h2>
        <div style={{
            background: "#fff", 
            color: "#000", 
            padding: 24, 
            borderRadius: 12,
            minHeight: "65vh"
        }}>
             {/* Force light mode for preview as forms might be light by default */}
            <RenderedForm survey={survey} mode={mode} showSubmissionData={true} />
        </div>
      </div>
    </div>
  );
}
