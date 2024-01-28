import { useState } from 'react';
import { EditorPage, SampleForms } from './EditorPage.tsx';
import { HeroSection } from './HeroSection.tsx';
import { exampleText } from '../main.tsx';

export function Home() {
    const [editorContent, setEditorContent] = useState(exampleText);

    return (
        <>
            <HeroSection></HeroSection>
            <div className='flex flex-col pt-8 items-center pb-16'>
                <a href='/waitlist' className='outline outline-1 p-6 w-2/3 rounded'>Join the waitlist</a>
                <h4
                    style={{ fontSize: '4rem' }}
                    className='p-6 w-2/3 rounded pt-10'>
                    Try it below
                </h4>
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
