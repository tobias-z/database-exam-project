import { useMemo, useRef, useState } from "react";
import "./autocompleted-query.css";
import { SERVER_PROPS_ID } from "next/dist/shared/lib/constants";

type Props = {
    query: string;
    onChange: (name: string, value: string) => void;
    isLoading: boolean;
};

type Command = {
    description: string;
    subCommands?: Commands;
};

type Commands = Record<string, Command>;

type Completion = {
    completion: string;
    description: string;
};

function createCompletions(commands: Commands) {
    return Object.entries(commands).map(([key, value]) => ({
        completion: key,
        description: value.description,
    }));
}

export default function AutocompletedQuery({ query, onChange, isLoading }: Props): JSX.Element {
    const [completions, setCompletions] = useState<Array<Completion>>([]);
    const queryInputRef = useRef<HTMLInputElement>(null);
    const commands = useMemo<Commands>(() => {
        // TODO: description, sub commands
        return {
            container: {
                description: "Used to find logs from a specific container / application.",
                subCommands: {
                    container_name: {
                        description: "Example: 'log-service'",
                    },
                },
            },
            log_level: {
                description: "Used to find logs with specific log level",
                subCommands: {
                    FATAL: {
                        description: "Filters to find fatal logs.",
                    },
                    ERROR: {
                        description: "Filters to find error logs.",
                    },
                    WARN: {
                        description: "Filters to find warn logs.",
                    },
                    INFO: {
                        description: "Filters to find info logs.",
                    },
                    DEBUG: {
                        description: "Filters to find debug logs.",
                    },
                    TRACE: {
                        description: "Filters to find trace logs.",
                    },
                },
            },
            sort: {
                description: "Used to sort the results by when they were logged.",
                subCommands: {
                    ASC: {
                        description: "Sorts the results in ascending order.",
                    },
                    DESC: {
                        description: "Sorts the results in decending order.",
                    },
                },
            },
            find: {
                description: "A regex to search for specific text in the logs",
                subCommands: {
                    message: {
                        description: "Example: 'Starting'",
                    },
                },
            },
            offset: {
                description:
                    "Used to return only the results from a specific offset, based on the current time.",
                subCommands: {
                    "10ms": {
                        description: "Milliseconds offset",
                    },
                    "10s": {
                        description: "Seconds offset",
                    },
                    "10m": {
                        description: "Minutes offset",
                    },
                    "10h": {
                        description: "Hours offset",
                    },
                    "10d": {
                        description: "Days offset",
                    },
                },
            },
            fzy_find: {
                description:
                    "Used to perform fuzzy finding in the log message. NOTICE: This has to be the first thing you do!",
                subCommands: {
                    message: {
                        description: "Example: 'world hello'",
                    },
                },
            },
            take: {
                description: "Used to limit the amount of results you want to get back.",
                subCommands: {
                    "100": {
                        description:
                            "returns the first 100 elements in your current stream of results",
                    },
                },
            },
        };
    }, []);
    const [currentCommand, setCurrentCommand] = useState<keyof typeof commands | null>(null);

    function handleChange(event: React.ChangeEvent<HTMLInputElement>): void {
        updateCompletions(event.target.value);
    }

    function updateCompletions(newQuery: string) {
        const splitQuery = newQuery.split(" ");
        const lastWord = splitQuery[splitQuery.length - 1].trim();
        if (
            (currentCommand && lastWord === "|") ||
            (currentCommand && lastWord === currentCommand) ||
            newQuery.length === 0
        ) {
            setCurrentCommand(null);
            setCompletions(createCompletions(commands));
        } else if (Object.hasOwn(commands, lastWord)) {
            let subCommands = commands[lastWord].subCommands;
            if (subCommands) {
                setCompletions(createCompletions(subCommands));
                setCurrentCommand(lastWord);
            }
        }
        // propagate it up
        onChange("query", newQuery);
    }

    function handleBlur(_event: React.FocusEvent<HTMLInputElement, Element>): void {
        // When a user clicks off the input field, stop showing suggestions
        setTimeout(() => {
            setCompletions([]);
        }, 100);
    }

    function handleFocus(_event: React.FocusEvent<HTMLInputElement, Element>): void {
        // TODO: set completion active again
        if (!currentCommand) {
            setCompletions(createCompletions(commands));
        } else {
            const subCommands = commands[currentCommand].subCommands;
            if (subCommands) {
                setCompletions(createCompletions(subCommands));
            }
        }
    }

    function handleSelectCompletion(completion: string): void {
        queryInputRef.current?.focus();
        const splitQuery = query.split(" ");
        const lastWord = splitQuery[splitQuery.length - 1];
        const replaceRegex = new RegExp(`${lastWord}$`);
        setTimeout(() => {
            updateCompletions(query.replace(replaceRegex, completion));
        }, 100);
    }

    return (
        <div style={{ display: "flex", flexDirection: "column" }}>
            <div className="inputcontainer">
                <input
                    name="query"
                    className="loading"
                    ref={queryInputRef}
                    value={query}
                    placeholder="YouBook Query"
                    onChange={handleChange}
                    onBlur={handleBlur}
                    onFocus={handleFocus}
                    autoComplete="off"
                    style={{
                        padding: `12px ${isLoading ? "35px" : "20px"}`,
                        margin: completions.length === 0 ? "8px 0" : "",
                    }}
                />
                {isLoading ? (
                    <div className="icon-container">
                        <i className="loader" />
                    </div>
                ) : null}
            </div>
            {completions.length !== 0
                ? completions.map(completion => (
                    <div
                        key={completion.completion}
                        className="completion"
                        onClick={() => handleSelectCompletion(completion.completion)}
                        style={{
                            paddingLeft: "20px",
                            // paddingBottom: "12px",
                            paddingTop: "3px",
                            lineHeight: "0px",
                            justifyContent: "center",
                            cursor: "pointer",
                            borderRight: "1px solid black",
                            borderLeft: "1px solid black",
                            borderBottom: "1px solid black",
                        }}
                    >
                        <div style={{ display: "flex", alignItems: "center", gap: "20px" }}>
                            <p>{completion.completion}</p>
                            <small>{completion.description}</small>
                        </div>
                    </div>
                ))
                : null}
        </div>
    );
}
