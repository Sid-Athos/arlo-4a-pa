/* @refresh reload */
import {render} from 'solid-js/web';
// @ts-ignore
import './index.css';
import {Router, Route, Routes, useNavigate} from "@solidjs/router";
import {AppBar, Box, Button, Divider, IconButton, Stack, TextField, Toolbar, Typography} from "@suid/material";
import MenuIcon from "@suid/icons-material/Menu";
import {createSignal} from "solid-js";
// @ts-ignore
import LoginComponent from "./components/login.jsx";
// @ts-ignore
import GameEditorComponent from "./components/game-editor.jsx";
// @ts-ignore
import LobbyComponent from "./components/lobby.jsx";
// @ts-ignore
import RankingComponent from "./components/rankings.jsx";
// @ts-ignore
import SearchComponent from "./components/search-bar.jsx";
// @ts-ignore
import GameLobbyComponent from "./components/game-lobby.jsx";

// @ts-ignore
import ListUsersComponent from "./components/list-users.jsx";

// @ts-ignore
import NotificationsComponent from "./components/notifications.jsx";
// @ts-ignore
import {UserProvider} from "./components/user-provider.jsx";
import SearchIcon from "@suid/icons-material/Search";
const root = document.getElementById('root');

// @ts-ignore
if (import.meta.env.DEV && !(root instanceof HTMLElement)) {
    throw new Error(
        'Root element not found. Did you forget to add it to your index.html? Or maybe the id attribute got mispelled?',
    );
}
const handleOpen = () => setOpen(true);
const [open, setOpen] = createSignal(false);
import { A } from "@solidjs/router";


// @ts-ignore
render(
    () => (
        <Router>
        <UserProvider token={""}>
            <Box sx={{backgroundColor: '#282c34', minHeight: '100vh'}}>
                <Box sx={{flexGrow: 1}}>
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
                                    <Typography>
                                        <A href="/game_lobbies"> Games</A>
                                    </Typography>
                                    <Typography>
                                        <A href={"/ranking"}> Leaderboard</A>
                                    </Typography>
                                    <Typography>
                                        <A href={"/code-editor"}> Code Editor</A>
                                    </Typography>
                                    <Typography>
                                        <A href={"/search-user"}> Search Users</A>
                                    </Typography>
                                </Stack>
                            </Typography>
                            <Button color="inherit" onClick={handleOpen}>Login</Button>
                            <div>

                                <NotificationsComponent></NotificationsComponent>
                            </div>
                        </Toolbar>
                    </AppBar>
                </Box>
                    <Routes>
                        {/* @ts_ignore */}
                        <Route path={"/"}
                               component={<LoginComponent open={open} setOpen={setOpen}></LoginComponent>}></Route>
                        <Route path={"/lobby"} component={<LobbyComponent></LobbyComponent>}></Route>
                        <Route path={"/code-editor"} component={<GameEditorComponent></GameEditorComponent>}></Route>
                        <Route path={"/ranking"} component={<RankingComponent></RankingComponent>}></Route>
                        <Route path={"/game-lobbies"} component={<GameLobbyComponent></GameLobbyComponent>}></Route>
                        <Route path={"/search-user"} component={<ListUsersComponent ></ListUsersComponent>}></Route>
                    </Routes>
            </Box>
        </UserProvider>
                </Router>
    ), root!);