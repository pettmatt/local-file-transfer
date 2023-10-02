import { useState, useEffect } from "react"
import DeleteIcon from "@mui/icons-material/Delete"
import DownloadIcon from "@mui/icons-material/Download"
import Chip from "@mui/material/Chip"
import Stack from "@mui/material/Stack"

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
        <ExtendableContainer header={ <h2>Files ({ localFiles.length })</h2> } showOnLoad={ true }>
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
                                                activate()
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
            <li key={ index } className="file-list-item">
                <div className="buttons">
                    <Stack>
                        <Chip label={ <DeleteIcon /> } className="button delete bg-red"
                            onClick={ () => props.removeFile(file.name, file.owner) }
                        />
                        <Chip label={ <DownloadIcon /> } className="button download bg-blue"
                            onClick={ () => (props.downloadFile) && props.downloadFile(file.name, file.owner) }
                        />
                    </Stack>
                </div>
                <div className="details">
                    <h3>{ file.name }</h3>
                    <span className="type">{ file.type }</span>&nbsp;
                    <span className="size">{ formatBytes(file.size) }</span>
                </div>
            </li>
        )
    })

    return (
        <ul className="file-list">
            { itemList }
        </ul>
    )
}

export default FileList