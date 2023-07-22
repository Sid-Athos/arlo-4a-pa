import {GameManagerInstance} from "./game-manager-instance";

const getConnectedFriends = async () => {
    return (await GameManagerInstance.getInstance.get("friends/connected_friends"));
}

const getMyGames = async (idUser) => {
    return (await GameManagerInstance.getInstance.get("games/mine/" + idUser));
}

const updateGame = async (id, game) => {
    return (await GameManagerInstance.getInstance.put("games/" + id, game));
}


export const GamesService = {
    findPlayers: getConnectedFriends,
    findMyGames: getMyGames,
    saveUpdatedGame: updateGame
}