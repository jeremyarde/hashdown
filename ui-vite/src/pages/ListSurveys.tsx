import { useContext, useEffect, useState } from 'react';
import { Button } from '../components/ui/button';
import { BASE_URL } from '@/lib/constants';
import { GlobalState, GlobalStateContext } from '@/main';
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


type Survey = {
    id: string;
    created_at: string,
    survey_id: string,
}

export function ListSurveys() {
    const [surveys, setSurveys] = useState([]);
    const [error, setError] = useState('');
    let globalState: GlobalState = useContext(GlobalStateContext);
    const navigate = useNavigate();

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
                    <Table className=''>
                        <TableCaption>Click on a survey to view details</TableCaption>
                        <TableHeader>
                            <TableRow>
                                <TableHead className="w-[100px]">ID</TableHead>
                                <TableHead className="">Title</TableHead>
                                <TableHead># Questions</TableHead>
                                <TableHead>Version</TableHead>
                                <TableHead className="text-right">Created at</TableHead>
                                <TableHead className="text-right">
                                    Actions
                                </TableHead>
                            </TableRow>
                        </TableHeader>
                        <TableBody className=''>
                            {surveys.map(survey => {
                                return (
                                    <TableRow className='outline outline-1 outline-gray-300 hover:bg-blue-100'>
                                        <TableCell className="font-medium">{survey.survey_id}</TableCell>
                                        <TableCell className="font-medium">{survey.title}</TableCell>
                                        <TableCell>{survey.questions?.length ?? 0}</TableCell>
                                        <TableCell className="">{survey.parse_version}</TableCell>
                                        <TableCell className="text-right">{survey.created_at}</TableCell>
                                        <TableCell className="text-right">
                                            <DropdownMenu>
                                                <DropdownMenuTrigger asChild>
                                                    <Button variant="ghost" className="h-8 w-8 p-0 shadow-lg hover:shadow-slate-500">
                                                        <span className="sr-only">Open menu</span>
                                                        <MoreHorizontal className="h-4 w-4" />
                                                    </Button>
                                                </DropdownMenuTrigger>
                                                <DropdownMenuContent align="end" className=" bg-white">
                                                    {/* <DropdownMenuLabel>Actions</DropdownMenuLabel> */}
                                                    <DropdownMenuItem
                                                        onClick={() => { console.log('go to survey'); return navigate(`/surveys/${survey.survey_id}`) }}
                                                        className='hover:bg-blue-900'
                                                    >
                                                        View Survey
                                                    </DropdownMenuItem>
                                                    {/* <DropdownMenuSeparator /> */}
                                                    {/* <DropdownMenuItem>View payment details</DropdownMenuItem> */}
                                                </DropdownMenuContent>
                                            </DropdownMenu>

                                        </TableCell>
                                    </TableRow>
                                )
                            })}
                        </TableBody>
                    </Table>
                </div>
                <div className='bg-red-600'>
                    {error ? error : ''}
                </div>
            </div >
        </>
    );
}
