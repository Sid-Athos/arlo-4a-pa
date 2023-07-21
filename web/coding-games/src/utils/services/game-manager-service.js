import {GameManagerInstance} from "./game-manager-instance";

const getConnectedFriends = async () => {
    return (await GameManagerInstance.getInstance.get("friends/connected_friends"));
}


export const GamesService = {
    findPlayers: getConnectedFriends
}