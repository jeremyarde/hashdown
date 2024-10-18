import { useState } from "react";
import { RenderedForm } from "../components/custom/RenderedForm.tsx";
import { markdown_to_form_wasm_v2 } from "../../../backend/pkg/markdownparser";

export function HeroSection() {
  const [heroContent, setHeroContent] = useState(`# Feedback

text: How did you hear about us?

radio: Can we contact you for follow up questions?
- yes
- no

submit: submit`);

  let sampleSurvey = markdown_to_form_wasm_v2(heroContent);

  return (
    <div className="flex flex-col p-6">
      <div className="p-6 pb-24">
        <h1
          className="flex justify-center pt-4 text-4xl text-center top-10"
          style={{ fontWeight: "700", color: "black" }}
        >
          The fastest way to create and share surveys.
          <br />
          Write, visualize, share.
        </h1>
        <p className="text-xl" style={{ color: "#ff5e5bff" }}>
          Hashdown is the easiest text based form maker
        </p>
      </div>
      <div className="flex flex-row p-6">
        <p
          style={{ whiteSpace: "pre-wrap" }}
          className="flex-wrap self-center flex-1 w-1/2 p-6 text-2xl"
        >
          {"A few lines of text like this"}
        </p>
        <div className="w-1/2 h-full pr-10 ">
          <ol
            style={{
              whiteSpace: "pre",
              wordWrap: "normal",
              backgroundColor: "white",
            }}
            className="flex flex-col pl-2 ml-4 bg-white border border-dashed"
          >
            {heroContent.split("\n").map((item, i) => {
              return (
                <li
                  key={i}
                  className="justify-between text-xl text-left min-h-6 "
                  style={{
                    fontSize: "1rem",
                    wordWrap: "normal",
                    wordBreak: "normal",
                    whiteSpace: "normal",
                    borderBottom: "1px dashed gray",
                  }}
                >
                  <div className="justify-between w-full h-full">{item}</div>
                </li>
              );
            })}
          </ol>
        </div>
      </div>
      <div className="flex flex-row">
        <p
          style={{ whiteSpace: "pre-wrap" }}
          className="flex-wrap self-center justify-center w-1/2 text-2xl"
        >
          {"Turns into this"}
        </p>
        <div className="w-1/2 pr-10">
          <RenderedForm
            survey={sampleSurvey}
            mode="test"
            showSubmissionData={false}
          ></RenderedForm>
        </div>
      </div>
    </div>
  );
}
