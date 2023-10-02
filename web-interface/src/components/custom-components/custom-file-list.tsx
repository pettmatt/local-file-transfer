import { formatBytes } from "../../services/formatting"
import RemoveCircleOutlineIcon from "@mui/icons-material/RemoveCircleOutline"
import Chip from "@mui/material/Chip"
import Stack from "@mui/material/Stack"

type CustomFunction = (parameter: string) => void | object

interface CustomListProps {
    objectList: Array<object>,
    removeItem: CustomFunction
}

interface CustomObject {
    name: string,
    size: number,
    type: string
}

const CustomFileList = (props: CustomListProps) => {

    const itemList = props.objectList.map((item, index) => {
        const file = item as CustomObject

        return (
            <li key={ index }>
                <div className="button">
                    <Stack>
                        <Chip label={ <RemoveCircleOutlineIcon /> } className="transparent-bg"
                            onClick={ () => props.removeItem(file.name) }
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
        <ul>
            { itemList }
        </ul>
    )
}

export default CustomFileList