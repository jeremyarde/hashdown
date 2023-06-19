import React from "react";

export function Navbar() {
    let navlinks = [{ title: "pricing", id: 1, link: "/pricing" }, { title: "login", id: 2, link: "/login" }];

    return (
        <div className="container">
            <nav className="navbar">
                <h1>
                    <a href="/" className="" style={{ display: "flex" }}>
                        <img src="ui-next/src/app/favicon.ico"></img>
                        <span className="">Formit</span>
                    </a>
                </h1>

                <ul className="navbar-links">
                    {navlinks.map((nav, index) => (
                        <li key={nav.id} className="navbar-items">
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