import React from 'react';

export function createTable(cols = [], dataKeys = [], data = []) {
    return (
        <div className="space-y-6">
            <form className="relative">
                <input
                    className="flex h-10 w-full rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background file:border-0 file:bg-transparent file:text-sm file:font-medium placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50 pl-8"
                    placeholder="Search respondents..."
                    type="search" />
            </form>
            <div className="w-full border">
                <div className='grid justify-start items-center w-full' style={{
                    gridTemplateColumns: `repeat(${cols.length}, 2fr)`
                }}>
                    {cols.map(colName => {
                        return (
                            <div className='w-full h-full border bg-yellow'>{colName}</div>
                        );
                    })}
                    {data.map(dataItem => {
                        return (
                            <>
                                {dataKeys.map((dataKey) => {
                                    let nested = dataKey.split('.');
                                    let value = dataItem;
                                    nested.forEach(key => {
                                        value = value[key];
                                    });
                                    return (
                                        <div className='h-full w-full border p-1 hover:bg-green'>{value ?? '-'}</div>
                                    );
                                })}
                            </>
                        );
                    })}
                </div>
            </div>
        </div>
    );
}
