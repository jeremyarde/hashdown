import React, { useState } from "react";
import Editor from "./Editor";
import Login from "./Login";
import Survey from "../app/Survey";

export default function App() {
    const [editor, setEditor] = React.useState("");
    const [survey, setSurvey] = useState('');
    const [results, setResults] = useState([]);

    const buttonstyle = 'border bg-gray-400 rounded-md px-4 py-2 m-2';

    const send_request = (event, endpoint, data) => {
        event.preventDefault();
        console.log('handling submit');
        // const data = {
        //     email: email,
        //     password: password
        // };

        // let data;

        // switch (endpoint) {
        //     case "survey":
        //         data = {
        //             plaintext: event.value
        //         }
        //         break;
        //     case "login":
        //         data = {
        //             email: event.target.email.value,
        //             password: event.target.password.value
        //         };
        // }

        // const url = event.target.id == 'signup' ? 'signup' : 'login';

        fetch(`http://localhost:8080/${endpoint}`, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json'
            },
            credentials: "include",
            body: JSON.stringify(data)
        })
            .then(response => response.json())
            .then(data => {
                console.log("Results: " + JSON.stringify(data));
                // setLoggedIn(true);
                setResults(data);
            })
            .catch(error => {
                console.error(error);
            });
    }


    return (
        <>
            <div>
                <Login></Login>
            </div>
            <div>
                <button className={buttonstyle} onClick={(event) => send_request(event, "surveys/test", { plaintext: "- title\n  - question" })}>surveys/test</button>
                <button className={buttonstyle} onClick={(event) => send_request(event, "surveys", { plaintext: "- title\n  - question" })}>surveys</button>
                {/* <button className={buttonstyle} onClick={(event) => send_request(event, "surveys/{}", { plaintext: "- title\n  - question" })}>Create</button>
                <button className={buttonstyle} onClick={(event) => send_request(event, "surveys/test", { plaintext: "- title\n  - question" })}>Create</button> */}
                <h2>
                    Data:
                </h2>
                {<div>{JSON.stringify(results)}</div>}
            </div>
            {/* <Editor editor={editor} setEditor={setEditor} setSurvey={setSurvey} />
            <div className="border bg-slate-500">
                <h2 className="text-lg">Survey below</h2>
                <Survey survey={survey}></Survey>
            </div> */}
        </>
    )
}