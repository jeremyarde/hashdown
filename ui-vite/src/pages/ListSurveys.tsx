import { useState } from 'react';
import { Button } from '../components/ui/button';
import { base_url } from '../App';

export function ListSurveys() {
    const [surveys, setSurveys] = useState(undefined);
    const [error, setError] = useState('');

    async function getSurveys() {
        const response = await fetch(`${base_url}/surveys`, {
            method: "GET",
            credentials: 'same-origin'
        });

        const result = await response.json();
        console.log('data: ', result);
        if (result.error) {
            console.log('failed to get surveys: ', result);
            setError(result.message ?? 'Generic error getting surveys');
        } else {
            setSurveys(result);
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
                    {[surveys]}
                </div>
                <div className='bg-red-600'>
                    Errors?
                    <div>
                        {error ? error : 'No errors'}
                    </div>
                </div>
            </div>
        </>
    );
}
