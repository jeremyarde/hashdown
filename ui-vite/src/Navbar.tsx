/**
 * v0 by Vercel.
 * @see https://v0.dev/t/LUoP6hiokbX
 */
// import a from "next/a"
import {  SESSION_TOKEN_KEY } from "./lib/constants";
import { Link, useNavigate } from "react-router-dom";
import { Toaster } from "./components/ui/toaster";
import { getSessionToken } from "./lib/utils";



export function Navbar() {
    // let globalState: GlobalState = useContext(GlobalStateContext);
    const navigate = useNavigate();
    const showWaitlist = true;
    const showTabs = true;

    async function logout() {
        console.log('logging out');
        // const response = await fetch(`${BASE_URL}/auth/logout`, {
        //     method: "POST",
        //     headers: {
        //         "Content-Type": "application/json",
        //     },
        //     // credentials: 'include',
        //     // body: payload,
        // });
        window.sessionStorage.removeItem(SESSION_TOKEN_KEY);
    };

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
                <div>
                    <Link className="text-2xl font-bold" to="/">
                        <span>Hashdown</span>
                    </Link>
                </div>
                <div className="flex items-center space-x-4">
                    {showTabs ? tabs : ''}
                    {waitlist}
                </div>
            </div >
            <Toaster />
        </>
    )
}
