import { RenderedForm } from './RenderedForm.tsx';
import { useGetSurvey } from '../../hooks/useGetSurvey.ts';

export function Waitlist() {
    const { survey, error, isPending } = useGetSurvey("k3itjqi4mxhq");
    return (
        <>
            {survey &&
                <RenderedForm survey={survey}></RenderedForm>}
        </>
    );
}
