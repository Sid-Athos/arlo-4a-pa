import {GameManagerInstance} from "./game-manager-instance";
import {ApiInstance} from "./api-instance";

const getConnectedFriends = async () => {
    return (await GameManagerInstance.getInstance.get("friends/connected_friends"));
}

const getRankByGames = async () => {
    return (await ApiInstance.getInstance.get("ranking/games"));
}

const getMyGames = async (idUser) => {
    return (await GameManagerInstance.getInstance.get("games/mine/" + idUser));
}

const updateGame = async (id, game) => {
    return (await GameManagerInstance.getInstance.put("games/" + id, game));
}

const createGame = async (game) => {
    return (await GameManagerInstance.getInstance.post("games/", game));
}

const deleteGame = async (id) => {
    return (await GameManagerInstance.getInstance.put("games/" + id));
}


export const GamesService = {
    findPlayers: getConnectedFriends,
    findMyGames: getMyGames,
    updateGame: updateGame,
    newGame: createGame,
    remove: deleteGame,
    allGamesRanking: getRankByGames
}