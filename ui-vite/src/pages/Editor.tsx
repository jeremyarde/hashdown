import { useState } from "react";
import { RenderedForm } from "../RenderedForm";

export type EditorProps = {
    editorContent: string;
    setEditorContent: React.Dispatch<React.SetStateAction<string>>;
}

export function Editor({ editorContent, setEditorContent }: EditorProps) {
    // let [editorText, setEditorText] = useState('');


    return (
        <>
            <h1 className="text-2xl font-bold mb-4">Enter Form Content</h1>
            <textarea
                className="w-full h-full p-2 rounded border border-gray-300"
                placeholder="Enter form content here..."
                value={editorContent}
                onChange={evt => setEditorContent(evt.target.value)} />
        </>
    );
}