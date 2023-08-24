import { List, ListItem, ListItemText} from "@mui/material"

interface Props {
    devices: Array<object>
}

const DeviceList = (props: Props) => {

    return (
        <div className="section-container">
            { props.devices.length > 0 ? (
                <div className="list-container">
                    <CustomList array={ props.devices } />
                </div>
            )
            :
            (
                <div className="notification-container no-devices">
                    <h3>No devices found</h3>
                    <p>
                        <small>
                            Make sure you're connected to the local network through WiFi or Ethernet cable.
                        </small>
                    </p>
                </div>
            ) }
        </div>
    )
}

interface ListProps {
    array: Array<object>
}

const CustomList = (props: ListProps) => {
    
    const itemList = props.array.map((item, index) => (
        <ListItem disablePadding key={ index }>
            <ListItemText primary={ item.name } />
        </ListItem>
    ))

    return (
        <List>
            { itemList }
        </List>
    )
}

export default DeviceList
