import { useState } from 'react';
import { EditorPage, SampleForms } from './EditorPage.tsx';
import { HeroSection } from './HeroSection.tsx';
import { exampleText } from '../main.tsx';
import { Link } from 'react-router-dom';

export function Home() {
    const [editorContent, setEditorContent] = useState(exampleText);

    return (
        <>
            <HeroSection></HeroSection>
            <div className='flex flex-row pt-8 items-center pb-16 p-5 justify-center'>
                <Link className="outline outline-1 p-6 text-2xl rounded w-1/3 bg-purple" to="/editor">Try it now</Link>
                <Link className="outline outline-1 p-6 text-2xl rounded w-1/3 bg-green" to="/waitlist">Join the waitlist</Link>
                <hr></hr>
            </div>
            <div className="p-16">
                <h4 className='text-left'>Click on one of the examples</h4>
                <SampleForms setEditorContent={setEditorContent}></SampleForms>
                <EditorPage mode={'test'} editorContent={editorContent}
                    setEditorContent={setEditorContent}
                />
            </div>
        </>);
}
