import { useState } from "react";

type Props = {
    onChange: (isToggled: boolean) => void;
    enabledIcon: React.ReactNode;
    disabledIcon: React.ReactNode;
    text?: string;
};

export default function Toggle(props: Props) {
    const [enabled, setEnabled] = useState(false);
    function handleClick(event: React.MouseEvent<HTMLLabelElement, MouseEvent>): void {
        setEnabled(enabled => !enabled);
        props.onChange(!enabled);
    }

    return (
        <label
            onClick={handleClick}
            style={{
                userSelect: "none",
                cursor: "pointer"
            }}
        >
            <>
                {props.text ? props.text : null}
                {enabled ? props.enabledIcon : props.disabledIcon}
            </>
        </label>
    );
}
