import React, { useState, ReactNode } from "react"
import Collapse from "@mui/material/Collapse"

interface Props {
    header: ReactNode,
    children: ReactNode
}

const ExtendableContainer: React.FC<Props> = (props: Props) => {
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
