const inputExample = {
    title: 'my string',
    date: '2000-11-01',
    price: 0.11,
    quantity: 5,
}

export function Crud(inputType) {
    inputType = inputExample;

    return (
        <>
            <div>{'Hello from /dev'}</div>
            <div className="flex justify-center">
                <pre className="text-left">
                    {JSON.stringify(inputType, null, 2)}
                </pre>
            </div>
            <div style={{
                maxWidth: '1020px',
                display: 'flex',
                flexDirection: 'column'
            }}>
                <div className="flex flex-col self-center p-4">
                    {Object.entries(inputType).map(([key, value]) => {
                        return (
                            <div className="flex justify-between max-w-screen-sm border border-solid">
                                <label className="pr-4">{key}</label>
                                <input value={value}></input>
                            </div>
                        )
                    })}
                    <button className="bg-green">Save (not ready)</button>
                </div>
            </div>
        </>
    );
}