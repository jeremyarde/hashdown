import React from "react";

export default function Radio({ radio_options }) {
    // export default function Radio() {
    const [radio, setRadio] = React.useState('');
    // const radio_options = ['a', 'b', 'c'];
    console.log('we are in radio');
    console.log(radio_options);
    return (
        <>
            <div>
                This
            </div>
            {radio_options.map(option => (
                <div key={option}
                    className={" rounded-xl bg-white  focus:ring-red-500"}>
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
            ))}
        </>
    );
}