"use client";

import AutocompletedQuery from "@/components/autocompleted-query/autocompleted-query";
import Toggle from "@/components/toggle";
import useLogServiceProxy from "@/utils/use-log-service-proxy";
import { FormEvent, useState } from "react";
import { MdSouth, MdEast } from "react-icons/md";
import useLocalStorage from "use-local-storage";

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
    // We use local storage for ease of use. This is a really bad idea in production, since we are storing the password in plain text.
    const [fields, setFields] = useLocalStorage("youbook-logs-input-fields", {
        query: "",
        username: "",
        password: "",
    });
    const [showCredentialInput, setShowCredentialInput] = useState(false);
    const {
        data: logs,
        isLoading,
        error,
        refetch: refetchLogQuery,
    } = useLogServiceProxy<Log[]>("search", `/search?q=${fields.query}`, transformLogs);

    function handleExecuteQuery(event: FormEvent<HTMLFormElement>): void {
        event.preventDefault();
        refetchLogQuery();
    }

    function handleFieldChange(name: string, value: string): void {
        setFields({
            ...fields,
            [name]: value,
        });
    }

    function handleChange(event: React.ChangeEvent<HTMLInputElement>): void {
        handleFieldChange(event.target.name, event.target.value);
    }

    return (
        <main style={{ display: "flex", flexDirection: "column" }}>
            <section
                style={{ height: "95vh", width: "95vw", alignSelf: "center", padding: "10px" }}
            >
                <form
                    onSubmit={handleExecuteQuery}
                    style={{ display: "flex", flexDirection: "column", gap: "10px" }}
                >
                    <div style={{ display: "flex", justifyContent: "space-between" }}>
                        <h1>YouBook Logs</h1>
                        <div
                            style={{
                                display: "flex",
                                justifyContent: "center",
                                alignItems: "center",
                            }}
                        >
                            <button
                                style={{
                                    padding: "20px",
                                    backgroundColor: "#00aae3",
                                    color: "white",
                                    height: "10%",
                                    lineHeight: "0px",
                                    border: "none",
                                    cursor: "pointer",
                                }}
                                type="submit"
                            >
                                <strong>Run Query</strong>
                            </button>
                        </div>
                    </div>
                    {error ? (
                        <div
                            style={{
                                borderRadius: "5px",
                                border: "1px solid black",
                                paddingLeft: "10px",
                                backgroundColor: "#eb4034",
                            }}
                        >
                            <p style={{ color: "white" }}>{error.message.message}</p>
                        </div>
                    ) : null}
                    <AutocompletedQuery query={fields.query} onChange={handleFieldChange} />
                    <div style={{ display: "flex", flexDirection: "column" }}>
                        <Toggle
                            text="Set Credentials "
                            onChange={setShowCredentialInput}
                            disabledIcon={<MdEast />}
                            enabledIcon={<MdSouth />}
                        />
                        {showCredentialInput ? (
                            <>
                                <input
                                    name="username"
                                    value={fields.username}
                                    placeholder="Username"
                                    onChange={handleChange}
                                    style={{
                                        padding: "10px 20px",
                                        margin: "5px 0",
                                    }}
                                />
                                <input
                                    name="password"
                                    type="password"
                                    value={fields.password}
                                    placeholder="Password"
                                    onChange={handleChange}
                                    style={{
                                        padding: "10px 20px",
                                        margin: "5px 0",
                                    }}
                                />
                            </>
                        ) : null}
                    </div>
                </form>
            </section>
        </main>
    );
}
