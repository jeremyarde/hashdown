import { useContext, useEffect, useState } from "react";
import { RenderedForm } from "../RenderedForm";
import { BASE_URL } from "@/lib/constants";
import { GlobalStateContext } from "@/main";
import { useToast } from "@/components/ui/use-toast";
import { markdown_to_form_wasm_v2 } from "../../../backend/pkg/markdownparser";

export type EditorProps = {
    editorContent: string;
    setEditorContent: React.Dispatch<React.SetStateAction<string>>;
}

export function EditorPage({ mode = "test" }) {
    let globalState = useContext(GlobalStateContext);
    const { toast } = useToast()
    const [editorContent, setEditorContent] = useState(`# User Registration Form

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

Dropdown: My question here
    - Option 1
    - Option 2
    - Option 3

submit: Send [default values]`);
    console.log('editorContent: ' + editorContent);
    const [survey, setSurvey] = useState(markdown_to_form_wasm_v2(editorContent));

    useEffect(() => {
        console.log('editor useeffect');
        const newSurvey = markdown_to_form_wasm_v2(editorContent);
        setSurvey(newSurvey);
    }, [editorContent]);
    // const [token, setToken] = useState('');

    async function submitSurvey(event) {
        const response = await fetch(`${BASE_URL}/surveys`, {
            method: "POST",
            // credentials: 'include',
            headers: {
                'content-type': 'application/json',
                'session_id': globalState.sessionId ?? '',
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
                <div className="w-1/2 p-4">
                    <h1 className="text-2xl font-bold mb-4">Enter Form Content</h1>
                    <textarea
                        className="w-full h-4/6 p-2 rounded border border-gray-300"
                        placeholder="Enter form content here..."
                        value={editorContent}
                        onChange={evt => setEditorContent(evt.target.value)} />
                    <div className="flex flex-row">
                        <button className="bg-gray-200 border p-1 w-full" onClick={submitSurvey}>Save Survey</button>
                        <button className="bg-green-200 border w-full p-1 flex-1" onClick={submitSurvey}>Publish</button>
                    </div>
                </div>
                <div className="w-1/2 p-4">
                    <h1 className="text-2xl font-bold mb-4">Preview</h1>
                    <RenderedForm survey={survey} mode={mode} ></RenderedForm>
                </div>
            </div>
        </>
    );
}