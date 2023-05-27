import {createEffect, createResource, createSignal, on, onMount, Show} from "solid-js";
import {getUser} from "./user-provider";
import AgGridSolid from 'ag-grid-solid';
import 'ag-grid-community/styles/ag-grid.css';
import 'ag-grid-community/styles/ag-theme-alpine.css';
import {Box, Grid} from "@suid/material";

export default function Lobby(){
    const fetchGames = async (token) =>{
        console.log(token)
        if(token !== ""){
            let res = await fetch("http://localhost:7590/games", {
                headers: {
                    Authorization: `${token}`,
                    "api-key": 'coding_games',
                },
            });
            setRowData(await res.json())
        }
    }
    const [userToken, { }] = getUser();
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
        //{ field: 'actions'}
    ];
    console.log("ouai")

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
                <div className="ag-theme-alpine" style={{ height: '150px', width: '800px', marginLeft:"300px", top:"100px"}}>
                    <AgGridSolid
                        columnDefs={columnDefs}
                        rowData={rowData()}
                        defaultColDef={columnDefs}
                    />
                </div>
            </Show>
                    </Grid>
                    <Grid item xs={6} md={4}>
                    </Grid>
                </Grid>
            </Box>
        </>
    )

};