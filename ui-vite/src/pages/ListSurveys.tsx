import { useContext, useEffect, useState } from 'react';
import { Button } from '../components/ui/button';
import { BASE_URL } from '@/lib/constants';
import { GlobalState, GlobalStateContext } from '@/main';


type Survey = {
    id: string;
    created_at: string,
    survey_id: string,
}

export function ListSurveys() {
    const [surveys, setSurveys] = useState([]);
    const [error, setError] = useState('');
    let globalState: GlobalState = useContext(GlobalStateContext);

    useEffect(() => {
        getSurveys();
    }, [])

    async function getSurveys() {
        const response = await fetch(`${BASE_URL}/surveys`, {
            method: "GET",
            credentials: 'include',
            headers: {
                'session_id': globalState.token ?? '',
            }
        });

        const result = await response.json();
        console.log('data: ', result);
        if (result.error) {
            console.log('failed to get surveys: ', result);
            setError(result.message ?? 'Generic error getting surveys');
            if (response.status === 401) {
                // redirect({ to: "/login", replace: true });
            }
        } else {
            console.log('Found surveys: ', result);
            setSurveys(result.surveys);
            setError('');
        }
    }

    return (
        <>
            <div className=''>
                <h1>
                    My Surveys
                </h1>
                <div>
                    <ul>
                        {surveys.map(survey => {
                            return (
                                <a href={`/surveys/${survey.survey_id}`} >
                                    <li className='flex flex-row w-full justify-between'>

                                        <div className='text-left'>
                                            Survey ID: {survey.id}
                                        </div>
                                        <div className=''>
                                            Created at: {survey.created_at}
                                        </div>
                                    </li >
                                </a>
                            )
                        })}
                    </ul >

                </div>

                <div className='bg-red-600'>
                    {error ? error : ''}
                </div>
            </div >
        </>
    );
}
