import {createEffect, createMemo, createSignal, onMount, Show} from "solid-js";
import 'ag-grid-community/styles/ag-grid.css';
import 'ag-grid-community/styles/ag-theme-alpine.css';
import {
    Box,
    Button,
    ButtonGroup,
    Card,
    CardActions,
    CardContent,
    Chip,
    CircularProgress,
    createTheme,
    Dialog,
    DialogActions,
    DialogContent,
    DialogTitle,
    FormControl,
    Grid,
    InputLabel,
    MenuItem,
    Modal,
    OutlinedInput,
    Paper,
    Select,
    Stack, styled,
    Table,
    TableBody,
    TableCell,
    TableContainer,
    TableHead,
    TableRow,
    ThemeProvider, Typography,
    useTheme
} from "@suid/material";
import {GamesService} from "../utils/services/game-manager-service";
import useSockets from "../hooks/useSockets";
import Chat from "./chat.jsx";
import Game from "./game.jsx";

const ITEM_HEIGHT = 48;
const ITEM_PADDING_TOP = 8;
const MenuProps = {
    PaperProps: {
        style: {
            "max-height": `${ITEM_HEIGHT * 4.5 + ITEM_PADDING_TOP}px`,
            width: `${250}px`,
        },
    },
};
export default function Lobby(){
    const [gameList,setGameList] = createSignal([]);
    const [open, setOpen] = createSignal(false);
    const [privateLobby, setPrivateLobby] = createSignal(false);
    const [selectedGame, setSelectedGame] = createSignal(null);
    const [onlineFriendsList, setOnlineFriendsList] = createSignal([]);
    const {socket, sendMessage, handleReceiveMessage, msg, messages, convo, setConvo} = useSockets();
    const [openDialog, setOpenDialog] = createSignal(false);
    const [openInviteFriendDialog, setOpenInviteFriendDialog] = createSignal(false);
    const [isGameSettingUp, setIsGameSettingUp] = createSignal(true);
    const [isGameReady, setIsGameReady] = createSignal(false);
    const [invitesToLobby, setInvitesToLobby] = createSignal([]);
    const theme = useTheme();
    onMount(async () => {
        let data = (await GamesService.findGames()).data;
        setGameList(data)
    })
    function test(val){
    }

    const msgList = createMemo((previous) => {

        return test(messages);
    });

    const conversation = createMemo((previous) => {

        return convo();
    });

     const initGame = async (gameId) => {
        setOpenDialog(true)
        setSelectedGame(gameId)
    }

    const initReplay = () => {

    }
    const handleClose = () => setOpen(false);
    const darkTheme = createTheme({
        palette: {
            mode: 'dark',
        },
    });


    const handleCloseDialog = () => {
        setOpenDialog(false);
    };




    const init = async (boolValue) => {
        setOpenDialog(false);
        if (boolValue) {
            const res = await GamesService.findPlayers();
            setOnlineFriendsList(res.data)
            if(selectedGame()){
                const payload = {CreateLobby: {
                    game_id: selectedGame(),
                    private: privateLobby()
                }}
                sendMessage(payload)
                if(res.data.length > 0){
                    setOpenInviteFriendDialog(true)
                }
                setIsGameSettingUp(false)
            }
        }
    };


    function renderManagementOptions(gameId) {
        return (
            <>
                <Button
                    style={{border: "none"}}
                    onClick={() => initGame(gameId)}>Play</Button>
                 <Button onClick={() => initReplay()}>Replay</Button>

            </>);
    }

    function getStyles(nameg, personName, theme) {
        return {
            "font-weight":
                personName.indexOf(name) === -1
                    ? theme.typography.fontWeightRegular
                    : theme.typography.fontWeightMedium,
        };
    }

    const handleInvitesChange = (e) => {
    }

    const Item = styled(Paper)(({ theme }) => {
        return ({
            ...theme.typography.body2,
            padding: theme.spacing(1),
            textAlign: "center",
            color: theme.palette.text.primary,
            backgroundColor:"unset"
        })
    });
    return (
        <>


            <Show when={isGameSettingUp()}>
                <ThemeProvider theme={darkTheme}>
                    <Dialog
                        open={openDialog()}
                        onClose={handleCloseDialog}
                        aria-describedby="alert-dialog-slide-description"
                    >
                        <DialogTitle>{"Lobby settings"}</DialogTitle>
                        <DialogContent>
                            <FormControl fullWidth>
                                <InputLabel id="demo-simple-select-label">Lobby privacy</InputLabel>
                                <Select
                                    labelId="demo-simple-select-label"
                                    id="demo-simple-select"
                                    value={privateLobby()}
                                    label="Age"
                                    onChange={() => setPrivateLobby(!privateLobby())}
                                >
                                    <MenuItem value={false}>Private</MenuItem>
                                    <MenuItem value={true}>Public</MenuItem>
                                </Select>
                            </FormControl>
                        </DialogContent>
                        <DialogActions>
                            <Button onClick={() => handleCloseDialog}>Cancel</Button>
                            <Button onClick={() => init(true)}>Create lobby</Button>
                        </DialogActions>
                    </Dialog>
                    <Dialog
                        open={openInviteFriendDialog()}
                        onClose={handleCloseDialog}
                        aria-describedby="alert-dialog-slide-description"
                    >
                        <DialogTitle>{"Lobby settings"}</DialogTitle>
                        <DialogContent>
                            <FormControl fullWidth>
                                <InputLabel id="demo-simple-select-label">Invite users to Lobby</InputLabel>
                                <FormControl
                                    sx={{
                                        m: 1,
                                        width: 300,
                                    }}
                                >
                                    <InputLabel id="demo-multiple-chip-label">Chip</InputLabel>
                                    <Select
                                        labelId="demo-multiple-chip-label"
                                        id="demo-multiple-chip"
                                        multiple
                                        value={invitesToLobby()}
                                        onChange={handleInvitesChange}
                                        input={<OutlinedInput id="select-multiple-chip" label="Chip"/>}
                                        renderValue={(selected) => (
                                            <Box
                                                sx={{
                                                    display: "flex",
                                                    flexWrap: "wrap",
                                                    gap: 0.5,
                                                }}
                                            >
                                                {selected.map((value) => (
                                                    <Chip label={value}/>
                                                ))}
                                            </Box>
                                        )}
                                        MenuProps={MenuProps}
                                    >
                                        {onlineFriendsList().map((name) => (
                                            <MenuItem value={name.id} style={getStyles(name, invitesToLobby(), theme)}>
                                                {name.pseudo}
                                            </MenuItem>
                                        ))}
                                    </Select>
                                </FormControl>
                            </FormControl>
                        </DialogContent>
                        <DialogActions>
                            <Button onClick={() => handleCloseDialog}>Cancel</Button>
                            <Button onClick={() => init(true)}>Create lobby</Button>
                        </DialogActions>
                    </Dialog>
                </ThemeProvider>
                <Box sx={{flexGrow: 1}}>
                    <Grid container spacing={2}>

                        <Show
                            when={!gameList.loading}
                            fallback={null}
                        >
                            <ThemeProvider theme={darkTheme}>
                                <TableContainer component={Paper}>
                                    <Table sx={{minWidth: 550, maxWidth: 1100, ml: 30, mt: 10}}
                                           aria-label="simple table">
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
                                <CardActions>
                                    <Box sx={{paddingLeft: '20px'}}>
                                        <ButtonGroup>
                                            DUEL !
                                        </ButtonGroup>
                                    </Box>
                                </CardActions>
                            </Card>
                        </Modal>
                    </Grid>
                </Box>
            </Show>
            <Show when={!isGameSettingUp() && !isGameReady()}>
                <Grid container>
                    <Box sx={{ height: "400px",mt:10,ml:20, width:"60%",textAlign:"center",color:"white" }} >
                        <Stack spacing={2}>
                            <Item>
                            <Box
                                component="img"
                                sx={{
                                    height: 233,
                                    width: 350,
                                    maxHeight: { xs: 233, md: 167 },
                                    maxWidth: { xs: 350, md: 250 },
                                }}
                                alt="The house from the offer."
                                src="https://i.gifer.com/qH.gif"
                            />

                            </Item>
                            <Item>

                        <Box>
                        Waiting for minimum amount of players
                            <CircularProgress color="primary" />
                        </Box>
                            </Item>
                        </Stack>
                        <Game></Game>
                    </Box>
                </Grid>
                <Chat socket={socket} sendMessage={sendMessage} conversation={conversation} setConvo={setConvo}></Chat>
            </Show>
        </>
    )

};