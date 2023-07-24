import {
    Alert,
    Badge,
    Box,
    createTheme,
    Fade,
    Modal,
    Paper,
    Table,
    TableBody,
    TableCell,
    TableContainer,
    TableHead,
    TableRow,
    ThemeProvider,
    Typography
} from "@suid/material";
import NotificationsIcon from "@suid/icons-material/Notifications";
import {createMemo, createSignal, onMount, Show} from "solid-js";
import {UserService} from "../utils/services/user-service";
import {UserStore} from "../utils/user-store";
import AddIcon from "@suid/icons-material/Check";

import ClearIcon from "@suid/icons-material/Clear";

export default function Notifications() {
    const [notifications, setNotifications] = createSignal([]);
    const [friendAccepted, setFriendAccepted] = createSignal(false);
    const [open, setOpen] = createSignal(false);
    const handleOpen = () => {
        if (notifications().length > 0) {
            setOpen(true);
        }
    }
    const handleClose = () => setOpen(false);

    const notificationsAmount = createMemo(() => {
        return notifications().length;
    });

    onMount(() => {
        setInterval(async () => {
            console.log(UserStore.get().username)
            if (UserStore.get().token) {
                await UserService.friendRequests().then(res => {
                    console.log(res)
                    setNotifications(res.data.filter(friendRequest => friendRequest.applicant.pseudo !== UserStore.get().username))
                })

            }
        }, 800)
    })

    const acceptFriendRequest = async (id) => {
        await UserService.acceptFriend(id)
        setFriendAccepted(true)
        let friendRequests = notifications().filter(notif => notif.id !== id)
        if (friendRequests.length === 0) {
            setOpen(false)
        }
        setNotifications(friendRequests)
    }
    const declineFriendRequest = async (id) => {
        await UserService.declineFriend(id)
    }

    const darkTheme = createTheme({
        palette: {
            mode: 'dark',
        },
    });

    return (
        <>
            <Show when={friendAccepted()} fallback={<></>}>
                <Fade timeout={{enter: 300, exit: 5000}} addEndListener={() => {
                    setTimeout(() => {
                        setFriendAccepted(false);
                    }, 5000);
                }}
                >
                    <Alert className="darkThemeSuccessAlert" severity="success">Friend added</Alert>

                </Fade>
            </Show>
            <Show when={UserStore.get().token}>
            <Badge badgeContent={notificationsAmount()} color={"primary"} onclick={handleOpen} sx={{paddingTop:"6px"}}>
                <NotificationsIcon color="action" sx={{color: "#fff", fontSize:20}}/>
            </Badge>
            </Show>
            <Modal
                open={open()}
                onClose={handleClose}
                aria-labelledby="modal-modal-title"
                aria-describedby="modal-modal-description"
            >
                <Box
                    sx={{
                        position: "absolute",
                        top: "50%",
                        left: "50%",
                        transform: "translate(-50%, -50%)",
                        width: 800,
                        bgcolor: "#282c34",
                        color: "#fff",
                        border: "2px solid #000",
                        boxShadow: "24px",
                        p: 4,
                    }}
                >
                    <Typography>Friend requests</Typography>
                    <ThemeProvider theme={darkTheme}>
                        <TableContainer component={Paper}>
                            <Table sx={{minWidth: 650}} aria-label="simple table">
                                <TableHead>
                                    <TableRow>
                                        <TableCell>#</TableCell>
                                        <TableCell align={"right"}>Username</TableCell>
                                        <TableCell align="right">Level</TableCell>
                                        <TableCell align="right">Experience</TableCell>
                                        <TableCell align="right">Actions</TableCell>
                                    </TableRow>
                                </TableHead>
                                <TableBody>
                                    {notifications().map((query, index) => {
                                            return (
                                                <>
                                                    <TableRow>
                                                        <TableCell>{index + 1}</TableCell>
                                                        <TableCell align={"right"}>{query.recipient.pseudo}</TableCell>
                                                        <TableCell align={"right"}>{query.recipient.experience}</TableCell>
                                                        <TableCell align={"right"}>{query.recipient.level}</TableCell>
                                                        <TableCell align={"right"}>
                                                            <AddIcon
                                                                onclick={() => acceptFriendRequest(query.id)}></AddIcon>
                                                            <ClearIcon
                                                                onclick={() => declineFriendRequest(query.id)}></ClearIcon>
                                                        </TableCell>
                                                    </TableRow>
                                                </>
                                            )
                                        }
                                    )
                                    }
                                </TableBody>
                            </Table>
                        </TableContainer>
                    </ThemeProvider>
                </Box>
            </Modal>
        </>
    );
}