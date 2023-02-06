import React from 'react';

export default function Editor() {
    const [editor, setEditor] = React.useState('');
    return (
        <>
            <div className={"p-4 rounded-xl bg-white dark:bg-gray-800 focus:ring-red-500"}>
                <form action="">
                    <label htmlFor="editor-field" className='sr-only'>
                        Create your survey
                    </label>
                    <textarea
                        className={'w-full text-sm text-gray-800 bg-white border-0 resize-y rounded-xl dark:bg-gray-800 dark:text-white dark:placeholder-gray-400'}
                        name="testname" id="editor-field" rows="10" value={editor} onChange={event => {
                            setEditor(event.target.value);
                        }}
                    ></textarea>
                    <button className={'hover:bg-violet-600 w-full text-blue-500 bg-blue-200 rounded p-2'} onClick={event => {
                        // postQuestions();
                        console.log('posting the questions');
                    }}>
                        Publish
                    </button>
                </form>

            </div>
        </>
    )
}

