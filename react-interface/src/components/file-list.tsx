import { useState } from "react"
import { styled, Button, SvgIcon } from "@mui/joy"
import { List, ListItem, ListItemText} from "@mui/material"

interface Props {
    devices: Array<object>,
    files: Array<object>
}

const FileList = (props: Props) => {
    const [devices, setDevices] = useState(props.devices)
    const [files, setFiles] = useState(props.files)

    const VisuallyHiddenInput = styled('input')`
        clip: rect(0 0 0 0);
        clip-path: inset(50%);
        height: 1px;
        overflow: hidden;
        position: absolute;
        bottom: 0;
        left: 0;
        white-space: nowrap;
        width: 1px;
    `

    return (
        <>
        { devices.length > 0 && (
            <div className="section-container">
                { files.length > 0 ? (
                    <div className="list-container">
                        <CustomList array={ files } />
                    </div>
                )
                :
                (
                    <div className="notification-container no-files">
                        <h2>No files to transfer</h2>
                        <p>
                            <small>
                                <span></span>
                                <Button 
                                    component="label"
                                    color="neutral"
                                    startDecorator={
                                        <SvgIcon>
                                            <svg
                                                xmlns="http://www.w3.org/2000/svg"
                                                fill="none"
                                                viewBox="0 0 24 24"
                                                strokeWidth={1.5}
                                                stroke="currentColor"
                                            >
                                                <path
                                                    strokeLinecap="round"
                                                    strokeLinejoin="round"
                                                    d="M12 16.5V9.75m0 0l3 3m-3-3l-3 3M6.75 19.5a4.5 4.5 0 01-1.41-8.775 5.25 5.25 0 0110.233-2.33 3 3 0 013.758 3.848A3.752 3.752 0 0118 19.5H6.75z"
                                                />
                                            </svg>
                                        </SvgIcon>
                                    }
                                >
                                    Upload file
                                    <VisuallyHiddenInput type="file" />
                                </Button>
                            </small>
                        </p>
                    </div>
                ) }
            </div>
        ) }
        </>
    )
}

interface ListProps {
    array: Array<object>
}

const CustomList = (props: ListProps) => {

    const itemList = props.array.map((item, index) => (
        <ListItem disablePadding key={ index }>
            <ListItemText primary={ item.name } />
        </ListItem>
    ))

    return (
        <List>
            { itemList }
        </List>
    )
}

export default FileList
