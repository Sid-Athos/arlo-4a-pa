import {AppBar, Box, Button, Divider, IconButton, Link, Stack, Toolbar, Typography} from "@suid/material";
import MenuIcon from "@suid/icons-material/Menu";
import NotificationsComponent from "../components/notifications";
import {createSignal, Show} from "solid-js";
import {useNavigate} from "@solidjs/router";
import {UserStore} from "../utils/user-store";

export default function NavBar({setOpen}){
    const navigate = useNavigate();
    const handleOpen = () => setOpen(true);

    const nav = (url)=> {
        navigate(url)
    }
    return (<>


                <AppBar position="sticky" sx={{backgroundColor: '#282c34'}}>
                    <Toolbar sx={{justifyContent:"center"}}>
                            <Stack
                                direction="row"
                                spacing={3}
                                sx={{fontSize:20}}
                            >
                                <Link href="#"  underline={"none"} color="white" onclick={() => nav("/lobby")}
                                > Games</Link>
                                <Link href={"#"}  underline={"none"} color="white"  onclick={() => nav("/ranking")}> Leaderboard</Link>
                                <Link href={"#"}  underline={"none"} color="white"  onclick={() => nav("/code-editor")}> Code Editor</Link>
                                <Link href={"#"} underline={"none"} color="white"  onclick={() => nav("/search-user")}> Search Users</Link>
                                <Link href={"#"} underline={"none"} color="white"  onclick={() => nav("/chat")}> Chat</Link>

                        <div>
                            <Show when={!UserStore.get().token && !sessionStorage.getItem("token")}>

                                <div color="inherit" onClick={handleOpen} sx={{justifyContent:"flex-start"}} >Login</div>
                            </Show>

                        </div>
                            <NotificationsComponent></NotificationsComponent>
                            </Stack>

                    </Toolbar>
                </AppBar>

        </>)
}