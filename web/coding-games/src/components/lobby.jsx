import {createResource, createSignal, onMount, Show} from "solid-js";
import AgGridSolid from 'ag-grid-solid';
import 'ag-grid-community/styles/ag-grid.css';
import 'ag-grid-community/styles/ag-theme-alpine.css';
import {
    Box,
    Button,
    ButtonGroup,
    Card,
    CardActions,
    CardContent, createTheme,
    Grid,
    Modal,
    Paper,
    Stack,
    Table, TableBody, TableCell, TableContainer, TableHead, TableRow, ThemeProvider
} from "@suid/material";
import {GamesService} from "../utils/services/game-manager-service";
import {UserStore} from "../utils/user-store";
import useSockets from "../hooks/useSockets";

export default function Lobby(){
    const [gameList,setGameList] = createSignal([]);

    const [open, setOpen] = createSignal(false);
    const [privateLobby, setPrivateLobby] = createSignal(false);

    onMount(async () => {
        let data = (await GamesService.findGames()).data;
        setGameList(data)
        useSockets().setUpSockets()
    })

     const initGame = async (gameId) => {
         const res = await GamesService.findPlayers();
         console.log(res)
         const payload = {
             game_id: gameId,
             private: privateLobby()
         }
         //useSockets().sendMessage(payload)
    }

    const initReplay = () => {

    }
    function renderManagementOptions(gameId) {
        return (
            <>
                <Button
                    style={{border: "none"}}
                    onClick={() => initGame(gameId)}>Play</Button>
                 <Button onClick={() => initReplay()}>Replay</Button>

            </>);
    }
    const handleClose = () => setOpen(false);
    const darkTheme = createTheme({
        palette: {
            mode: 'dark',
        },
    });
    return (
        <>
            <Box sx={{ flexGrow: 1 }}>
                <Grid container spacing={2}>

                        <Show
                            when={!gameList.loading}
                            fallback={null}
                        >
                            <ThemeProvider theme={darkTheme}>
                            <TableContainer component={Paper}>
                                <Table sx={{minWidth: 550, maxWidth:1100,ml:30,mt:10}} aria-label="simple table">
                                    <TableHead>
                                        <TableRow>
                                            <TableCell align={"left"}>#</TableCell>
                                            <TableCell>Game</TableCell>
                                            <TableCell align="right">Min players</TableCell>
                                            <TableCell align="right">Max players</TableCell>
                                            <TableCell align="right">Description</TableCell>
                                            <TableCell align="right">Actions</TableCell>
                                        </TableRow>
                                    </TableHead>
                                    <TableBody>
                                        {gameList().map(
                                            (row, index) => {
                                                return (
                                                    <TableRow
                                                        sx={{"&:last-child td, &:last-child th": {border: 0}}}
                                                    >
                                                        <TableCell component="th" scope="row" align={"left"}>
                                                            {index + 1}
                                                        </TableCell>
                                                        <TableCell component="th" scope="row">
                                                            {row.name}
                                                        </TableCell>
                                                        <TableCell component="th" scope="row" align="right">
                                                            {row.min_players}
                                                        </TableCell>
                                                        <TableCell component="th" scope="row" align="right">
                                                            {row.max_players}
                                                        </TableCell>
                                                        <TableCell
                                                            align="right">{row.description}</TableCell>
                                                        <TableCell
                                                            align="right">{() => renderManagementOptions(row.id)}</TableCell>
                                                    </TableRow>
                                                )
                                            }
                                        )}
                                    </TableBody>
                                </Table>
                            </TableContainer>
                            </ThemeProvider>
                        </Show>
                        <Modal
                            open={open()}
                            onClose={handleClose}
                            aria-labelledby="modal-modal-title"
                            aria-describedby="modal-modal-description"
                        >
                            <Card
                                className="signCard"
                            >
                                <CardContent>
                                    <Stack spacing={2}>
                                        Select a user
                                    </Stack>
                                </CardContent>
                                <CardActions >
                                    <Box sx={{paddingLeft: '20px'}}>
                                        <ButtonGroup >
                                            DUEL !
                                        </ButtonGroup>
                                    </Box>
                                </CardActions>
                            </Card>
                        </Modal>
                    </Grid>

            </Box>
        </>
    )

};