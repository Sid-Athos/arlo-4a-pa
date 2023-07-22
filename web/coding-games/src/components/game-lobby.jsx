import {createSignal, onMount, Show} from "solid-js";
import {GamesService} from "../utils/services/game-manager-service";
import {
    Box, Button, createTheme,
    Grid,
    List,
    ListItem, ListItemText,
    Paper,
    Table,
    TableBody,
    TableCell,
    TableContainer,
    TableHead,
    TableRow, ThemeProvider
} from "@suid/material";

export default function GameLobby() {
    const [gameLobbies, setGameLobbies] = createSignal([])

    onMount(async () => {
        let res = await GamesService.getAvailableLobbies()
        setGameLobbies(res.data)
        console.log(res.data)
    })
    const darkTheme = createTheme({
        palette: {
            mode: 'dark',
        },
    });

    const joinLobby= async (lobbyId) => {
        if (gameLobbies().find(lobby => lobby.id === lobbyId).game.max_players > gameLobbies().find(lobby => lobby.id === lobbyId).members.length) {
            await GamesService.joinLobby({lobby_id: lobbyId})
        } else {
            console.log("nope")
        }
    }
    return (
        <>
            <ThemeProvider theme={darkTheme}>
            <Show when={gameLobbies().length > 0}>
                <Grid item xs={6} md={6}>
                    <Box sx={{flexGrow: 1, marginTop: "20px"}}>
                        <TableContainer component={Paper}>
                            <Table sx={{minWidth: 650}} aria-label="simple table">
                                <TableHead>
                                    <TableRow>
                                        <TableCell>#</TableCell>
                                        <TableCell>Game</TableCell>
                                        <TableCell align="left">Members</TableCell>
                                        <TableCell align="right">Description</TableCell>
                                        <TableCell align="right">Minimum players</TableCell>
                                        <TableCell align="right">Maximum players</TableCell>
                                        <TableCell align="right"></TableCell>
                                    </TableRow>
                                </TableHead>
                                <TableBody>
                                    {gameLobbies().map(
                                        (row, index) => {
                                            console.log(row)
                                            return (
                                                <TableRow
                                                    sx={{"&:last-child td, &:last-child th": {border: 0}}}
                                                >
                                                    <TableCell component="th" scope="row">
                                                        {index + 1}
                                                    </TableCell>
                                                    <TableCell component="th" scope="row">
                                                        {row.game.name}
                                                    </TableCell>
                                                    <TableCell
                                                        align="right">
                                                        <List>
                                                            {row.members.map(user => {
                                                                if(user.is_host) {
                                                                    return (
                                                                        <>
                                                                            <ListItem disablePadding>
                                                                                <ListItemText primary={user.pseudo} sx={{color:"green"}}/>
                                                                            </ListItem>
                                                                        </>
                                                                    )
                                                                } else {

                                                                    return (
                                                                        <>
                                                                            <ListItem disablePadding>
                                                                                <ListItemText primary={user.pseudo}/>
                                                                            </ListItem>
                                                                        </>
                                                                    )
                                                                }
                                                            })}
                                                        </List>
                                                    </TableCell>
                                                    <TableCell component="th" scope="row" align="right">
                                                        {row.game.description}
                                                    </TableCell>
                                                    <TableCell component="th" scope="row" align="right">
                                                        {row.game.min_players}
                                                    </TableCell>
                                                    <TableCell component="th" scope="row" align="right">
                                                        {row.game.max_players}
                                                    </TableCell>
                                                    <TableCell
                                                        align="right"><Button onClick={() => joinLobby(row.id)}>Join Lobby</Button></TableCell>
                                                </TableRow>
                                            )
                                        }
                                    )}
                                </TableBody>
                            </Table>
                        </TableContainer>
                    </Box>
                </Grid>
            </Show>
            </ThemeProvider>
        </>
    )
}