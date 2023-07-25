import {createMemo, createSignal, For, Show} from "solid-js";
import {
    Badge,
    Box,
    Button,
    Card,
    CardActions,
    CardContent,
    CardHeader, Container,
    createTheme, Dialog, DialogActions, DialogContent, DialogTitle,
    Divider, FormControl,
    FormGroup,
    Grid,
    IconButton, InputLabel,
    List,
    ListItem,
    ListItemButton,
    ListItemText, Menu, MenuItem, Select,
    TextField,
    ThemeProvider
} from "@suid/material";
import ClearIcon from "@suid/icons-material/Clear";
import MailIcon from "@suid/icons-material/Mail";
import {UserStore} from "../utils/user-store";
import {MoreVert} from "@suid/icons-material";

export default function Chat({socket, sendMessage, conversation, setConvo}) {
    const [showChat, setShowChat] = createSignal(true)
    const [rowsValue, setRowsValue] = createSignal(2)
    const [anchorEl, setAnchorEl] = createSignal(null);
    const [showKickUserDialog, setShowKickUserDialog] = createSignal(false)
    const [showGiveHostDialog, setShowGiveHostDialog] = createSignal(false)
    const [toKick, setToKick] = createSignal({});
    const [toHost, setToHost] = createSignal({})
    const open = () => Boolean(anchorEl());
    const handleClose = (str) => {
        if(str){
            console.log(str)
                if(recipients().length > 0){
            if(str === "Kick"){
                console.log(recipients())
                    setShowKickUserDialog(true)
            } else {
                setShowGiveHostDialog(true)
            }
                }
        }
        setAnchorEl(null);
    };

    const darkTheme = createTheme({
        palette: {
            mode: 'dark',
            backgroundColor: "#decba4"
        },
    });

    const handleShowChat = () => {
        let current = showChat();
        setShowChat(!current)
    }

    const recipients = createMemo(() => {
        return [...new Set(conversation().filter(message => message.from_user.pseudo !== UserStore.get().username))];
    });


    let date = new Date().toDateString();


    const sendOnCtrlEnter = (e) => {
        if (e.ctrlKey && e.key === "Enter") {
            e.preventDefault()
            const messageToSend = e.target.value
            setRowsValue(2)
            sendMessage({Message:{message: messageToSend}})
            e.target.value = ""
        }
    }

    const handleCloseDialog = () => {
        setShowKickUserDialog(false)
    }

    const giveHost = () => {

    }

    const updateUserToKick = (val)=> {
        console.log(val)
    }
    const updateUserForhost = (val)=> {
        console.log(val)
    }

    const handleCloseHostDialog = (e) => {
        setShowGiveHostDialog(false)
    }

    return (<>
        <ThemeProvider theme={darkTheme}>
            <Show when={showGiveHostDialog()}>
                <Dialog
                    open={showGiveHostDialog()}
                    onClose={handleCloseHostDialog}
                    aria-describedby="alert-dialog-slide-description"
                >
                    <DialogTitle>{"Lobby settings"}</DialogTitle>
                    <DialogContent>
                        <FormControl fullWidth>
                            <InputLabel id="demo-simple-select-label">Kick user(s)</InputLabel>
                            <Select
                                labelId="demo-simple-select-label"
                                id="demo-simple-select"

                                label="Age"
                                onchange={updateUserForhost}
                            >
                                {recipients().map(user => {
                                    return (
                                        <>
                                            <MenuItem value={user.from_user.id}>user.from_user.pseudo</MenuItem>

                                        </>
                                    )
                                })}
                                <MenuItem value={2}>Arnaud</MenuItem>
                            </Select>
                        </FormControl>
                    </DialogContent>
                    <DialogActions>
                        <Button onClick={() => handleCloseHostDialog}>Cancel</Button>
                        <Button onClick={() => giveHost}>Kick user(s)</Button>
                    </DialogActions>
                </Dialog>
            </Show>
            <Show when={showKickUserDialog()}>

            <Dialog
                open={showKickUserDialog()}
                onClose={handleCloseDialog}
                aria-describedby="alert-dialog-slide-description"
            >
                <DialogTitle>{"Lobby settings"}</DialogTitle>
                <DialogContent>
                    <FormControl fullWidth>
                        <InputLabel id="demo-simple-select-label">Kick user(s)</InputLabel>
                        <Select
                            labelId="demo-simple-select-label"
                            id="demo-simple-select"
                            value={recipients()}
                            label="Age"
                            onchange={updateUserToKick}
                        >
                            <MenuItem value={1}>Armand</MenuItem>
                            <MenuItem value={2}>Arnaud</MenuItem>
                        </Select>
                    </FormControl>
                </DialogContent>
                <DialogActions>
                    <Button onClick={() => handleCloseDialog}>Cancel</Button>
                    <Button onClick={() => kickUser}>Kick user(s)</Button>
                </DialogActions>
            </Dialog>
            </Show>
            <Show when={!showChat()}>
                <Button sx={{backgroundColor: "black", position: "absolute", bottom: 0, right: 0, width: 200}}
                        variant="outlined" onclick={handleShowChat}>Open chat
                    <Badge badgeContent={4} color="primary" sx={{position: "absolute", right: 20}}>
                        <MailIcon color="action"/>
                    </Badge>
                </Button>
            </Show>
            <Show when={showChat()}>
                <Menu
                    id="basic-menu"
                    anchorEl={anchorEl()}
                    open={open()}
                    onClose={() => handleClose(null)}
                    MenuListProps={{ "aria-labelledby": "basic-button" }}
                >
                    <MenuItem onClick={() => handleClose("Kick")}>KickUser</MenuItem>
                    <MenuItem onClick={() => handleClose("GiveHost")}>Give Host</MenuItem>
                </Menu>
                <Card sx={{
                    minWidth: 400,
                    maxWidth: 400,
                    mL: 3,
                    minHeight: 400,
                    position: "absolute",
                    bottom: 0,
                    right: 0
                }}>
                    <CardHeader
                        action={
                            <IconButton aria-label="settings" onClick={(event) => {
                                setAnchorEl(event.currentTarget);
                            }}>
                                <MoreVert sx={{color: "#fff"}}/>
                            </IconButton>
                        }
                        sx={{maxHeight: 50}}
                        title={recipients().join(", ")}
                        titleTypographyProps={{variant: 'subtitle1'}}
                        subheader={date}
                    />
                    <Divider/>
                    <Divider/>
                    <CardContent sx={{minWidth: 275, maxWidth: 500, maxHeight: 260, mL: 3, overflow: "scroll"}}>
                        <Grid container>
                            <Grid item xs={5} md={4} sx={{minWidth: 350}}>
                                <div style={{paddingTop: "20"}}>
                                    <List disablePadding>
                                        <For each={conversation()}>{(message, i) => {
                                            if (message.from_user.pseudo !== UserStore.get().username) {
                                                if (i() === conversation().length - 1) {
                                                    return (
                                                        <>
                                                            <ListItem disablePadding
                                                                      sx={{marginBottom: "-20px", color: "white"}}>
                                                                <ListItemButton autoFocus>
                                                                    <div className={"bubble mini"}>
                                                                        {message.from_user.pseudo}
                                                                        <ListItemText primary={message.message}/>
                                                                    </div>
                                                                    <ListItemText primary=""/>

                                                                </ListItemButton>
                                                            </ListItem>
                                                        </>
                                                    )
                                                } else {
                                                    return (
                                                        <>
                                                            <ListItem disablePadding sx={{marginBottom: "-20px"}}>
                                                                <ListItemButton>
                                                                    <div className={"bubble mini"}>
                                                                        {message.from_user.pseudo}
                                                                        <ListItemText primary={message.message}/>
                                                                    </div>
                                                                    <ListItemText primary=""/>
                                                                </ListItemButton>
                                                            </ListItem>
                                                        </>
                                                    )
                                                }
                                            } else {
                                                if (i() === conversation().length - 1) {
                                                    return (
                                                        <>
                                                            <ListItem disablePadding sx={{marginBottom: "-20px"}}>
                                                                <ListItemButton autoFocus>
                                                                    <ListItemText primary=""/>
                                                                    <div className={"bubble mini"}>
                                                                        {message.from_user.pseudo}
                                                                        <ListItemText primary={message.message}/>
                                                                    </div>
                                                                </ListItemButton>
                                                            </ListItem>
                                                        </>
                                                    )
                                                } else {
                                                    return (
                                                        <>
                                                            <ListItem disablePadding sx={{marginBottom: "-20px"}}>
                                                                <ListItemButton>
                                                                    <ListItemText primary=""/>
                                                                    <div className={"bubble mini"}
                                                                         sx={{whiteSpace: "pre-wrap"}}>
                                                                        {message.from_user.pseudo}
                                                                        <ListItemText primary={message.message}/>
                                                                    </div>
                                                                </ListItemButton>
                                                            </ListItem>
                                                        </>
                                                    )
                                                }
                                            }
                                        }
                                        }
                                        </For>
                                    </List>
                                </div>
                            </Grid>
                        </Grid>
                    </CardContent>
                    <CardActions>

                    <Container sx={{minHeight:90}}>
                                <TextField multiline sx={{width: 300, position:"absolute",bottom:2, left:60}} id="lol" rows={rowsValue()}
                                           onkeydown={sendOnCtrlEnter} autoFocus></TextField>
                    </Container>
                    </CardActions>
                </Card>
            </Show>
        </ThemeProvider>
    </>)
}
