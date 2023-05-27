import {AxiosInstance} from "./axios-instance";

const signIn = async (userSignIn) => {
    return (await AxiosInstance.getInstance.post('/user/login', userSignIn));
}

const signUp = async (userSignUp) => {
    return await AxiosInstance.getInstance.post('/user', userSignUp);
}

const findOtherPlayers = async (identifier) => {
    return await AxiosInstance.getInstance.get(`/user/other_players/${identifier}`)
}

const findGames = async () => {
    return await AxiosInstance.getInstance.get(`/games/search/`)
}

export const UserService = {
    signIn: signIn,
    findPlayers: findOtherPlayers,
    signUp: signUp,
    getGames: findGames
}