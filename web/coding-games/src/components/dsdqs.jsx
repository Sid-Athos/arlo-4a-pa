import {createSignal, For, Show} from "solid-js";
import {
    Avatar, Badge,
    Box,
    Button,
    Card,
    CardActions,
    CardContent,
    CardHeader,
    createTheme,
    FormGroup,
    Grid, IconButton,
    List,
    ListItem,
    ListItemButton,
    ListItemText, styled,
    TextField,
    ThemeProvider, Typography
} from "@suid/material";
import MoreVertIcon from "@suid/icons-material/MoreVert";
import {Clear} from "@suid/icons-material";
import ClearIcon from "@suid/icons-material/Clear";
import MailIcon from "@suid/icons-material/Mail";
import {createStore} from "solid-js/store";

export default function Chat() {
    const [showChat, setShowChat] = createSignal(true)
    const [recipients, setRecipients] = createSignal([{user: "Arnaud"}, {user:"Armand"}])
    const [messages, setMessages] = createSignal([{message: "lol", user : {pseudo: "sid"}}, {message: "Vas y joue le coin en haut à droite fdp", user : {pseudo: "lol"}}, {message: "lodsqdqsl", user : {pseudo: "lol"}}, {message: "lodsqdqsl", user : {pseudo: "lol"}}, {message: "lodsqdqsl", user : {pseudo: "lol"}}, {message: "lodsqdqsl", user : {pseudo: "lol"}}])
    const [messageTerm, setMessageTerm] = createSignal('');
    const [messageId, setMessageId] = createSignal('vision');
    const darkTheme = createTheme({
        palette: {
            mode: 'dark',
            backgroundColor:"#decba4"
        },
    });



    const handleShowChat = () => {
        let current = showChat();
        setShowChat(!current)
    }


    let date = new Date().toDateString();

    return (<>
        <ThemeProvider theme={darkTheme}>
            <Show when={!showChat()}>
                <Button sx={{backgroundColor:"black", position:"absolute", bottom:0, right:0, width:200}} variant="outlined" onclick={handleShowChat}>Open chat
                    <Badge badgeContent={4} color="primary" sx={{position:"absolute", right:20}}>
                        <MailIcon color="action"/>
                    </Badge>
                </Button>
            </Show>
            <Show when={showChat()}>

                <Card sx={{minWidth: 400, maxWidth:400, mL: 3, minHeight:400, position:"absolute", bottom:0, right:0}}>
                    <CardHeader
                        action={
                            <IconButton aria-label="settings" onclick={handleShowChat} >
                                <ClearIcon sx={{color:"#fff"}} />
                            </IconButton>
                        }
                        title={recipients().map(users => users.user).join(", ")}
                        sx={{fontSize:12}}
                        subheader={date}
                    />
                    <CardContent sx={{minWidth: 275, maxWidth:500, maxHeight:180, mL: 3, overflow:"scroll"}}>
                            <Grid container>
                                <Grid item xs={5} md={4} sx={{minWidth: 350}}>
                                    <div style={{paddingTop:"20"}}>
                                    <List disablePadding>
                                                <For each={messages()}>{(message,i) => {
                                                    console.log(i() === messages().length - 1)
                                                        if(message.user.pseudo === "sid"){
                                                            if(i() === messages().length - 1){
                                                                return (
                                                                    <>
                                                                        <Typography >Sid</Typography>
                                                                        <ListItem disablePadding sx={{marginBottom:"-20px", color:"white"}} >
                                                                            <ListItemButton autoFocus>
                                                                                <div className={"bubble mini"}>
                                                                                <ListItemText primary={message.message} />
                                                                                </div>
                                                                                <ListItemText primary="" />

                                                                            </ListItemButton>
                                                                        </ListItem>
                                                                    </>
                                                                )
                                                            } else {
                                                                return (
                                                                    <>
                                                                        <ListItem disablePadding sx={{marginBottom:"-20px"}}>
                                                                            <ListItemButton>
                                                                                <div className={"bubble mini"}>
                                                                                    <ListItemText primary={message.message} />
                                                                                </div>
                                                                                <ListItemText primary="" />
                                                                            </ListItemButton>
                                                                        </ListItem>
                                                                    </>
                                                                )
                                                            }
                                                        } else {
                                                            if(i() === messages().length - 1) {
                                                                return (
                                                                    <>
                                                                        <ListItem disablePadding sx={{marginBottom:"-20px"}}>
                                                                            <ListItemButton autoFocus>
                                                                                <ListItemText primary="" />
                                                                                <div className={"bubble mini"}>
                                                                                    <ListItemText primary={message.message} />
                                                                                </div>
                                                                            </ListItemButton>
                                                                        </ListItem>
                                                                    </>
                                                                )
                                                            } else {
                                                                return (
                                                                    <>
                                                                        <ListItem disablePadding sx={{marginBottom:"-20px"}} >
                                                                            <ListItemButton>

                                                                            <ListItemText primary="" />
                                                                            <div className={"bubble mini"}>
                                                                                <ListItemText primary={message.message} />
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
                        <Box sx={{position:"relative", maxWidth:200, right:-40, bottom:0}} align={"right"}>
                            <FormGroup >
                                <TextField multiline sx={{width:300}}></TextField>
                                <Button>Send Message</Button>
                            </FormGroup>

                        </Box>
                    </CardActions>
                </Card>
            </Show>
        </ThemeProvider>
    </>)
}
