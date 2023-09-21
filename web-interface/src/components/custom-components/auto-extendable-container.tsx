import React from "react"
import Collapse from "@mui/material/Collapse"
import { extendableContainerProps } from "../../interfaces/props"

const AutoExtendableContainer = (props: extendableContainerProps) => {
    const [display, setDisplay] = React.useState(false)

    const handleChange = () => {
        setDisplay((value) => !value)
    }

    return (
        <div className="container extendable">
            { props.manualSwitch && (
                <div className="header clickable" onClick={ () => handleChange }>
                    { props.header }
                </div>
            ) }
            
            {/* Only when the container is opened programmably the component can be closed or opened manually */}
            <Collapse orientation="vertical" in={ (props.manualSwitch) ? display : props.manualSwitch }>
                { props.children }
            </Collapse>
        </div>
    )
}

export default AutoExtendableContainer
