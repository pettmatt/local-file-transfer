import { uploadListProps } from "../../interfaces/props"
import { CustomFile } from "../../interfaces/file"
import ExtendableContainer from "../custom-components/extendable-container"
import ListItem from "./list-item"

const DownloaderFileList = (props: uploadListProps) => {
    const rootFolder = "uploads"
    const fileList = props.fileList
    const list = separateByProperty(fileList, "owner")

    const listElements = (list as Array<CustomFile[]>).map((childList, index) => {

        const listedItems = childList.map((item: object, childIndex) => {
            const file = item as CustomFile
            return <ListItem key={ childIndex } file={ file } removeFile={ props.removeFile } downloadFile={ props.downloadFile } />
        })

        const owner: string = childList[0].owner

        const finalList = (owner !== rootFolder)
            ? (
                <ExtendableContainer key={ index } header={ <h2>{ owner } ({ childList.length })</h2> } showOnLoad={ true }>
                    { listedItems }
                </ExtendableContainer>
            )
            : (
                <ExtendableContainer key={ index } header={ <h2>root ({ childList.length })</h2> } showOnLoad={ true }>
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

export default DownloaderFileList

type customFileType = {
    [key: string]: object[]
}

const separateByProperty = (list: Array<object> | undefined, property: string) => {
    const lists: customFileType = {}

    if (!list) return list

    list.forEach(item => {
        const file = item as CustomFile
        // eslint-disable-next-line @typescript-eslint/ban-ts-comment
        // @ts-ignore
        const prop = file[property]

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