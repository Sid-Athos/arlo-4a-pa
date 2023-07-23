import {MainContainer} from "@minchat/react-chat-ui";
import {createSignal, For} from "solid-js";
import {
    Box, Button, Card, CardActions, CardContent,
    createTheme, FormGroup,
    Grid, List, ListItem, ListItemButton, ListItemIcon, ListItemText,
    Paper,
    styled,
    Table,
    TableBody,
    TableCell,
    TableContainer,
    TableHead,
    TableRow, TextField,
    ThemeProvider
} from "@suid/material";

export default function Chat() {
    const [messages, setMessages] = createSignal([{message: "lol", user : {pseudo: "sid"}}, {message: "lodsqdqsl", user : {pseudo: "lol"}}, {message: "lodsqdqsl", user : {pseudo: "lol"}}, {message: "lodsqdqsl", user : {pseudo: "lol"}}, {message: "lodsqdqsl", user : {pseudo: "lol"}}, {message: "lodsqdqsl", user : {pseudo: "lol"}}])
    const [messageTerm, setMessageTerm] = createSignal('');
    const [messageId, setMessageId] = createSignal('vision');
    const darkTheme = createTheme({
        palette: {
            mode: 'dark',
            backgroundColor:"#decba4"
        },
    });


    return (<>
        <ThemeProvider theme={darkTheme}>
        <Card sx={{minWidth: 275, maxWidth:500, mL: 3, position:"absolute", bottom:0, right:0}}>
            <CardContent sx={{minWidth: 275, maxWidth:500, maxHeight:200, mL: 3, overflow:"scroll"}}>
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
                                                                <ListItem disablePadding sx={{marginBottom:"-20px"}} >
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
        </ThemeProvider>
    </>)
}
