import {
    Box,
    createTheme,
    Grid,
    Paper,
    styled,
    Table,
    TableBody,
    TableCell,
    TableContainer,
    TableHead,
    TableRow,
    ThemeProvider
} from "@suid/material";
import {createSignal, onMount} from "solid-js";
import {GamesService} from "../utils/services/game-manager-service";

export default function Rankings() {
    const [rankData, setRankData] = createSignal([]);
    const Item = styled(Paper)(({theme}) => ({
        ...theme.typography.body2,
        backgroundColor: 'transparent',
        padding: theme.spacing(1),
        textAlign: "center",
        maxWidth: "200px",
        marginLeft:"45%",
        color: theme.palette.text.secondary,
    }));
    onMount(async () => {
        let res = await GamesService.allGamesRanking();
        divideRankingsByGame(res.data)
    });

    const divideRankingsByGame = (rankingData) => {
        if (rankingData.length > 0) {
            let finalData = [];
            let json = {
                name: rankingData[0].game,
                rankings: []
            }
            for (let i = 0; i < rankingData.length; i++) {
                let newRanking = {};
                if (rankingData[i].game !== json.name) {
                    finalData.push({...json})
                    json.rankings = []
                    json.name = rankingData[i].game
                    newRanking.rank = rankingData[i].rank
                    newRanking.username = rankingData[i].pseudo
                    newRanking.nb_games = rankingData[i].nb_games
                    json.rankings.push(newRanking)
                } else {
                    newRanking.rank = rankingData[i].rank
                    newRanking.username = rankingData[i].pseudo
                    newRanking.nb_games = rankingData[i].nb_games
                    json.rankings.push(newRanking)
                }
            }
            finalData.push(json)
            setRankData(finalData)
        }
    }
    const darkTheme = createTheme({
        palette: {
            mode: 'dark',
        },
    });

    return (
        <>
            <ThemeProvider theme={darkTheme}>
                <Box sx={{flexGrow: 1, m : 2}}>
                    <Grid container spacing={2}>
                        {
                            rankData().map((game, index) => {
                                return (<>
                                        <Grid item xs={6} md={6}>
                                            <Box sx={{flexGrow: 1, marginTop: "20px"}}>
                                                <Item>{game.name}</Item>
                                                <TableContainer component={Paper}>
                                                    <Table sx={{minWidth: 650}} aria-label="simple table">
                                                        <TableHead>
                                                            <TableRow>
                                                                <TableCell>#</TableCell>
                                                                <TableCell>Username</TableCell>
                                                                <TableCell align="right">Elo</TableCell>
                                                                <TableCell align="right">Game amount</TableCell>
                                                            </TableRow>
                                                        </TableHead>
                                                        <TableBody>
                                                            {game.rankings.map(
                                                                (row, index) => {

                                                                    return (
                                                                        <TableRow
                                                                            sx={{"&:last-child td, &:last-child th": {border: 0}}}
                                                                        >
                                                                            <TableCell component="th" scope="row">
                                                                                {index + 1}
                                                                            </TableCell>
                                                                            <TableCell component="th" scope="row">
                                                                                {row.username}
                                                                            </TableCell>
                                                                            <TableCell
                                                                                align="right">{row.rank}</TableCell>
                                                                            <TableCell
                                                                                align="right">{row.nb_games}</TableCell>
                                                                        </TableRow>
                                                                    )
                                                                }
                                                            )}
                                                        </TableBody>
                                                    </Table>
                                                </TableContainer>
                                            </Box>
                                        </Grid>
                                    </>
                                )
                            })
                        }
                    </Grid>
                </Box>
            </ThemeProvider>
        </>
    )
}