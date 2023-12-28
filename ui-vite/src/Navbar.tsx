import { SESSION_TOKEN_KEY } from "./lib/constants";
import { Link, useNavigate } from "react-router-dom";
import { Toaster } from "./components/ui/toaster";
import { getSessionToken } from "./lib/utils";



export function Navbar() {
    // let globalState: GlobalState = useContext(GlobalStateContext);
    const navigate = useNavigate();
    const showWaitlist = true;
    const showTabs = true;

    let tabs = !getSessionToken() ? (
        <>
            <Link className="hover:animate-pulse" to="/editor">Editor</Link>
            <Link className="hover:animate-pulse" to="/login">Login</Link>
        </>
    ) : (
        <>
            <Link className="hover:animate-pulse" to="/surveys">Surveys</Link>
            <Link className="hover:animate-pulse" to="/editor">Editor</Link>
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
            <div className="flex items-center justify-between w-full">
                <h1>
                    <Link className="text-2xl font-bold" to="/">
                        <span>hashdown</span>
                    </Link>
                </h1>
                <div className="flex items-center space-x-4">
                    {showTabs ? tabs : ''}
                    {waitlist}
                </div>
            </div >
            <Toaster />
        </>
    )
}
