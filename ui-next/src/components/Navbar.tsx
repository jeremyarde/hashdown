import React from "react";

export function Navbar() {
    let navlinks = [{ title: "pricing", id: 1, link: "/pricing" }, { title: "login", id: 2, link: "/login" }];

    return (
        <div className="container debug">
            <nav className="navbar debug">
                <a href="/" className="debug" style={{ display: "flex" }}>
                    <img src="ui-next/src/app/favicon.ico"></img>
                    <span className="">Formit</span>
                </a>

                <ul className="navbar-links">
                    {navlinks.map((nav, index) => (
                        <li key={nav.id} className="">
                            <a href={nav.link} className="">{nav.title}</a>
                            <a></a>
                        </li>
                    ))}
                </ul>
            </nav>
            <div className="debug">Testing</div>
        </div >
    )
}