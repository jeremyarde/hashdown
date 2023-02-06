import React from 'react';

async function getData() {
    const data = await fetch('http://localhost:3000/surveys').then((response) => response.json());
    return data;
}

const data = getData();

export default async function Survey() {
    const [survey, setSurvey] = React.useState({});

    // let allsurveys = survey.forEach(element => { });


    return (
        <>

            <h3></h3>
        </>
    )
}