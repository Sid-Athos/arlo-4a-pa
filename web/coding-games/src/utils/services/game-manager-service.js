import {GameManagerInstance} from "./game-manager-instance";
import {ApiInstance} from "./api-instance";

const getConnectedFriends = async () => {
    return (await GameManagerInstance.getInstance.get("friends/connected_friends"));
}

const getRankByGames = async () => {
    return (await ApiInstance.getInstance.get("ranking/games"));
}

const getPublicLobbies = async () => {
    return (await GameManagerInstance.getInstance.get("lobby/get_public"));
}

const getMyGames = async () => {
    return (await GameManagerInstance.getInstance.get("games/mine"));
}

const getAllGames = async () => {
    return (await GameManagerInstance.getInstance.get("games/all"));
}

const updateGame = async (id, game) => {
    return (await GameManagerInstance.getInstance.put("games/" + id, game));
}

const createGame = async (game) => {
    return (await GameManagerInstance.getInstance.post("games", game));
}

const joinRtc = async () => {
    return (await GameManagerInstance.getInstance.post("rtc/join_rtc"));
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
    allGamesRanking: getRankByGames,
    getAvailableLobbies:getPublicLobbies,
    rtcMeeting: joinRtc,
    findGames:getAllGames
}