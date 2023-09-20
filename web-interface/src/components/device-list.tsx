import { List, ListItem, ListItemText} from "@mui/material"
import { deviceListProps, customDeviceListProps, Device } from "../interfaces/props"

const DeviceList = (props: deviceListProps) => {

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
                            Make sure you're connected to the local network through Wi-Fi or Ethernet cable.
                        </small>
                    </p>
                </div>
            ) }
        </div>
    )
}

const CustomList = (props: customDeviceListProps) => {

    const itemList = props.array.map((item, index) => {
        const device = item as Device

        return (
            <ListItem disablePadding key={ index }>
                <ListItemText primary={ device.name } />
            </ListItem>
        )
    })

    return (
        <List>
            { itemList }
        </List>
    )
}

export default DeviceList
