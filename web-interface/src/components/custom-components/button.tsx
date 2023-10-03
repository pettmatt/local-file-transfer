import { ReactNode } from "react"
import { Stack } from "@mui/joy"
import { Chip } from "@mui/material"
import LoadingButton from "@mui/lab/LoadingButton"

interface Props {
    children: ReactNode,
    onClickFunction: Function,
    loading?: boolean
}

const Button = (props: Props) => {
    return (
        <Stack>
            {
                <Chip label={ props.children } className="button download bg-blue"
                    onClick={ () => (props.onClickFunction) && props.onClickFunction() }
                />
            }

            {
                <LoadingButton
                    loading
                    loadingPosition="start"
                    startIcon={ "S" }
                    variant="outlined"
                >
                    { props.children }
                </LoadingButton>
            }
        </Stack>
    )
}

export default Button