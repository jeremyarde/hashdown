import { useEffect, useState } from 'react';
import { Button } from '../components/ui/button';
import {
    Table,
    TableBody,
    TableCaption,
    TableCell,
    TableHead,
    TableHeader,
    TableRow,
} from "@/components/ui/table"
import { Link, Navigate, redirect, useNavigate } from 'react-router-dom';
import { MoreHorizontal } from "lucide-react"
import { DropdownMenu, DropdownMenuContent, DropdownMenuItem, DropdownMenuLabel, DropdownMenuSeparator, DropdownMenuTrigger } from '@/components/ui/dropdown-menu';
import { getBaseUrl, getSessionToken, handleResponse } from '@/lib/utils';
import { Survey, styleTokens } from '@/lib/constants';
import { useListSurveys } from '@/hooks/useListSurveys';
import { DataTable } from '@/components/custom/data-table';
import { columns, columns2, data2 } from "@/components/custom/columns";


export function ListSurveys() {
    const { surveys, error, isPending } = useListSurveys();
    // const [surveys, setSurveys] = useState([]);
    // const [error, setError] = useState('');
    const navigate = useNavigate();

    // useEffect(() => {
    //     getSurveys();
    // }, [])

    // async function getSurveys() {
    //     const response = await fetch(`${getBaseUrl()}/surveys`, {
    //         method: "GET",
    //         // credentials: 'include',
    //         headers: {
    //             'session_id': getSessionToken() ?? '',
    //         }
    //     });

    //     handleResponse(response);

    //     const result = await response.json();
    //     console.log('data: ', result);
    //     if (result.error) {
    //         console.log('failed to get surveys: ', result);
    //         setError(result.message ?? 'Generic error getting surveys');
    //         if (response.status === 401) {
    //             // redirect({ to: "/login", replace: true });
    //         }
    //     } else {
    //         console.log('Found surveys: ', result);
    //         setSurveys(result.surveys);
    //         setError('');
    //     }
    // }

    const viewSurvey = (surveyId: string) => {
        console.log('go to survey');
        navigate(`/surveys/${surveyId}`);
        console.log('go to survey - END');
    };


    return (
        <>
            <div className=''>
                <h1>
                    My Surveys
                </h1>
                <div className='bg-red-600'>
                    {error ? error : ''}
                </div>
            </div >
            <div className="container mx-auto py-10">
                {/* <DataTable columns={columns} data={data} /> */}
                <DataTable columns={columns2} data={data2} />
            </div>
        </>
    );
}
