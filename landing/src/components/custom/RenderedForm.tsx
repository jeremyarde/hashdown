import { useState } from "react";

type SurveyEvent = {
  question_id: string;
  value: any;
};
export type RenderedFormProps = {
  survey: any; // changed from object to any
  mode: "test" | "prod";
  showSubmissionData: boolean;
};

export function RenderedForm({
  survey,
  mode,
  showSubmissionData = false,
}: RenderedFormProps) {
  const [displayTextMode, setDisplayTextMode] = useState(false);
  const [showEndScreen, setShowEndScreen] = useState(false);
  const [dummy, setDummy] = useState(true); // use to trigger rerender
  const [exampleSubmission, setExampleSubmittion] = useState(getDefaultState());

  function getDefaultState() {
    return {
      survey_id: survey.survey_id,
      answers: {},
    };
  }

  function handleEvent(surveyEvent: any) {
    // surveyEvent: { question_id, value, type?, checked? }
    setExampleSubmittion((curr: any) => {
      if (surveyEvent.type === "checkbox") {
        if (!curr.answers[surveyEvent.question_id]) {
          curr.answers[surveyEvent.question_id] = [];
        }
        if (!surveyEvent.checked) {
          curr.answers[surveyEvent.question_id] = curr.answers[
            surveyEvent.question_id
          ].filter((c: any) => c !== surveyEvent.value);
          return { ...curr };
        } else {
          curr.answers[surveyEvent.question_id] = [
            ...new Set([
              ...curr.answers[surveyEvent.question_id],
              surveyEvent.value,
            ]),
          ];
          return { ...curr };
        }
      } else {
        curr.answers[surveyEvent.question_id] = surveyEvent.value;
        return { ...curr };
      }
    });
    setDummy(dummy ? false : true);
  }

  let parsingError = undefined;
  if (!survey.blocks) {
    parsingError = survey;
  }

  const handleSubmit = async (evt: any) => {
    evt.preventDefault();
    const survey_id = survey.survey_id;
    const surveySubmission = exampleSubmission;
    if (mode === "test") {
      setExampleSubmittion(surveySubmission);
      setShowEndScreen(true);
      return;
    }
    // No getApiBaseUrl, so just skip fetch
    setShowEndScreen(true);
  };

  const handleUpdate = (evt: any) => {
    // No-op for now
  };

  return (
    <>
      <div
        className="flex justify-center items-center"
        style={{
          height: window.location.href.endsWith("/editor") ? "" : "",
        }}
      >
        {parsingError ? (
          <div style={{ whiteSpace: "pre-wrap", textAlign: "left" }}>
            <pre>
              <code className="bg-red-200">
                {JSON.stringify(parsingError, null, 2)}
              </code>
            </pre>
          </div>
        ) : (
          ""
        )}
        {showEndScreen ? (
          <EndScreen></EndScreen>
        ) : (
          <div
            className="align-middle"
            style={{
              width: "1000px",
              maxWidth: "48rem",
              minWidth: "12rem",
            }}
          >
            <form
              onKeyUp={(evt) => {
                if (evt.key === "Enter") {
                  // No toast, just log
                  console.log("Pressed Enter", exampleSubmission);
                }
              }}
              onSubmit={handleSubmit}
              onChange={handleUpdate}
              className="text-left rounded-xl border border-solid"
            >
              {survey.blocks?.map((block: any, idx: number) => {
                let blockHtml = undefined;
                switch (block.block_type) {
                  case "Title":
                    blockHtml = (
                      <h1 className="space-y-2 text-xl font-bold text-center">
                        {block.properties.title}
                      </h1>
                    );
                    break;
                  case "TextInput":
                    blockHtml = (
                      <div>
                        {TextInput(block, setExampleSubmittion, handleEvent)}
                      </div>
                    );
                    break;
                  case "Textarea":
                    blockHtml = (
                      <div>
                        {TextareaComponent(
                          block,
                          setExampleSubmittion,
                          handleEvent
                        )}
                      </div>
                    );
                    break;
                  case "Checkbox":
                    blockHtml = (
                      <div>
                        {CheckboxGroup(
                          block,
                          setExampleSubmittion,
                          handleEvent
                        )}
                      </div>
                    );
                    break;
                  case "Radio":
                    blockHtml = (
                      <div>
                        {RadioGroup(block, setExampleSubmittion, handleEvent)}
                      </div>
                    );
                    break;
                  case "Submit":
                    blockHtml = <div>{SubmitButton(block)}</div>;
                    break;
                  case "ErrorBlock":
                    blockHtml = <div>{ErrorBlock(block)}</div>;
                }
                return (
                  <div key={idx} style={{ margin: "20px", border: "line" }}>
                    {blockHtml}
                  </div>
                );
              })}
            </form>
          </div>
        )}
      </div>
      {showEndScreen && mode === "test" ? (
        <div>
          <button
            onClick={(evt) => setShowEndScreen(false)}
            className="p-2 w-2/3 bg-purple"
          >
            Go back
          </button>
        </div>
      ) : (
        <></>
      )}
      {exampleSubmission && showSubmissionData ? (
        <>
          <div className="">
            <div>
              <h3>Submission data</h3>
            </div>
            <div className="p-6 text-left border border-dotted">
              <pre>
                <code className="bg-blue-200">
                  {JSON.stringify(exampleSubmission, null, 2)}
                </code>
              </pre>
            </div>
          </div>
        </>
      ) : (
        ""
      )}
    </>
  );
}

