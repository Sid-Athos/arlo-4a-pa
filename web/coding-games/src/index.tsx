/* @refresh reload */
import { render } from 'solid-js/web';
// @ts-ignore
import './index.css';
import {Router, Route, Routes} from "@solidjs/router";
import {AppBar, Box, Button, Divider, IconButton, Stack, Toolbar, Typography} from "@suid/material";
import MenuIcon from "@suid/icons-material/Menu";
import {createSignal} from "solid-js";
// @ts-ignore
import LoginComponent from "./components/login.jsx";
// @ts-ignore
import GameEditorComponent from "./components/game-editor.jsx";
// @ts-ignore
import LobbyComponent from "./components/lobby.jsx";

// @ts-ignore
import {UserProvider} from "./components/user-provider.jsx";
const root = document.getElementById('root');

// @ts-ignore
if (import.meta.env.DEV && !(root instanceof HTMLElement)) {
    throw new Error(
        'Root element not found. Did you forget to add it to your index.html? Or maybe the id attribute got mispelled?',
    );
}
const handleOpen = () => setOpen(true);
const [open, setOpen] = createSignal(false);

render(
    () => (
        <UserProvider token={""}>


        <Box sx={{backgroundColor:'#282c34', minHeight:'100vh'}}>
            <Box sx={{ flexGrow: 1 }}>
                <AppBar position="sticky" sx={{backgroundColor:'#282c34'}}>
                    <Toolbar>
                        <IconButton
                            size="large"
                            edge="start"
                            color="inherit"
                            aria-label="menu"
                            sx={{ mr: 2 }}
                        >
                            <MenuIcon />
                        </IconButton>
                        <Typography variant="h6" component="div" sx={{ flexGrow: 1 }}>
                            <Stack
                                direction="row"
                                divider={<Divider orientation="vertical" flexItem />}
                                spacing={2}
                            >
                            <Typography>
                                Games
                            </Typography>
                            <Typography>
                                Leaderboard
                            </Typography>
                            <Typography>
                                Code Editor
                            </Typography>
                            </Stack>
                        </Typography>
                        <Button color="inherit" onClick={handleOpen}>Login</Button>
                    </Toolbar>
                </AppBar>
            </Box>
        <Router>
            <Routes>
                {/* @ts_ignore */}
                <Route path={"/"} component={<LoginComponent open={open} setOpen={setOpen}></LoginComponent>}></Route>
                <Route path={"/lobby"} component={<LobbyComponent ></LobbyComponent>}></Route>
                <Route path={"/code-editor"} component={<GameEditorComponent></GameEditorComponent>}></Route>
            </Routes>
        </Router>
    </Box>
        </UserProvider>
    ), root!);