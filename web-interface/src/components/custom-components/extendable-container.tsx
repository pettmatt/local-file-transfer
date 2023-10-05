import React, { useState, MouseEvent } from "react"
import Collapse from "@mui/material/Collapse"
import { extendableContainerProps } from "../../interfaces/props"

const ExtendableContainer: React.FC<extendableContainerProps> = (props: extendableContainerProps) => {
    const [display, setDisplay] = useState(props.showOnLoad)

    const handleToggle = (event: MouseEvent<HTMLDivElement>) => {
        // This component works as a wrapper and some times it's wrapped around clickable elements,
        // meaning there is cases when this click event shouldn't be executed.
        // For now we only check if target is a button, if so we stop this click event.
        const target = event.target as Element
        if (target.tagName === "BUTTON") return

        setDisplay((value) => !value)
    }

    return (
        <div className="container extendable">
            <div className="header clickable" onClick={ handleToggle }>
                { props.header }
            </div>
            <Collapse orientation="vertical" in={ display } /*collapsedSize={ 40 }*/>
                { props.children }
            </Collapse>
        </div>
    )
}

export default ExtendableContainer
