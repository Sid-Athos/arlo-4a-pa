/* @refresh reload */
import {render} from 'solid-js/web';
// @ts-ignore
import './index.css';
import {Router, Route, Routes, useNavigate} from "@solidjs/router";
import {AppBar, Box, Button, Divider, IconButton, Link, Stack, TextField, Toolbar, Typography} from "@suid/material";
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

import NavBarComponent from "./render/navbar.jsx";

import Chat from "./components/dsdqs.jsx";

import HomeMeeting from "./screens/Home.jsx";
import Meeting from "./screens/Meeting.jsx";
// @ts-ignore
import {UserProvider} from "./components/user-provider.jsx";
import SearchIcon from "@suid/icons-material/Search";
const root = document.getElementById('root');
const [open, setOpen] = createSignal(false);

// @ts-ignore
if (import.meta.env.DEV && !(root instanceof HTMLElement)) {
    throw new Error(
        'Root element not found. Did you forget to add it to your index.html? Or maybe the id attribute got mispelled?',
    );
}

import { A } from "@solidjs/router";


// @ts-ignore
render(
    () => (
        <Box sx={{backgroundColor: '#282c34', minHeight: '100vh'}}>
            <Box sx={{flexGrow: 1}}>
        <Router>
        <UserProvider token={""}>
            <NavBarComponent setOpen={setOpen} open={open}>
            </NavBarComponent>
                    <Routes>
                        {/* @ts_ignore */}
                        <Route path={"/"}
                               component={<LoginComponent open={open} setOpen={setOpen}></LoginComponent>}></Route>
                        <Route path={"/lobby"} component={LobbyComponent}></Route>
                        <Route path={"/code-editor"} component={GameEditorComponent}></Route>
                        <Route path={"/ranking"} component={RankingComponent}></Route>
                        <Route path={"/game-lobbies"} component={GameLobbyComponent}></Route>
                        <Route path={"/search-user"} component={ListUsersComponent}></Route>
                        <Route path={"/chat"} component={Chat}></Route>
                        <Route path={"/meeting"} component={HomeMeeting}></Route>
                        <Route path="/:meetCode" element={<Meeting/>} />
                    </Routes>
        </UserProvider>
                </Router>
                    </Box>
                </Box>
    ), root!);