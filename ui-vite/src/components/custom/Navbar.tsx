import { FEATURES, SESSION_TOKEN_KEY, STAGE } from "../../lib/constants";
import { Link, useNavigate } from "react-router-dom";
import { Toaster } from "../ui/toaster";
import { getSessionToken, getStage, isDev, isFeatureEnabled, logout } from "../../lib/utils";



export function Navbar() {
    // let globalState: GlobalState = useContext(GlobalStateContext);
    const navigate = useNavigate();
    const showWaitlist = isFeatureEnabled(FEATURES.WAITLIST);
    const showTabs = isFeatureEnabled(FEATURES.LOGIN) && getSessionToken();
    const testTabs = isFeatureEnabled(FEATURES.TESTTABS)

    let waitlist = showWaitlist ? (
        <div>
            <button onClick={(evt) => navigate(`/waitlist`)
            }>
                {window.location.href.endsWith('waitlist') ? '' : 'Waitlist'}
            </button>
        </div>
    ) : <></>;


    const innerRadius = 16;
    const distance = 2; // padding of outer element
    const outerRadius = innerRadius + distance;

    let middleTabs = [
        { route: "/surveys", display: "Surveys" },
        { route: "/editor", display: "Editor" },
    ]

    if (testTabs) {
        middleTabs.push({ route: "/dev", display: "dev" })
        middleTabs.push({ route: "/test", display: "test" })
    }

    return (
        <>
            <div className="flex flex-row p-4 pl-8 pr-8 justify-between align-middle items-center">
                <h1>
                    <Link className="text-2xl font-bold" to="/">
                        <span>hashdown</span>
                    </Link>
                </h1>

                {showTabs &&
                    <div className="flex flex-row border-solid items-center"
                        style={{ borderRadius: `${outerRadius}px`, padding: `${distance}px`, backgroundColor: 'whitesmoke', borderWidth: '1.5px' }}>
                        <ul className="flex">
                            {middleTabs.map((tab) => {
                                return (
                                    <li
                                        key={tab.display}
                                        className=" hover:bg-blue p-1 pl-3 pr-3" style={{ borderRadius: `${outerRadius}px` }}>
                                        <Link className="" to={tab.route}>{tab.display}</Link>
                                    </li>
                                )
                            })}
                        </ul>
                    </div>
                }
                <div className="flex flex-row items-center">
                    {!getSessionToken() && isDev() && (
                        <>
                            <div className="border-solid border flex" style={{ borderRadius: `${outerRadius}px`, padding: `${distance}px`, backgroundColor: 'black' }}>
                                <div className=" hover:bg-blue p-1 pl-3 pr-3" style={{ borderRadius: `${outerRadius}px` }}>
                                    <Link className="p-1" style={{ color: 'white' }} to="/login">Login</Link>
                                </div>
                                <div className=" hover:bg-blue p-1 pl-3 pr-3" style={{ borderRadius: `${outerRadius}px` }}>
                                    <Link className="p-1" style={{ color: 'white' }} to="/signup">Signup</Link>
                                </div>                            </div>
                        </>
                    )}
                    {getSessionToken() && (
                        <>
                            <div className="border-solid border" style={{ borderRadius: `${outerRadius}px`, padding: `${distance}px`, backgroundColor: 'black' }}>
                                <div className=" hover:bg-blue p-1 pl-3 pr-3" style={{ borderRadius: `${outerRadius}px` }}>
                                    <Link className="p-1" style={{ color: 'white' }} to="/" onClick={logout}>Logout</Link>
                                </div>
                            </div>
                        </>
                    )}
                    {showWaitlist && (
                        <div className=" hover:bg-blue p-1 pl-3 pr-3" style={{ borderRadius: `${outerRadius}px` }}>
                            <Link className="p-1" style={{ color: 'white' }} to="/waitlist">Waitlist</Link>
                        </div>
                    )}
                </div >
            </div >
            <Toaster />
        </>
    )
}
