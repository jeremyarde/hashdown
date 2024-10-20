import { useState } from "react";
import { EditorPage, SampleForms } from "./EditorPage.tsx";
import { HeroSection } from "./HeroSection.tsx";
import { exampleText } from "../main.tsx";
import { Link } from "react-router-dom";

export function Home() {
  const [editorContent, setEditorContent] = useState(exampleText);

  return (
    <>
      <HeroSection />
      {/* <div className="flex flex-row p-5 pt-8 pb-16 justify-evenly"> */}
      <div className="flex flex-col justify-between p-5">
        <div className="flex-1"></div>
        <Link
          className="flex-1 p-2 text-2xl rounded outline outline-1 button"
          to="/testEditor"
        >
          Try it now
        </Link>
        <div className="flex-1"></div>
        <Link
          className="flex-1 p-2 text-2xl rounded outline outline-1 button"
          to="/waitlist"
        >
          Join the waitlist
        </Link>
        <div className="flex-1"></div>
        {/* <hr></hr> */}
      </div>
    </>
  );
}
