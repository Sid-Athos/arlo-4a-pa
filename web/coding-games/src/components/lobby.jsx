import {createEffect, createResource, createSignal, Match, on, onMount, Show, Switch} from "solid-js";
import {useUserProvider} from "./user-provider";
import AgGridSolid from 'ag-grid-solid';
import 'ag-grid-community/styles/ag-grid.css';
import 'ag-grid-community/styles/ag-theme-alpine.css';
import {
    Alert,
    Box,
    Button, ButtonGroup,
    Card,
    CardActions,
    CardContent,
    Fade,
    Grid, Modal,
    Stack,
    TextField,
    Typography
} from "@suid/material";
import {GamesService} from "../utils/services/game-manager-service";
import {UserStore} from "../utils/user-store";


export default function Lobby(){
    const handleOpen = () => setOpen(true);
    const [open, setOpen] = createSignal(false);
    onMount(() => {
        console.log(UserStore.get().token)
        const socket = new WebSocket('ws://localhost:7589/ws?token='+ UserStore.get().token);
        socket.onopen = function() {
            this.send(JSON.stringify("hello"));
        };
        socket.onmessage = function(msg) {
            if (msg.data && msg.data.slice(0, 3) === 'pub') {
                console.log(message.data)
                // resource updated, refetch resource
            }
        };
    })
    const fetchGames = async (token) =>{
        if(token !== ""){
            let res = await fetch("http://localhost:7589/games/all", {
                headers: {
                    Authorization: `${token}`,
                    "api-key": 'coding_games',
                },
            });
            setRowData(await res.json())
        }
    };



    const [userToken, {updateToken }] = useUserProvider();
    const [gameList] = createResource(userToken, fetchGames);
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