import { memo, useState } from "react";
import { MdEast, MdSouth } from "react-icons/md";
import Toggle from "./toggle";

export const QueryInformation = memo(function Information() {
    const [showAvailableCommands, setShowAvailableCommands] = useState(false);

    return (
        <>
            <Toggle
                text="Help "
                onChange={setShowAvailableCommands}
                disabledIcon={<MdEast />}
                enabledIcon={<MdSouth />}
            />
            {showAvailableCommands ? (
                <section>
                    <p>
                        Example longer query:{" "}
                        <strong>
                            container log-service | log_level ERROR | sort DESC | offset 7d
                        </strong>
                        . This query finds all errors that have occored during the last 7 days, and
                        sorts the results
                    </p>
                    <p>
                        The query language allows you to pipe the results of each of the commands
                        into eachother. What you are doing after each pipe is working on the results
                        of the previous pipe.
                    </p>
                    <h3>container</h3>
                    <p>
                        This command allows you to search for a specific container. Example usage: <strong>container hello world</strong>
                    </p>
                    <h3>log_level</h3>
                    <p>
                        This command allows you to only show logs in a specific log level. Example
                        usage: <strong>log_level ERROR</strong>
                    </p>
                    <h3>sort</h3>
                    <p>This command allows you to sort by the log date. Example usage: <strong>sort DESC</strong></p>
                    <h3>find</h3>
                    <p>
                        This command allows you to find anything inside a message. You can provide
                        it a regex and it will match it on the log message. Example usage: <strong>find hello world</strong>
                    </p>
                    <h3>offset</h3>
                    <p>
                        This command allows you to only take the messages that have been logged x
                        amount of time before now. Example usage: <strong>offset 1d</strong>
                    </p>
                    <h3>fzy_find</h3>
                    <p>
                        This command allows you to perform fuzzy finding on the log message. The
                        only caviar is that you HAVE to use it as the first thing, otherwise the
                        query will error. Example usage: <strong>fzy_find ERROR world something</strong>
                    </p>
                    <h3>take</h3>
                    <p>
                        This command allows you to limit the amount of results you are looking at.
                        Example usage: <strong>take 10</strong>
                    </p>
                </section>
            ) : null}
        </>
    );
});