function CheckboxGroup(block: any, setStateFn: any, handleEvent: any) {
  return (
    <>
      <label className="font-semibold">{block.properties.question}</label>
      <div className="flex flex-col space-y-2">
        {block.properties.options.map((option: any, i: number) => {
          return (
            <div className="flex items-center" key={i}>
              <input
                type="checkbox"
                id={`${block.properties.id}.${option.id}`}
                name={`${block.properties.id}.${option.id}`}
                onChange={(e) => {
                  handleEvent({
                    value: option.text,
                    question_id: block.properties.id,
                    type: "checkbox",
                    checked: e.target.checked,
                  });
                }}
              />
              <label
                className="items-center ml-2 text-sm"
                htmlFor={`${block.properties.id}.${option.id}`}
              >
                {option.text}
              </label>
            </div>
          );
        })}
      </div>
    </>
  );
}

function RadioGroup(block: any, setStateFn: any, handleEvent: any) {
  return (
    <>
      <label className="space-y-2 text-left">{block.properties.question}</label>
      <div className="flex flex-col">
        <ul className="space-y-2">
          {block.properties.options.map((option: string, i: number) => {
            return (
              <li key={i}>
                <div
                  onClick={(evt) =>
                    handleEvent({
                      value: option,
                      question_id: block.properties.id,
                    })
                  }
                  className="flex items-center space-x-2"
                >
                  <input
                    type="radio"
                    id={option}
                    name={block.properties.id}
                    value={option}
                  />
                  <label className="items-center" htmlFor={option}>
                    {option}
                  </label>
                </div>
              </li>
            );
          })}
        </ul>
      </div>
    </>
  );
}

function TextInput(block: any, setStateFn: any, handleEvent: any) {
  return (
    <>
      <label htmlFor={block.properties.id}>{block.properties.question}</label>
      <input
        type="text"
        onChange={(evt) => {
          handleEvent({
            value: evt.target.value,
            question_id: block.properties.id,
          });
        }}
        id={block.properties.id}
        name={block.properties.id}
        placeholder="Enter text"
      />
    </>
  );
}

function TextareaComponent(block: any, setStateFn: any, handleEvent: any) {
  return (
    <>
      <label htmlFor={block.properties.id}>{block.properties.question}</label>
      <textarea
        onChange={(evt) => {
          handleEvent({
            value: evt.target.value,
            question_id: block.properties.id,
          });
        }}
        id={block.properties.id}
        name={block.properties.id}
        placeholder="Enter text"
      />
    </>
  );
}

function ErrorBlock(block: any) {
  return (
    <div className="flex flex-col border rounded-s bg-yellow">
      <label className="underline">{"Problem with this line:"}</label>
      <div className="">{block.properties.text}</div>
    </div>
  );
}

function SubmitButton(block: any) {
  return (
    <>
      <div>
        <button className="outline outline-1 active:bg-green" type="submit">
          {block.properties.button}
        </button>
      </div>
    </>
  );
}

function EndScreen(block?: any) {
  return (
    <>
      <div
        className="flex justify-center w-2/3 text-center rounded-xl border border-solid outline outline-1 outline-solid"
        style={{ height: "50vh", display: "", alignItems: "center" }}
      >
        <h1>Thanks for your response!</h1>
      </div>
    </>
  );
}
