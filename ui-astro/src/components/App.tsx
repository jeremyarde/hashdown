import React, { useState } from "react";
import Editor from "./Editor";
import Login from "./Login";
import Survey from "./Survey";

export default function App() {
    const [editor, setEditor] = React.useState("");
    const [survey, setSurvey] = useState('');
    return (
        <>
            <div>
                <Login></Login>
            </div>
            {/* <Editor editor={editor} setEditor={setEditor} setSurvey={setSurvey} />
            <div className="border bg-slate-500">
                <h2 className="text-lg">Survey below</h2>
                <Survey survey={survey}></Survey>
            </div> */}
        </>
    )
}