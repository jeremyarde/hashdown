import React, { useState } from "react";
import Editor from "./Editor";
import Survey from "./Survey";

export default function App() {
    const [editor, setEditor] = React.useState("");
    const [survey, setSurvey] = useState(null);
    return (
        <>
            <Editor editor={editor} setEditor={setEditor} setSurvey={setSurvey} />
            <div className="border bg-slate-500">
                <h2 className="text-lg">Survey below</h2>
                <Survey survey={survey}></Survey>
            </div>
        </>
    )
}