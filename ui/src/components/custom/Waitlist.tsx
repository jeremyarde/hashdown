import { RenderedForm } from './RenderedForm.tsx';
import { useGetSurvey } from '../../hooks/useGetSurvey.ts';
import { getStage } from '@/lib/utils.ts';

export function Waitlist() {
    const { survey, error, isPending } = useGetSurvey("k3itjqi4mxhq");
    return (
        <>
            {survey &&
                <RenderedForm mode={getStage()} survey={survey}></RenderedForm>}
        </>
    );
}
