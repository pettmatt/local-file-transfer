import { useState, useEffect } from "react"
import { styled, Button, SvgIcon } from "@mui/joy"
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

        const newArray = Array.from(inputFiles).map(file => file)

        setFiles(prevFiles => [...prevFiles , ...newArray])
    }

    // const removeFile = (index: number) => {
    //     const fileList: Array<File> = files
    //     const newList = fileList.filter(file => file.id !== index)
    //     setFiles(...newList)
    // }

    const sendFiles = () => {
        const filesFormData = new FormData()

        files.map((file, index) => {
            filesFormData.append(`file_${ index }`, file)
        })

        console.log("FILES", files)
        console.log(filesFormData)

        http.request("http://127.0.0.1:7878/send-file", "POST", {
            body: filesFormData
        })
        .then(response => console.log("File send successfully:", response))
        .catch(error => console.log(error))
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
                        <CustomList array={ files } />
                    </div>
                ) }
            </div>
        </div>
        </>
    )
}

interface ListProps {
    array: Array<object>
}

const CustomList = (props: ListProps) => {

    const formatBytes = (bytes: number, decimals = 2): string => {
        if (bytes === 0) return '0 Bytes'

        const k = 1024
        const dm = decimals < 0 ? 0 : decimals
        const sizes = ['Bytes', 'KB', 'MB', 'GB', 'TB', 'PB', 'EB', 'ZB', 'YB']

        const i = Math.floor(Math.log(bytes) / Math.log(k))

        return parseFloat((bytes / Math.pow(k, i)).toFixed(dm)) + ' ' + sizes[i]
    }

    const itemList = props.array.map((item, index) => (
        <ListItem disablePadding key={ index }>
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
