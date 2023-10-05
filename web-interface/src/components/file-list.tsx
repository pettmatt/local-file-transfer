import { useState, useEffect } from "react"
import { downloadFile } from "../services/fileManagement"
import { getServerAddress } from "../services/localStorage"
import DownloaderFileList from "./component-parts/downloader-file-list"
import ExtendableContainer from "./custom-components/extendable-container"

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
                            <DownloaderFileList fileList={ localFiles }
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
                                downloadFile={ (filename: string, uploader: string, loadingSetter: React.Dispatch<React.SetStateAction<boolean | null>>) => {
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

export default FileList