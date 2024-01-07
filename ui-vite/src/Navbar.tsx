import { FEATURES, SESSION_TOKEN_KEY, STAGE } from "./lib/constants";
import { Link, useNavigate } from "react-router-dom";
import { Toaster } from "./components/ui/toaster";
import { getSessionToken, getStage, isFeatureEnabled } from "./lib/utils";



export function Navbar() {
    // let globalState: GlobalState = useContext(GlobalStateContext);
    const navigate = useNavigate();
    const showWaitlist = isFeatureEnabled(FEATURES.WAITLIST);
    const showTabs = isFeatureEnabled(FEATURES.LOGIN);

    let tabs = !getSessionToken() ? (
        <>
            <Link className="hover:text-green p-1" to="/editor">Editor</Link>
            <Link className="hover:text-green p-1" to="/login">Login</Link>
        </>
    ) : (
        <>
            <Link className="hover:text-green p-1" to="/surveys">Surveys</Link>
            <Link className="hover:text-green p-1" to="/editor">Editor</Link>
        </>
    );

    let waitlist = showWaitlist ? (
        <div>
            <button onClick={(evt) => navigate(`/waitlist`)
            }>
                {window.location.href.endsWith('waitlist') ? '' : 'Waitlist'}
            </button>
        </div>
    ) : <></>;

    return (
        <>
            <div className="flex justify-between w-full">
                <h1>
                    <Link className="text-2xl font-bold" to="/">
                        <span>hashdown</span>
                    </Link>
                </h1>
                <div className="flex items-center">
                    {showTabs ? tabs : ''}
                    {waitlist}
                </div>
            </div >
            <Toaster />
        </>
    )
}
