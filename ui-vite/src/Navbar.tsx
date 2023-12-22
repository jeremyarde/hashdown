/**
 * v0 by Vercel.
 * @see https://v0.dev/t/LUoP6hiokbX
 */
// import a from "next/a"
import { BASE_URL, SESSION_TOKEN_KEY } from "./lib/constants";
import { Link } from "react-router-dom";
import { Toaster } from "./components/ui/toaster";
import { getSessionToken } from "./lib/utils";



export function Navbar() {
    // let globalState: GlobalState = useContext(GlobalStateContext);

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
        // globalState.setSessionId('');

    };

    return (
        <>
            <div className="flex items-center justify-between w-full">
                <div>
                    <Link className="text-2xl font-bold" to="/">
                        <span>Hashdown</span>
                    </Link>
                </div>
                <div className="flex items-center space-x-4">
                    {!getSessionToken() ? (
                        <>
                            <Link className="hover:animate-pulse" to="/editor">Editor</Link>
                            <Link className="hover:animate-pulse" to="/login">Login</Link>
                        </>
                    ) : (
                        <>
                            <Link className="hover:animate-pulse" to="/surveys">Surveys</Link>
                            <Link className="hover:animate-pulse" to="/editor">Editor</Link>
                            {/* <Link className="hover:animate-pulse" to='/' onClick={(e) => logout()} > Logout</Link> */}
                        </>
                    )
                    }
                </div>
            </div >
            <Toaster />
        </>
    )
}
