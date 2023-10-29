import { useContext, useState } from 'react';
import { Button } from '../components/ui/button';
import { BASE_URL } from '@/lib/constants';
import { GlobalStateContext } from '@/App';
import { Link } from '@tanstack/react-router';


type Survey = {
    id: string;
    created_at: string,
    survey_id: string,
}

export function ListSurveys() {
    const [surveys, setSurveys] = useState([]);
    const [error, setError] = useState('');
    let globalState = useContext(GlobalStateContext);


    async function getSurveys() {
        const response = await fetch(`${BASE_URL}/surveys`, {
            method: "GET",
            credentials: 'include',
            headers: {
                'session_id': globalState?.token ?? '',
            }
        });

        const result = await response.json();
        console.log('data: ', result);
        if (result.error) {
            console.log('failed to get surveys: ', result);
            setError(result.message ?? 'Generic error getting surveys');
        } else {
            console.log('Found surveys: ', result);
            setSurveys(result.surveys);
            setError('');
        }
    }

    return (
        <>
            <div className='bg-green-300'>
                <Button onClick={(evt) => {
                    console.log('clicked button');
                    getSurveys();
                }}>My Surveys</Button>
                <div>
                    Surveys
                    <ul>
                        {surveys.map(survey => {
                            return (
                                <li className='flex flex-row w-full justify-between'>
                                    <Link to={`/surveys/${survey.survey_id}`}></Link>
                                    <div className='text-left'>
                                        Survey ID: {survey.id}
                                    </div>
                                    <div className=''>
                                        Created at: {survey.created_at}
                                    </div>
                                </li>
                            )
                        })}
                    </ul>
                </div>

                <div className='bg-red-600'>
                    {error ? error : ''}
                </div>
            </div>
        </>
    );
}
