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
import LinearProgressBar from "./custom-components/linearProgressBar"

interface Props {
    fileHook: {
        fetchFiles: boolean,
        activate: () => void,
        deactivate: () => void
    }
}

const FileList = (props: Props) => {
    const [localFiles, setLocalFiles] = useState([])
    const [connectionError, setConnectionError] = useState(false)
    const { fetchFiles, activate, deactivate } = props.fileHook

    useEffect(() => {
        // Fetch files whenever the hook is activated
        getLocalFiles()
        deactivate()
    }, [fetchFiles])

    useEffect(() => {
        const fileUpdateInterval = setInterval(() => {
            activate()
        }, 5000)

        return () => clearInterval(fileUpdateInterval)
    }, [])

    const getLocalFiles = () => {
        fetch(getServerAddress(`/files`))
            .then(response => {
                if (response.ok) {
                    response.json()
                        .then(content => {
                            setLocalFiles(content.files)
                            setConnectionError(false)
                        })
                        .catch(error => {
                            // If the fetch operation fails the API isn't possibly running correctly
                            console.log("Failed to fetch files", error)
                            setConnectionError(true)
                        })
                }

                else {
                    console.log("Fetching local files failed", response)
                    setConnectionError(true)
                }
            })
            .catch(error => {
                console.log("Error occured while fetching files", error)
                setConnectionError(true)
            })
    }

    return (
        <>
        <ExtendableContainer header={ <h2>Files ({ localFiles.length })</h2> } showOnLoad={ true }>
            <div className="section-container">
                <div className="files-container">
                    {
                        (localFiles.length === 0)
                        ? (
                            <div className="file-message">
                                <h3>No files found</h3>
                                { (connectionError) && (
                                    <p>
                                        Seems like the server doesn't respond with expected response. Please make sure the server is set correctly and is running.
                                    </p>
                                )}
                            </div>
                        )
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
                                downloadFile={ (filename: string | object, uploader: string, loadingSetter: React.Dispatch<React.SetStateAction<any>>) => {
                                    loadingSetter(true)
                                    fetch(getServerAddress(`/download?file_name=${ filename }&username=${ uploader }`))
                                        .then(response => {
                                            if (response.ok) {
                                                downloadFile(
                                                    response.body,
                                                    filename,
                                                    Number(response.headers.get("content-length")),
                                                    loadingSetter
                                                )
                                            }

                                            else {
                                                console.log("Download failed")
                                                loadingSetter(null)
                                            }
                                        })
                                        .catch(error => {
                                            console.log("Download error:", error)
                                            loadingSetter(null)
                                        })
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
    const rootFolder = "uploads"
    const list = separateByProperty(props.fileList, "owner")

    const listElements = list.map((childList, index1) => {

        const listedItems = childList.map((item, index2) => {
            const file = item as CustomFile
            return <ListItem index={ index2 } file={ file } removeFile={ props.removeFile } downloadFile={ props.downloadFile } />
        })

        const owner = childList[0].owner

        const finalList = (owner !== rootFolder)
        ? (
            <ExtendableContainer header={ <h2>{ owner } ({ childList.length })</h2> } showOnLoad={ true } key={ index1 }>
                { listedItems }
            </ExtendableContainer>
        )
        : (
            <ExtendableContainer header={ <h2>root ({ childList.length })</h2> } showOnLoad={ true }>
                { listedItems }
            </ExtendableContainer>
        )

        return finalList
    })

    return (
        <ul className="file-list">
            { listElements }
        </ul>
    )
}

export default FileList

interface ListItemProps {
    index: number,
    file: CustomFile
}

const ListItem = (props: uploadListProps & ListItemProps) => {
    const [progress, setProgress] = useState(null)
    const file = props.file

    return (
        <li key={ props.index } className="file-list-item">
            <div className="buttons">
                <Stack>
                    <Chip label={ <DeleteIcon /> } className="button delete"
                        onClick={ () => props.removeFile(file.name, file.owner) }
                    />
                    <Chip label={ <DownloadIcon /> } className="button download"
                        onClick={ () => (props.downloadFile) && (props.downloadFile(file.name, file.owner, setProgress)) }
                    />
                </Stack>
            </div>
            <div className="details">
                <h3>{ file.name }</h3>
                <span className="type">{ file.type }</span>&nbsp;
                <span className="size">{ formatBytes(file.size) }</span>
                { (progress !== null) &&
                    <LinearProgressBar value={ progress } />
                }
            </div>
        </li>
    )
}

const separateByProperty = (list: Array<any>, property: string) => {
    const lists = {}

    list.forEach(item => {
      const prop = item[property]

      // Check if the list includes a list based on the property value
      if (!lists[prop]) {
        lists[prop] = []
      }
  
      // Push the item to the corresponding list
      lists[prop].push(item)
    })
  
    // Convert the lists object to an array of lists
    const result = Object.values(lists)
  
    return result
}