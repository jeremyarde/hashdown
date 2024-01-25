import { useNavigate } from 'react-router-dom';
import { useListSurveys } from '@/hooks/useListSurveys';
import { DataTable } from '@/components/custom/data-table';
import { surveyColumns } from "@/components/custom/columns";


export function ListSurveys() {
    const { surveys, error, isPending } = useListSurveys();
    // const navigate = useNavigate();

    // const viewSurvey = (surveyId: string) => {
    //     console.log('go to survey');
    //     navigate(`/surveys/${surveyId}`);
    //     console.log('go to survey - END');
    // };
    console.log(`jere/ pending: ${isPending}, error: ${error}, condition: ${(!error || !isPending)}`)
    return (
        <>
            <div className=''>
                <div className='bg-red-600'>
                    {error ? error : ''}
                </div>
            </div >
            {!(error || isPending) && <div className="container mx-auto py-10">
                {/* <DataTable columns={columns} data={data} /> */}
                {/* <DataTable columns={columns2} data={data2} /> */}
                <DataTable columns={surveyColumns} data={surveys?.surveys ?? []} />
            </div>}
        </>
    );
}
