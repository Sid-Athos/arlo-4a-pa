import {createResource, createSignal, onMount, Show} from "solid-js";
import AgGridSolid from 'ag-grid-solid';
import 'ag-grid-community/styles/ag-grid.css';
import 'ag-grid-community/styles/ag-theme-alpine.css';
import {Box, Button, ButtonGroup, Card, CardActions, CardContent, Grid, Modal, Stack} from "@suid/material";
import {GamesService} from "../utils/services/game-manager-service";
import {UserStore} from "../utils/user-store";


export default function Lobby(){
    const [gameList,setGameList] = createSignal([]);

    const [open, setOpen] = createSignal(false);
    onMount(async () => {
        console.log(UserStore.get().token)
        let data = await GamesService.findGames().data;
        setGameList(data)
        const socket = new WebSocket('ws://localhost:7589/ws?token=' + UserStore.get().token);
        UserStore.save({socketConnection: socket})
        socket.onopen = function () {
            this.send(JSON.stringify({
                CreateLobby: {
                    game_id: 2,
                    private: false
                }
            }));
        };
        socket.onmessage = function (msg) {
            console.log(JSON.parse(msg.data))

        };

    })




    const [rowData, setRowData] = createSignal([]);
    const columnDefs = [
        {
            field: 'name',
            headerName:'Jeu',
            width: 200
        },
        {
            field: 'min_players',
            headerName :'Participants minimum',
            width: 200
        },
        {
            field: 'max_players',
            headerName: 'Participants maximum',
            width: 200
        },
        {
            field: 'description',
            headerName: 'Description',
            width: 200
        },
        {
            field: 'actions',
            suppressMovable: true,
            width: 200,
            headerName: 'Actions',
            cellRenderer: params => renderManagementOptions("products", params, true, false),
            pinned: 'right',
        }
        //{ field: 'actions'}
    ];
     const initGame = async () => {
         const res = await GamesService.findPlayers();
         console.log(res);

     }

    const initReplay = () => {

    }
    function renderManagementOptions(type, params, withSave, withDelete) {
        return (
            <>
                <Button
                    style={{border: "none"}}
                    onClick={(e) => initGame(type, params)}>Play</Button>
                 <Button onClick={() => initReplay(params)}>Replay</Button>

            </>);
    }
    const handleClose = () => setOpen(false);

    return (
        <>
            <Box sx={{ flexGrow: 1 }}>
                <Grid container spacing={2}>
                    <Grid item xs={6} md={8}>

                        <Show
                            when={!gameList.loading}
                            fallback={null}
                        >
                            {JSON.stringify(gameList())}
                            <div className="ag-theme-alpine-dark" style={{ height: '150px', width: '800px', marginLeft:"300px", top:"100px"}}>
                                <AgGridSolid
                                    columnDefs={columnDefs}
                                    rowData={rowData()}
                                    defaultColDef={columnDefs}
                                />
                            </div>
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
                    <Grid item xs={6} md={4}>
                    </Grid>
                </Grid>
            </Box>
        </>
    )

};