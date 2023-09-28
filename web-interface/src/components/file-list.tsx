import { useState, useEffect } from "react"
import { List, ListItem, ListItemText} from "@mui/material"
// import { styled, Button, SvgIcon } from "@mui/joy"

import ExtendableContainer from "./custom-components/extendable-container"
import { uploadListProps } from "../interfaces/props"
import { downloadFile } from "../services/fileManagement"
import { getServerAddress } from "../services/localStorage"
import { formatBytes } from "../services/formatting"
import { CustomFile } from "../interfaces/file"
import useFetchFilesHook from "../hooks/fetchFilesHook"

const FileList = () => {
    const [localFiles, setLocalFiles] = useState([])
    const { fetchFiles, activate, deactivate } = useFetchFilesHook()

    useEffect(() => {
        // Fetch files whenever the hook is activated
        getLocalFiles()
        deactivate()
    }, [fetchFiles])

    const getLocalFiles = () => {
        fetch(getServerAddress(`/files`))
        .then(response => {
            if (response.ok) {
                response.json()
                    .then(content => {
                        console.log("content", content)
                        setLocalFiles(content.files)
                    })
                    .catch(error => console.log("Failed to fetch files", error))
            }

            else console.log("Fetching local files failed", response)
        })
        .catch(error => console.log("Error occured while fetching files", error))
    }

    return (
        <>
        <ExtendableContainer header={ <h2>Files ({ localFiles.length })</h2> }>
            <div className="section-container">
                <div className="files-container">
                    {
                        (localFiles.length === 0)
                        ? <h3>No files have been added</h3>
                        : (
                            <>
                            <CustomLocalFileList fileList={ localFiles }
                                setFiles={ setLocalFiles }
                                removeFile={ (filename: string, uploader: string) => {
                                    fetch(getServerAddress(`/files?username=${ uploader }&file_name=${ filename }`), {
                                        method: "DELETE"
                                    })
                                    .then(response => {
                                        if (response.ok) {
                                            response.json()
                                                .then(data => {
                                                    if (typeof data === "object") {
                                                        console.log("File removed successfully")
                                                    }

                                                    activate()
                                                    
                                                })
                                                .catch(error => console.log(error))
                                        }

                                        else console.log(`Removing '${ filename }' file failed`)
                                    })
                                    .catch(error => console.log("Error occured while removing a file:", error))
                                } }
                                downloadFile={ (filename: string | object, uploader: string) => {
                                    fetch(getServerAddress(`/download?file_name=${ filename }&username=${ uploader }`))
                                        .then(response => {
                                            if (response.ok) {
                                                downloadFile(response.body, filename)
                                            }

                                            else console.log("Download failed")
                                        })
                                        .catch(error => console.log("Download error:", error))
                                } }
                            />
                            </>
                        )
                    }
                </div>
            </div>
        </ExtendableContainer>
        </>
    )
}

const CustomLocalFileList = (props: uploadListProps) => {
    const itemList = props.fileList.map((item, index) => {
        const file = item as CustomFile

        return (
            <ListItem disablePadding key={ index }>
                <button className="bg-red" onClick={ () => props.removeFile(file.name, file.owner) }>Remove</button>
                <button className="bg-blue" onClick={ () => (props.downloadFile) && props.downloadFile(file.name, file.owner) }>Download</button>
                <ListItemText primary={ `${ file.name } ${ formatBytes(file.size) }` } />
            </ListItem>
        )
    })

    return (
        <List>
            { itemList }
        </List>
    )
}

export default FileList
