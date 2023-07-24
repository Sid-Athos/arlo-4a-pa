import {AppBar, Box, Button, Divider, IconButton, Link, Stack, Toolbar, Typography} from "@suid/material";
import MenuIcon from "@suid/icons-material/Menu";
import NotificationsComponent from "../components/notifications";
import {createSignal} from "solid-js";
import {useNavigate} from "@solidjs/router";

export default function NavBar({setOpen}){
    const navigate = useNavigate();
    const handleOpen = () => setOpen(true);

    const nav = (url)=> {
        navigate(url)
    }
    return (<>


                <AppBar position="sticky" sx={{backgroundColor: '#282c34'}}>
                    <Toolbar>
                        <IconButton
                            size="large"
                            edge="start"
                            color="inherit"
                            aria-label="menu"
                            sx={{mr: 2}}
                        >
                            <MenuIcon/>
                        </IconButton>
                        <Typography variant="h6" component="div" sx={{flexGrow: 1}}>
                            <Stack
                                direction="row"
                                divider={<Divider orientation="vertical" flexItem/>}
                                spacing={4}
                            >
                                <Link href="#"  underline={"none"} color="white" onclick={() => nav("/game-lobby")}> Games</Link>
                                <Link href={"#"}  underline={"none"} color="white" onclick={() => nav("/ranking")}> Leaderboard</Link>
                                <Link href={"#"}  underline={"none"} color="white" onclick={() => nav("/code-editor")}> Code Editor</Link>
                                <Link href={"#"} underline={"none"} color="white" onclick={() => nav("/search-user")}> Search Users</Link>
                                <Link href={"#"} underline={"none"} color="white" onclick={() => nav("/meeting")}> Meeting</Link>

                            </Stack>
                        </Typography>
                        <Button color="inherit" onClick={handleOpen}>Login</Button>
                        <div>

                            <NotificationsComponent></NotificationsComponent>
                        </div>
                    </Toolbar>
                </AppBar>

        </>)
}