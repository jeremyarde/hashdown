import { FEATURES, SESSION_TOKEN_KEY, STAGE } from "../../lib/constants";
import { Link, useNavigate } from "react-router-dom";
import { Toaster } from "../ui/toaster";
import { getSessionToken, getStage, isDev, isFeatureEnabled } from "../../lib/utils";



export function Navbar() {
    // let globalState: GlobalState = useContext(GlobalStateContext);
    const navigate = useNavigate();
    const showWaitlist = isFeatureEnabled(FEATURES.WAITLIST);
    const showTabs = isFeatureEnabled(FEATURES.LOGIN);

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

    const middleTabs = [
        { route: "/surveys", display: "Surveys" },
        { route: "/editor", display: "Editor" },
        { route: "/dev", display: "dev" },
        { route: "/test", display: "test" },
    ]

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
                                    <li className=" hover:bg-blue p-1 pl-3 pr-3" style={{ borderRadius: `${outerRadius}px` }}>
                                        <Link className="" to={tab.route}>{tab.display}</Link>
                                    </li>
                                )
                            })}
                        </ul>
                    </div>
                }
                <div className="flex flex-row border-solid border items-center"
                    style={{ borderRadius: `${outerRadius}px`, padding: `${distance}px`, backgroundColor: 'black' }}>
                    {!getSessionToken() && isDev() && (
                        <>
                            <div className="">
                                <div className=" hover:bg-blue p-1 pl-3 pr-3" style={{ borderRadius: `${outerRadius}px` }}>
                                    <Link className="p-1" style={{ color: 'white' }} to="/login">Login</Link>
                                </div>
                            </div>
                        </>
                    )}
                    {showWaitlist && (
                        <div className="">
                            <div className=" hover:bg-blue p-1 pl-3 pr-3" style={{ borderRadius: `${outerRadius}px` }}>
                                <Link className="p-1" style={{ color: 'white' }} to="/waitlist">Waitlist</Link>
                            </div>
                        </div>

                    )}
                </div >
            </div >
            <Toaster />
        </>
    )
}
