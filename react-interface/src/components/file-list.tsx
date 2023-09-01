import { useState } from "react"
// import { styled, Button, SvgIcon } from "@mui/joy"
import { List, ListItem, ListItemText} from "@mui/material"

import * as http from "../services/http-requests"

// interface File {
//     id: number,
//     name: string,
//     type: string,
//     size: number,
//     content?: string
// }

const FileList = () => {
    const [files, setFiles] = useState([])

    const handleChange = (event) => {
        const inputFiles = event.target.files

        const newArray = Array.from(inputFiles).filter(newFile =>
            !files.some(existingFiles => existingFiles.name === newFile.name)
        )

        event.target.value = null

        setFiles(prevFiles => [...prevFiles , ...newArray])
    }

    const sendFiles = () => {
        files.map((file, index) => {
            const fileReader = new FileReader()
            fileReader.readAsArrayBuffer(file)

            fileReader.onload = (event) => {
                const content = event.target.result
                const chunkSize = 1000
              
                const totalChunks = event.target.result.byteLength / chunkSize
                const id = Math.random().toString(36).slice(-10)

                for (let i = 0; i < totalChunks + 1; i++) {
                    const chunk = content.slice(i * chunkSize, (i + 1) * chunkSize)

                    // if (chunk.size === 0) return
                    console.log("CHUNK", chunk.length)
    
                    http.request(`http://127.0.0.1:7878/send-file?file_id=${ id }`, "POST", {
                        headers: {
                            "content-type": "application/octet-stream",
                            "content-length": chunk.length
                        },
                        body: {
                            chunk: chunk,
                            name: file.name
                        }
                    })
                    .then(response => console.log("File send successfully:", response))
                    .catch(error => console.log(error))

                    // remove 'return' later
                    return
                }
            }
        })
        console.log("FILES", files)
    }

    return (
        <>
        <div className="section-container">
            <div className="notification-container no-files">
                <h3>No files to transfer</h3>
                <input type="file" name="filepicker" multiple onChange={ handleChange } />
                <div>
                    <button onClick={ sendFiles }>Send</button>
                </div>
                { files.length > 0 && (
                    <div className="list-container">
                        <CustomList fileList={ files } setFiles={ setFiles } />
                    </div>
                ) }
            </div>
        </div>
        </>
    )
}

interface ListProps {
    fileList: Array<object>,
    setFiles: Function
}

const CustomList = (props: ListProps) => {

    const formatBytes = (bytes: number, decimals = 2): string => {
        if (bytes === 0) return '0 Bytes'

        const k = 1024
        const dm = decimals < 0 ? 0 : decimals
        const sizes = ['Bytes', 'KB', 'MB', 'GB', 'TB', 'PB', 'EB', 'ZB', 'YB']

        const i = Math.floor(Math.log(bytes) / Math.log(k))

        return parseFloat((bytes / Math.pow(k, i)).toFixed(dm)) + " " + sizes[i]
    }

    const removeFile = (fileName: string) => {
        const newList = props.fileList.filter(file => file.name !== fileName)
        props.setFiles(newList)
    }

    const itemList = props.fileList.map((item, index) => (
        <ListItem disablePadding key={ index }>
            <button onClick={ () => removeFile(item.name) }>Remove</button>
            <ListItemText primary={ `${ item.name } ${ item.type } ${ formatBytes(item.size) }` } />
        </ListItem>
    ))

    return (
        <List>
            { itemList }
        </List>
    )
}

export default FileList
