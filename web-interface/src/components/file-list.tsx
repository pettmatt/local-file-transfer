import { useState, useEffect, ChangeEvent } from "react"
import { List, ListItem, ListItemText} from "@mui/material"
// import { styled, Button, SvgIcon } from "@mui/joy"

import { uploadListProps } from "../interfaces/props"
import { downloadFile } from "../services/fileManagement"
import { getServerAddress } from "../services/localStorage"
import { formatBytes } from "../services/formatting"

const FileList = () => {
    const [localFiles, setLocalFiles] = useState([])

    useEffect(() => {
        getLocalFiles()
    }, [])

    const getLocalFiles = () => {
        fetch(getServerAddress(`/local-files`))
        .then(response => {
            if (response.ok) {
                response.json()
                    .then(content => {
                        console.log(content)
                        setLocalFiles(content.files)
                    })
                    .catch(error => console.log("Failed to fetch local files", error))
            }

            else console.log("Fetching local files failed", response)
        })
        .catch(error => console.log("Error occured while fetching local files", error))
    }

    return (
        <>
        <div className="section-container">
            <div className="notification-container">
                {
                    (localFiles.length === 0)
                    ? <h3>No files have been added</h3>
                    : (
                        <>
                        <h3>Your files ({ localFiles.length })</h3>
                        <CustomLocalFileList fileList={ localFiles }
                            setFiles={ setLocalFiles }
                            removeFile={ (filename: string) => {
                                fetch(getServerAddress(`/local-file?file_name=${ filename }&user_name=${ localStorage.getItem("name") }`), {
                                    method: "DELETE"
                                })
                                .then(response => {
                                    if (response.ok) {
                                        response.json()
                                            .then(data => {
                                                if (typeof data === "object") {
                                                    console.log("File removed successfully")
                                                }

                                                getLocalFiles()
                                            })
                                            .catch(error => console.log(error))
                                    }

                                    else console.log(`Removing '${ filename }' file failed`)
                                })
                                .catch(error => console.log("Error occured while removing a file:", error))
                            } }
                            downloadFile={ (filename: string | object) => {
                                fetch(getServerAddress(`/download-file?file_name=${ filename }`))
                                    .then(response => {
                                        if (response.ok) {
                                            downloadFile(response.body, filename)
                                        }

                                        else console.log("Download failed")
                                    })
                                    .catch(error => console.log("Error", error))
                            } }
                        />
                        </>
                    )
                }
            </div>
        </div>
        </>
    )
}

const CustomLocalFileList = (props: uploadListProps) => {
    const itemList = props.fileList.map((item, index) => {
        const file = item as CustomFile

        return (
            <ListItem disablePadding key={ index }>
                <button onClick={ () => props.removeFile(file.name) }>Remove</button>
                <button onClick={ () => (props.downloadFile) && props.downloadFile(file.name) }>Download</button>
                <ListItemText primary={ `${ file.name } ${ file.type } ${ formatBytes(file.size) }` } />
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
