"use client";

import useLogServiceProxy from "@/utils/use-log-service-proxy";
import { FormEvent, useState } from "react";

type Log = {
    container_id: string;
    container_name: string;
    message: string;
    date: string;
    level: number;
};

function transformLogs(logs: Log[]): Log[] {
    return logs.map(log => {
        return {
            ...log,
            date: new Intl.DateTimeFormat("en-US").format(log.date as unknown as number),
        };
    });
}

export default function Home() {
    const [query, setQuery] = useState("");
    const { refetch: refetchLogQuery } = useLogServiceProxy<Log[]>(
        "search",
        `/search?q=${query}`,
        transformLogs
    );

    function handleExecuteQuery(event: FormEvent<HTMLFormElement>): void {
        event.preventDefault();
        refetchLogQuery();
    }

    return (
        <main>
            <form onSubmit={handleExecuteQuery}>
                <input value={query} onChange={e => setQuery(e.target.value)} />
                <button type="submit">Submit</button>
            </form>
        </main>
    );
}
