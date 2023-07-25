import {createMemo, createSignal, For, Show} from "solid-js";
import {
    Badge,
    Box,
    Button,
    Card,
    CardActions,
    CardContent,
    CardHeader,
    createTheme,
    Divider,
    FormGroup,
    Grid,
    IconButton,
    List,
    ListItem,
    ListItemButton,
    ListItemText,
    TextField,
    ThemeProvider
} from "@suid/material";
import ClearIcon from "@suid/icons-material/Clear";
import MailIcon from "@suid/icons-material/Mail";
import {UserStore} from "../utils/user-store";

export default function Chat({socket, sendMessage, conversation, setConvo}) {
    const [showChat, setShowChat] = createSignal(true)
    const [rowsValue, setRowsValue] = createSignal(2)

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
        console.log(conversation())
        return [...new Set(conversation().map(message => message.from_user.pseudo))];
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

    return (<>
        <ThemeProvider theme={darkTheme}>
            <Show when={!showChat()}>
                <Button sx={{backgroundColor: "black", position: "absolute", bottom: 0, right: 0, width: 200}}
                        variant="outlined" onclick={handleShowChat}>Open chat
                    <Badge badgeContent={4} color="primary" sx={{position: "absolute", right: 20}}>
                        <MailIcon color="action"/>
                    </Badge>
                </Button>
            </Show>
            <Show when={showChat()}>
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
                            <IconButton aria-label="settings" onclick={handleShowChat}>
                                <ClearIcon sx={{color: "#fff"}}/>
                            </IconButton>
                        }
                        sx={{maxHeight: 50}}
                        title={recipients().join(", ")}
                        titleTypographyProps={{variant: 'subtitle1'}}
                        subheader={date}
                    />
                    <Divider/>
                    <Divider/>
                    <CardContent sx={{minWidth: 275, maxWidth: 500, maxHeight: 280, mL: 3, overflow: "scroll"}}>
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
                                                if (i() === conversation.length - 1) {
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
                        <Box sx={{position: "absolute", maxWidth: 200, right:-20, bottom:0}} align={"right"}>
                                <TextField multiline sx={{width: 300, position:"relative", bottom:0}} rows={rowsValue()}
                                           onkeydown={sendOnCtrlEnter}></TextField>
                                {/**<Button >Send Message</Button>*/}
                        </Box>
                </Card>
            </Show>
        </ThemeProvider>
    </>)
}
