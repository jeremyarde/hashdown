import { useState } from 'react';
import { RenderedForm } from '../components/custom/RenderedForm.tsx';
import { markdown_to_form_wasm_v2 } from '../../../backend/pkg/markdownparser';

export function HeroSection() {
    const [heroContent, setHeroContent] = useState(`# Feedback

text: How did you hear about us?

radio: Can we contact you for follow up questions?
- yes
- no

submit: submit`);

    let sampleSurvey = markdown_to_form_wasm_v2(heroContent);

    return (
        <div className=''>
            <div className='flex-col flex'>
                <div className='p-6 pb-24'>
                    <h2 className='flex top-10 text-center justify-center text-4xl pt-4' style={{ fontWeight: '700', color: 'black' }}>
                        The fastest way to create and share surveys.
                        <br />
                        Write, visualize, share.
                    </h2>
                    <h3 className='text-xl' style={{ color: 'forestgreen' }}>Hashdown is the easiest text based form maker</h3>
                </div>
                <div className='flex flex-row pt-10 pb-10 pr-10'>
                    <p
                        style={{ whiteSpace: 'pre-wrap' }}
                        className='p-6 text-2xl flex-1 w-1/2 flex-wrap self-center'
                    >
                        {'A few lines of text like this'}
                    </p>
                    <div
                        className=' w-1/2 h-full'
                    >
                        <ol style={{ whiteSpace: 'pre', wordWrap: 'normal', backgroundColor: 'white' }}
                            className='flex flex-col pl-2 ml-4 border border-dashed bg-white'>
                            {heroContent.split('\n').map((item, i) => {
                                return (
                                    <li className='text-left justify-between min-h-6 text-xl '
                                        style={{
                                            fontSize: '1rem',
                                            wordWrap: 'normal',
                                            wordBreak: 'normal',
                                            whiteSpace: 'normal',
                                            borderBottom: '1px dashed gray',
                                        }}>
                                        <div className='w-full h-full justify-between'>
                                            {item}
                                        </div>
                                    </li>
                                );
                            })}
                        </ol>
                    </div>
                </div>
                <div className='flex flex-row'>
                    <p
                        style={{ whiteSpace: 'pre-wrap' }}
                        className='p-6 text-2xl w-1/2 flex-wrap justify-center self-center'
                    >

                        {'Turns into this'}
                    </p>
                    {/* <p style={{ whiteSpace: 'pre-wrap' }} className='p-6 text-left flex-1 w-1/2 flex-wrap h-full'>
              {'explanation of the output'}
            </p> */}
                    <div className='w-1/2 h-full pr-10'>
                        <RenderedForm survey={sampleSurvey} mode="test"></RenderedForm>
                    </div>
                </div>
            </div>
        </div>
    );
}
