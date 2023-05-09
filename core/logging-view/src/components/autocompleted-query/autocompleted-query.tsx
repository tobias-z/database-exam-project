import { useState } from "react";
import "./autocompleted-query.css";

type Props = {
    query: string;
    onChange: React.ChangeEventHandler<HTMLInputElement>;
};

const commands = {
    container: {},
    log_level: {},
    sort: {},
    find: {},
    offset: {},
    fzy_find: {},
    take: {}
}

export default function AutocompletedQuery({ query, onChange }: Props) {
    const [completions, setCompletions] = useState<Array<string>>(["container"]);
    const [currentCommand, setCurrentCommand] = useState<keyof typeof commands | null>(null);

    function handleChange(event: React.ChangeEvent<HTMLInputElement>): void {
        // propagate it up
        onChange(event);
    }

    function handleBlur(_event: React.FocusEvent<HTMLInputElement, Element>): void {
        // When a user clicks off the input field, stop showing suggestions
        setTimeout(() => {
            setCompletions([]);
        }, 100);
    }

    return (
        <div style={{ display: "flex", flexDirection: "column" }}>
            <input
                name="query"
                autoFocus={true}
                value={query}
                placeholder="YouBook Query"
                onChange={handleChange}
                onBlur={handleBlur}
                style={{
                    padding: "12px 20px",
                    margin: completions.length === 0 ? "8px 0" : "",
                }}
            />
            {completions.length !== 0
                ? completions.map(completion => (
                    <div
                        key={completion}
                        className="completion"
                        style={{
                            paddingLeft: "20px",
                            paddingBottom: "12px",
                            paddingTop: "15px",
                            lineHeight: "0px",
                            justifyContent: "center",
                            cursor: "pointer",
                            borderRight: "1px solid black",
                            borderLeft: "1px solid black",
                            borderBottom: "1px solid black",
                        }}
                    >
                        {completion}
                    </div>
                ))
                : null}
        </div>
    );
}
