import React, { useState } from "react";
import Editor from "./Editor";
import Survey from "./Survey";

export default function App() {
    const [editor, setEditor] = React.useState("");
    const [survey, setSurvey] = useState(null);
    return (
        <>
            <Editor editor={editor} setEditor={setEditor} setSurvey={setSurvey} />
            <Survey survey={survey}></Survey>
        </>
    )
}