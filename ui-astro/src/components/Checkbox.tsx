import React, { useEffect } from "react";

// const allOptions: { [key: string]: boolean } = {};
const allOptions = { 'ase': true, 'fdsa': false };
// export default function Checkbox(checkbox_options: Array<string>) {
// export default function Checkbox({ options, setOptions }) {
export default function Checkbox() {
    const [
        options,
        setOptions
    ] = React.useState(allOptions);

    console.log('we are in checkbox');
    console.log(`data: ${JSON.stringify(options)}`);

    // const [checkboxOptions, setCheckboxOptions] = React.useState(options);

    // console.log(`data: ${JSON.stringify(options)}`);
    const chechboxList = Object.keys(options);
    // console.log(`checkboxlist: ${JSON.stringify(chechboxList)}`);

    return (
        <>
            <form>
                <fieldset>
                    <legend>
                        Select toppings:
                    </legend>

                    {/*
            Iterate over those toppings, and
            create a checkbox for each one:
          */}
                    {chechboxList.map(option => (
                        <div key={option}>
                            <input
                                type="checkbox"
                                id={option}
                                value={option}
                                checked={options[option] === true}
                                onChange={event => {
                                    setOptions({
                                        ...options,
                                        [option]: event.target.checked,
                                    })
                                }}
                            />
                            <label htmlFor={option}>
                                {option}
                            </label>
                        </div>
                    ))}
                </fieldset>
            </form>
            <p>
                <strong>Opt in:</strong> {JSON.stringify(options, null, 2)}
            </p>
        </>
    );
}