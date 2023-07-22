import {GameManagerInstance} from "./game-manager-instance";

const getConnectedFriends = async () => {
    return (await GameManagerInstance.getInstance.get("friends/connected_friends"));
}

const getMyGames = async () => {
    return (await GameManagerInstance.getInstance.get("games/mine/1"));
}

const updateGame = async (game) => {
    return (await GameManagerInstance.getInstance.put("games/1", game));
}


export const GamesService = {
    findPlayers: getConnectedFriends,
    findMyGames: getMyGames,
    saveUpdatedGame: updateGame
}