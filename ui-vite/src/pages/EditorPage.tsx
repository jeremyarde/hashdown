import { useContext, useEffect, useState } from "react";
import { RenderedForm } from "../RenderedForm";
import { BASE_URL } from "@/lib/constants";
import { GlobalStateContext } from "@/main";
import { markdown_to_form_wasm } from "markdownparser";
import { useToast } from "@/components/ui/use-toast";

export type EditorProps = {
    editorContent: string;
    setEditorContent: React.Dispatch<React.SetStateAction<string>>;
}

export function EditorPage() {
    let globalState = useContext(GlobalStateContext);
    const { toast } = useToast()
    const [editorContent, setEditorContent] = useState('# A survey title here\n- q1\n  - option 1\n  - option 2\n  - option 3\n- question 2\n  - q2 option 1\n  - q2 option 2');
    const [survey, setSurvey] = useState(undefined);

    useEffect(() => {
        console.log('editor useeffect');
        const newSurvey = markdown_to_form_wasm(editorContent);
        setSurvey(newSurvey);
    }, [editorContent]);
    // const [token, setToken] = useState('');

    async function submitSurvey(event) {
        const response = await fetch(`${BASE_URL}/surveys`, {
            method: "POST",
            credentials: 'include',
            headers: {
                'content-type': 'application/json',
                'session_id': globalState.token ?? '',
            },
            body: JSON.stringify({ plaintext: editorContent })
        });

        const result = await response.json();
        console.log('data: ', result);

        if (response.status === 401) {
            // redirect({ to: "/login", replace: true });
            toast({
                title: "Submit Survey: Failed",
                description: result.message
            })
        }

        if (response.status === 200) {
            toast({
                title: "Submit Survey: Success",
                description: result.message
            })
        }
    };

    return (
        <>
            <div className="h-screen w-full flex">
                <div className="w-1/2 border-r-2 p-4">
                    {/* <Editor editorContent={formtext} setEditorContent={setFormtext}></Editor> */}
                    <h1 className="text-2xl font-bold mb-4">Enter Form Content</h1>
                    <textarea
                        className="w-full h-4/6 p-2 rounded border border-gray-300"
                        placeholder="Enter form content here..."
                        value={editorContent}
                        onChange={evt => setEditorContent(evt.target.value)} />
                    <div className="flex flex-row">
                        <button className="bg-gray-200 outline p-1 m-1 rounded w-full" onClick={submitSurvey}>Save Survey</button>
                        <button className="bg-green-200 outline p-1 m-1 rounded w-full flex-1" onClick={submitSurvey}>Publish</button>
                    </div>
                </div>
                <div className="w-1/2 p-4">
                    <h1 className="text-2xl font-bold mb-4">Preview</h1>
                    <div className="border border-gray-300 p-4 rounded">
                        <RenderedForm plaintext={editorContent} survey={survey} ></RenderedForm>
                    </div>
                </div>
            </div>
        </>
    );
}