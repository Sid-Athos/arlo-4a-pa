import {GameManagerInstance} from "./game-manager-instance";

const getConnectedFriends = async () => {
    return (await GameManagerInstance.getInstance.get("friends/connected_friends"));
}

const getMyGames = async () => {
    return (await GameManagerInstance.getInstance.get("games/mine/1"));
}


export const GamesService = {
    findPlayers: getConnectedFriends,
    findMyGames: getMyGames
}