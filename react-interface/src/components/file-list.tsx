import { useState } from "react"

interface Props {
    devices: Array<object>,
    files: Array<object>
}

const FileList = (props: Props) => {
    const [devices, setDevices] = useState(props.devices)
    const [files, setFiles] = useState(props.files)

    const filesListed = files.map(file => (
        <>
        <span>{ file.name }</span>
        <button key={ file.id }>X</button>
        </>
    ))

    return (
        <>
        { devices.length > 0 && (
            <div className="section-container">
                { files.length > 0 ? (
                    <div className="list-container">
                        { filesListed }
                    </div>
                )
                :
                (
                    <div className="notification-container no-files">
                        <h2>No files to transfer</h2>
                        <p>
                            <small>
                                Please drop a file here
                            </small>
                        </p>
                    </div>
                ) }
            </div>
        ) }
        </>
    )
}

export default FileList
