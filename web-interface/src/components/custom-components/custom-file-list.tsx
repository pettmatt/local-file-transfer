import { List, ListItem, ListItemText} from "@mui/material"
import { formatBytes } from "../../services/formatting"

type customFunction = (parameter: string) => void | object

interface CustomListProps {
    objectList: Array<object>,
    removeItem: customFunction
}

interface CustomObject {
    name: string,
    size: number
    type: string
}

const CustomFileList = (props: CustomListProps) => {

    const itemList = props.objectList.map((item, index) => {
        const file = item as CustomObject

        return (
            <ListItem disablePadding key={ index }>
                <button onClick={ () => props.removeItem(file.name) }>Remove</button>
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

export default CustomFileList