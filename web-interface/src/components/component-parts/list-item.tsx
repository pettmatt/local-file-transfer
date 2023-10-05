import { useState } from "react"
import DeleteIcon from "@mui/icons-material/Delete"
import DownloadIcon from "@mui/icons-material/Download"
import Chip from "@mui/material/Chip"
import Stack from "@mui/material/Stack"
import { uploadListProps } from "../../interfaces/props"
import { CustomFile } from "../../interfaces/file"
import { formatBytes } from "../../services/formatting"
import LinearProgressBar from "../custom-components/linearProgressBar"

interface ListItemProps {
    file: CustomFile,
    index: number
}

const ListItem = (props: uploadListProps & ListItemProps) => {
    const [progress, setProgress] = useState(null)
    const file = props.file
    const index = props.index

    return (
        <li key={ index } className="file-list-item">
            <div className="buttons">
                <Stack>
                    <Chip label={ <DeleteIcon /> } className="button delete"
                        onClick={ () => (props.removeFile) && props.removeFile(file.name, file.owner) }
                    />
                    <Chip label={ <DownloadIcon /> } className="button download"
                        onClick={ () => (props.downloadFile) && props.downloadFile(file.name, file.owner, setProgress) }
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

export default ListItem