import React, { useState } from "react"
import Collapse from "@mui/material/Collapse"
import { extendableContainerProps } from "../../interfaces/props"

const ExtendableContainer: React.FC<extendableContainerProps> = (props: extendableContainerProps) => {
    const [display, setDisplay] = useState(false)

    const handleToggle = () => {
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
