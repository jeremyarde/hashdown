import { useContext, useState } from "react";
import { RenderedForm } from "../RenderedForm";
import { BASE_URL } from "@/lib/constants";
import { redirect } from "@tanstack/react-router";
import { GlobalStateContext } from "@/main";

export type EditorProps = {
    editorContent: string;
    setEditorContent: React.Dispatch<React.SetStateAction<string>>;
}

export function Editor({ editorContent, setEditorContent }: EditorProps) {
    let globalState = useContext(GlobalStateContext);


    async function submitSurvey(event) {
        const response = await fetch(`${BASE_URL}/surveys`, {
            method: "POST",
            credentials: 'include',
            headers: {
                'content-type': 'application/json',
                'session_id': globalState?.token ?? '',
            },
            body: JSON.stringify({ plaintext: editorContent })
        });

        const result = await response.json();
        console.log('data: ', result);

        if (response.status === 401) {
            redirect({ to: "/login", replace: true });
        }
    };

    return (
        <>
            <h1 className="text-2xl font-bold mb-4">Enter Form Content</h1>
            <textarea
                className="w-full h-4/6 p-2 rounded border border-gray-300"
                placeholder="Enter form content here..."
                value={editorContent}
                onChange={evt => setEditorContent(evt.target.value)} />
            <button className="bg-gray-400 outline p-2 rounded w-full" onClick={submitSurvey}>Save Survey</button>
        </>
    );
}