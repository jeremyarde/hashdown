import React from "react";

// export default function Radio({ radio_options }) {
export default function Radio() {
    const [radio, setRadio] = React.useState('');
    let radio_options = ['a', 'b', 'c'];
    console.log('we are in radio');
    console.log(radio_options);
    return (
        <>
            {radio_options.map(option => {
                <div key={option}
                    className={"p-4 rounded-xl bg-white dark:bg-gray-800 focus:ring-red-500"}>
                    <input
                        type="radio"
                        name="current-language"
                        id={option}
                        value={option}
                        checked={option === radio}
                        onChange={event => {
                            setRadio(event.target.value);
                        }}
                    />
                    <label htmlFor={option}>
                        {option}
                    </label>
                </div>
            })}
        </>
    );
}