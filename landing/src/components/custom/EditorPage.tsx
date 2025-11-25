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
        flex: "1 1 auto",
        minWidth: "120px",
        padding: "10px 16px",
        fontSize: 13,
        borderRadius: 8,
        border: `1px solid ${active ? "#EDEDED" : "#27272A"}`,
        cursor: "pointer",
        fontWeight: active ? 600 : 500,
        boxShadow: active ? "0 2px 8px rgba(237, 237, 237, 0.2)" : "none",
        transition: "all 0.2s",
        textAlign: "center",
      }}
      onClick={onClick}
      onMouseEnter={(e) => {
        if (!active) {
          e.currentTarget.style.backgroundColor = "#1a1a1f";
          e.currentTarget.style.borderColor = "#3a3a3f";
        }
      }}
      onMouseLeave={(e) => {
        if (!active) {
          e.currentTarget.style.backgroundColor = "transparent";
          e.currentTarget.style.borderColor = "#27272A";
        }
      }}
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
    <div
      style={{
        background: "#15171B",
        border: "1px solid #27272A",
        borderRadius: 12,
        padding: 16,
        boxShadow: "0 4px 6px rgba(0, 0, 0, 0.1)",
      }}
    >
      <div
        style={{
          fontSize: "14px",
          fontWeight: 600,
          color: "#EDEDED",
          marginBottom: 12,
        }}
      >
        Quick Templates
      </div>
      <div
        style={{
          display: "flex",
          width: "100%",
          gap: 8,
          flexWrap: "wrap",
        }}
      >
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

  if (!markdownToForm || !survey) {
    return (
      <div
        style={{
          display: "flex",
          alignItems: "center",
          justifyContent: "center",
          minHeight: "400px",
          color: "#EDEDED",
          fontSize: "16px",
        }}
      >
        <div style={{ textAlign: "center" }}>
          <div
            style={{
              width: "40px",
              height: "40px",
              border: "3px solid #27272A",
              borderTopColor: "#EDEDED",
              borderRadius: "50%",
              animation: "spin 1s linear infinite",
              margin: "0 auto 16px",
            }}
          />
          <div>Loading editor...</div>
        </div>
      </div>
    );
  }

  return (
    <div
      className="editor-layout"
      style={{
        display: "flex",
        flexDirection: "row",
        width: "100%",
        background: "#0B0D11",
        color: "#EDEDED",
        gap: 32,
        minHeight: "calc(100vh - 200px)",
      }}
    >
      {/* Editor Section */}
      <div
        style={{
          display: "flex",
          flexDirection: "column",
          flex: 1,
          padding: 0,
          boxSizing: "border-box",
          minWidth: 0,
        }}
      >
        <div
          style={{
            display: "flex",
            alignItems: "center",
            justifyContent: "space-between",
            marginBottom: 16,
          }}
        >
          <h2
            style={{
              fontSize: 20,
              fontWeight: 600,
              margin: 0,
              color: "#EDEDED",
            }}
          >
            Editor
          </h2>
          <div
            style={{
              display: "flex",
              gap: 8,
            }}
          >
            <button
              style={{
                border: "1px solid #27272A",
                background: rules ? "#27272A" : "transparent",
                color: rules ? "#EDEDED" : "#A1A1AA",
                padding: "8px 16px",
                borderRadius: 6,
                cursor: "pointer",
                fontSize: "13px",
                fontWeight: 500,
                transition: "all 0.2s",
              }}
              onClick={() => setRules((r) => !r)}
              onMouseEnter={(e) => {
                if (!rules) {
                  e.currentTarget.style.background = "#1a1a1f";
                  e.currentTarget.style.borderColor = "#3a3a3f";
                }
              }}
              onMouseLeave={(e) => {
                if (!rules) {
                  e.currentTarget.style.background = "transparent";
                  e.currentTarget.style.borderColor = "#27272A";
                }
              }}
            >
              {rules ? "✓ Rules" : "Show Rules"}
            </button>
            <button
              style={{
                border: "1px solid #27272A",
                background: hidden ? "#27272A" : "transparent",
                color: hidden ? "#EDEDED" : "#A1A1AA",
                padding: "8px 16px",
                borderRadius: 6,
                cursor: "pointer",
                fontSize: "13px",
                fontWeight: 500,
                transition: "all 0.2s",
              }}
              onClick={() => setHidden((h) => !h)}
              onMouseEnter={(e) => {
                if (!hidden) {
                  e.currentTarget.style.background = "#1a1a1f";
                  e.currentTarget.style.borderColor = "#3a3a3f";
                }
              }}
              onMouseLeave={(e) => {
                if (!hidden) {
                  e.currentTarget.style.background = "transparent";
                  e.currentTarget.style.borderColor = "#27272A";
                }
              }}
            >
              {hidden ? "✓ Examples" : "Examples"}
            </button>
          </div>
        </div>

        <div
          style={{
            background: "#15171B",
            border: "1px solid #27272A",
            borderRadius: 12,
            padding: 4,
            marginBottom: 16,
            boxShadow: "0 4px 6px rgba(0, 0, 0, 0.1)",
          }}
        >
          <textarea
            style={{
              padding: 20,
              width: "100%",
              borderRadius: 8,
              border: "none",
              background: "transparent",
              color: "#EDEDED",
              height: "60vh",
              minHeight: "400px",
              boxSizing: "border-box",
              fontFamily:
                '"SF Mono", "Monaco", "Inconsolata", "Roboto Mono", monospace',
              fontSize: "14px",
              lineHeight: "1.6",
              outline: "none",
              resize: "none",
            }}
            placeholder="# Your Form Title

Text: What's your name?

Textarea: Tell us about yourself

radio: Choose an option
- Option 1
- Option 2

Submit: Submit Form"
            value={editorContent}
            onChange={(evt) => setEditorContent(evt.target.value)}
          />
        </div>

        {hidden && (
          <div style={{ marginBottom: 16 }}>
            <SampleForms setEditorContent={setEditorContent} />
          </div>
        )}

        {rules && (
          <div
            style={{
              background: "#15171B",
              border: "1px solid #27272A",
              borderRadius: 12,
              marginBottom: 16,
              padding: 20,
              boxShadow: "0 4px 6px rgba(0, 0, 0, 0.1)",
            }}
          >
            <h3
              style={{
                fontSize: "16px",
                fontWeight: 600,
                margin: "0 0 12px 0",
                color: "#EDEDED",
              }}
            >
              Markdown Syntax Guide
            </h3>
            <pre
              style={{
                textAlign: "left",
                color: "#A1A1AA",
                fontSize: "13px",
                lineHeight: "1.6",
                whiteSpace: "pre-wrap",
                margin: 0,
                fontFamily:
                  '"SF Mono", "Monaco", "Inconsolata", "Roboto Mono", monospace',
              }}
            >
              {MARKDOWN_RULES}
            </pre>
          </div>
        )}

        <div style={{ marginBottom: 16 }}>
          <button
            disabled={!survey.validation[0]}
            style={{
              padding: "14px 24px",
              width: "100%",
              borderRadius: 8,
              border: "none",
              background: survey.validation[0]
                ? "linear-gradient(135deg, #fff 0%, #f0f0f0 100%)"
                : "#1a1a1f",
              color: survey.validation[0] ? "#000" : "#666",
              fontWeight: 600,
              fontSize: "15px",
              cursor: survey.validation[0] ? "pointer" : "not-allowed",
              transition: "all 0.2s",
              boxShadow: survey.validation[0]
                ? "0 4px 12px rgba(255, 255, 255, 0.1)"
                : "none",
            }}
            onMouseEnter={(e) => {
              if (survey.validation[0]) {
                e.currentTarget.style.transform = "translateY(-1px)";
                e.currentTarget.style.boxShadow =
                  "0 6px 16px rgba(255, 255, 255, 0.15)";
              }
            }}
            onMouseLeave={(e) => {
              if (survey.validation[0]) {
                e.currentTarget.style.transform = "translateY(0)";
                e.currentTarget.style.boxShadow =
                  "0 4px 12px rgba(255, 255, 255, 0.1)";
              }
            }}
          >
            {survey.validation[0] ? "✓ Save Form" : "✗ Invalid Syntax"}
          </button>
        </div>

        {survey.validation[1].length > 0 && (
          <div
            style={{
              background: "rgba(239, 68, 68, 0.1)",
              border: "1px solid rgba(239, 68, 68, 0.3)",
              borderRadius: 8,
              padding: 16,
              marginBottom: 16,
            }}
          >
            <div
              style={{
                fontSize: "14px",
                fontWeight: 600,
                color: "#ef4444",
                marginBottom: 8,
              }}
            >
              Errors ({survey.validation[1].length})
            </div>
            <ul
              style={{
                paddingLeft: 20,
                listStyle: "none",
                margin: 0,
              }}
            >
              {survey.validation[1].map((error: string, idx: number) => (
                <li
                  style={{
                    color: "#fca5a5",
                    padding: "6px 0",
                    fontSize: "13px",
                    lineHeight: "1.5",
                  }}
                  key={idx}
                >
                  • {error}
                </li>
              ))}
            </ul>
          </div>
        )}
      </div>

      {/* Preview Section */}
      <div
        style={{
          display: "flex",
          flexDirection: "column",
          flex: 1,
          padding: 0,
          boxSizing: "border-box",
          minWidth: 0,
        }}
      >
        <div
          style={{
            display: "flex",
            alignItems: "center",
            justifyContent: "space-between",
            marginBottom: 16,
          }}
        >
          <h2
            style={{
              fontSize: 20,
              fontWeight: 600,
              margin: 0,
              color: "#EDEDED",
            }}
          >
            Preview
          </h2>
          {survey.validation[0] && (
            <div
              style={{
                background: "rgba(34, 197, 94, 0.1)",
                color: "#22c55e",
                padding: "4px 12px",
                borderRadius: 6,
                fontSize: "12px",
                fontWeight: 500,
                border: "1px solid rgba(34, 197, 94, 0.2)",
              }}
            >
              Valid
            </div>
          )}
        </div>
        <div
          style={{
            background: "#fff",
            color: "#000",
            padding: 32,
            borderRadius: 12,
            minHeight: "60vh",
            boxShadow: "0 8px 24px rgba(0, 0, 0, 0.3)",
            border: "1px solid #27272A",
          }}
        >
          <RenderedForm survey={survey} mode={mode} showSubmissionData={true} />
        </div>
      </div>

      <style>{`
        @keyframes spin {
          to { transform: rotate(360deg); }
        }
      `}</style>
    </div>
  );
}
